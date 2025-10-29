// Gallery layout engine - intelligent positioning of media cards

use crate::types::*;
use crate::card::MediaCard;

/// Result of laying out a card
#[derive(Debug, Clone)]
pub struct CardPlacement {
    pub card_id: u32,
    pub bounds: Rect,
}

/// Layout all cards in a gallery
pub fn layout_gallery(
    cards: &[&MediaCard],
    container: Rect,
    mode: GalleryMode,
) -> Vec<CardPlacement> {
    match mode {
        GalleryMode::Auto => auto_layout(cards, container),
        GalleryMode::Grid => grid_layout(cards, container),
        GalleryMode::Masonry => masonry_layout(cards, container),
        GalleryMode::Filmstrip => filmstrip_layout(cards, container),
        GalleryMode::Comparison => comparison_layout(cards, container),
        GalleryMode::Deck => deck_layout(cards, container),
        GalleryMode::FileExplorer => file_explorer_layout(cards, container),
    }
}

/// Automatically choose best layout based on content
fn auto_layout(cards: &[&MediaCard], container: Rect) -> Vec<CardPlacement> {
    if cards.is_empty() {
        return Vec::new();
    }

    // Detect best mode
    let mode = if cards.len() == 2 {
        // Two items - comparison view
        GalleryMode::Comparison
    } else if cards.len() <= 4 && all_similar_aspect_ratios(cards) {
        // Few items with similar aspects - grid
        GalleryMode::Grid
    } else if is_sequential(cards) {
        // Sequential content - filmstrip
        GalleryMode::Filmstrip
    } else {
        // Default to masonry for flexible layout
        GalleryMode::Masonry
    };

    layout_gallery(cards, container, mode)
}

/// Grid layout - uniform card sizes in rows and columns
pub fn grid_layout(cards: &[&MediaCard], container: Rect) -> Vec<CardPlacement> {
    if cards.is_empty() {
        return Vec::new();
    }

    let spacing = 12.0;
    let padding = 16.0;

    // Calculate optimal columns
    let columns = calculate_grid_columns(container.width as u32, cards.len());

    // Calculate card dimensions
    let available_width = container.width - (padding * 2.0) - (spacing * (columns - 1) as f32);
    let card_width = available_width / columns as f32;

    // Calculate rows needed
    let rows = (cards.len() as f32 / columns as f32).ceil() as usize;

    let mut placements = Vec::with_capacity(cards.len());

    for (i, card) in cards.iter().enumerate() {
        let col = i % columns;
        let row = i / columns;

        let x = container.x + padding + (col as f32 * (card_width + spacing));
        let y = container.y + padding + (row as f32 * (card_width + spacing));

        // Maintain aspect ratio
        let height = card_width / card.layout.aspect_ratio;

        placements.push(CardPlacement {
            card_id: card.id,
            bounds: Rect::new(x, y, card_width, height),
        });
    }

    placements
}

/// Masonry layout - Pinterest-style with variable heights
pub fn masonry_layout(cards: &[&MediaCard], container: Rect) -> Vec<CardPlacement> {
    if cards.is_empty() {
        return Vec::new();
    }

    let spacing = 12.0;
    let padding = 16.0;

    // Calculate columns
    let columns = calculate_grid_columns(container.width as u32, cards.len());

    // Calculate card width
    let available_width = container.width - (padding * 2.0) - (spacing * (columns - 1) as f32);
    let card_width = available_width / columns as f32;

    // Track height of each column
    let mut column_heights = vec![0.0; columns];

    let mut placements = Vec::with_capacity(cards.len());

    for card in cards {
        // Find shortest column
        let (shortest_col, &shortest_height) = column_heights
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        // Calculate position
        let x = container.x + padding + (shortest_col as f32 * (card_width + spacing));
        let y = container.y + padding + shortest_height;

        // Calculate height maintaining aspect ratio
        let height = card_width / card.layout.aspect_ratio;

        placements.push(CardPlacement {
            card_id: card.id,
            bounds: Rect::new(x, y, card_width, height),
        });

        // Update column height
        column_heights[shortest_col] += height + spacing;
    }

    placements
}

/// Filmstrip layout - horizontal scrolling strip
pub fn filmstrip_layout(cards: &[&MediaCard], container: Rect) -> Vec<CardPlacement> {
    if cards.is_empty() {
        return Vec::new();
    }

    let spacing = 8.0;
    let padding = 16.0;

    // Calculate card height (fill container minus padding)
    let card_height = container.height - (padding * 2.0) - 40.0; // Space for caption

    let mut x_offset = container.x + padding;

    let mut placements = Vec::with_capacity(cards.len());

    for card in cards {
        // Calculate width maintaining aspect ratio
        let card_width = card_height * card.layout.aspect_ratio;

        placements.push(CardPlacement {
            card_id: card.id,
            bounds: Rect::new(x_offset, container.y + padding, card_width, card_height),
        });

        x_offset += card_width + spacing;
    }

    placements
}

/// Comparison layout - side-by-side view
pub fn comparison_layout(cards: &[&MediaCard], container: Rect) -> Vec<CardPlacement> {
    if cards.is_empty() {
        return Vec::new();
    }

    let spacing = 16.0;
    let padding = 16.0;

    let num_cards = cards.len().min(4); // Max 4 in comparison
    let card_width = (container.width - (padding * 2.0) - (spacing * (num_cards - 1) as f32)) / num_cards as f32;

    let mut placements = Vec::with_capacity(num_cards);

    for (i, card) in cards.iter().take(num_cards).enumerate() {
        let x = container.x + padding + (i as f32 * (card_width + spacing));

        // Maintain aspect ratio
        let height = card_width / card.layout.aspect_ratio;
        let y = container.y + padding;

        placements.push(CardPlacement {
            card_id: card.id,
            bounds: Rect::new(x, y, card_width, height),
        });
    }

    placements
}

/// Deck layout - stacked cards with focus on top card
pub fn deck_layout(cards: &[&MediaCard], container: Rect) -> Vec<CardPlacement> {
    if cards.is_empty() {
        return Vec::new();
    }

    let padding = 32.0;
    let card_width = container.width - (padding * 2.0);
    let card_height = container.height - (padding * 2.0) - 60.0; // Space for navigation

    let mut placements = Vec::with_capacity(cards.len());

    // For now, just position all cards at same location
    // (actual rendering will show only focused card)
    for card in cards {
        placements.push(CardPlacement {
            card_id: card.id,
            bounds: Rect::new(
                container.x + padding,
                container.y + padding,
                card_width,
                card_height,
            ),
        });
    }

    placements
}

/// File explorer layout - tree structure with previews
pub fn file_explorer_layout(cards: &[&MediaCard], container: Rect) -> Vec<CardPlacement> {
    if cards.is_empty() {
        return Vec::new();
    }

    let padding = 16.0;
    let tree_width = 300.0;
    let preview_x = container.x + tree_width + padding;

    let mut placements = Vec::with_capacity(cards.len());
    let mut y_offset = container.y + padding;
    let line_height = 32.0;

    // List layout for file tree
    for card in cards {
        placements.push(CardPlacement {
            card_id: card.id,
            bounds: Rect::new(container.x + padding, y_offset, tree_width, line_height),
        });

        y_offset += line_height + 4.0;
    }

    placements
}

/// Calculate optimal number of columns for grid
pub fn calculate_grid_columns(container_width: u32, num_items: usize) -> usize {
    let cols = match container_width {
        0..=600 => 1,
        601..=900 => 2,
        901..=1200 => 3,
        1201..=1600 => 4,
        _ => 5,
    };

    // Don't use more columns than items
    cols.min(num_items)
}

/// Check if all cards have similar aspect ratios
fn all_similar_aspect_ratios(cards: &[&MediaCard]) -> bool {
    if cards.len() < 2 {
        return true;
    }

    let first_ratio = cards[0].layout.aspect_ratio;
    let threshold = 0.2;

    cards.iter().all(|card| {
        (card.layout.aspect_ratio - first_ratio).abs() < threshold
    })
}

/// Check if cards appear to be sequential (numbered titles, timestamps, etc.)
fn is_sequential(cards: &[&MediaCard]) -> bool {
    if cards.len() < 3 {
        return false;
    }

    // Check for numbered titles
    let numbered_count = cards
        .iter()
        .filter(|card| {
            card.metadata
                .title
                .as_ref()
                .map(|t| t.chars().next().map(|c| c.is_numeric()).unwrap_or(false))
                .unwrap_or(false)
        })
        .count();

    numbered_count >= cards.len() / 2
}

/// Calculate total gallery height for given layout
pub fn calculate_gallery_height(cards: &[&MediaCard], container_width: f32, mode: GalleryMode) -> f32 {
    if cards.is_empty() {
        return 0.0;
    }

    let container = Rect::new(0.0, 0.0, container_width, 10000.0); // Tall container
    let placements = layout_gallery(cards, container, mode);

    // Find max Y + height
    placements
        .iter()
        .map(|p| p.bounds.y + p.bounds.height)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0)
        + 16.0 // Bottom padding
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_card(id: u32, aspect_ratio: f32) -> MediaCard {
        let mut card = MediaCard::new(
            id,
            MediaContent::Text("test".to_string()),
            CardMetadata::default(),
        );
        card.layout.aspect_ratio = aspect_ratio;
        card
    }

    #[test]
    fn test_grid_layout() {
        let cards_data: Vec<MediaCard> = (0..4).map(|i| create_test_card(i, 1.0)).collect();
        let cards: Vec<&MediaCard> = cards_data.iter().collect();
        let container = Rect::new(0.0, 0.0, 800.0, 600.0);

        let placements = grid_layout(&cards, container);

        assert_eq!(placements.len(), 4);
        // Check that cards are in grid positions
        assert!(placements[0].bounds.x < placements[1].bounds.x);
    }

    #[test]
    fn test_calculate_columns() {
        assert_eq!(calculate_grid_columns(500, 10), 1);
        assert_eq!(calculate_grid_columns(800, 10), 2);
        assert_eq!(calculate_grid_columns(1000, 10), 3);
        assert_eq!(calculate_grid_columns(1000, 2), 2); // Max 2 items
    }

    #[test]
    fn test_masonry_layout() {
        let mut cards_data: Vec<MediaCard> = Vec::new();
        cards_data.push(create_test_card(0, 1.0)); // Square
        cards_data.push(create_test_card(1, 2.0)); // Wide
        cards_data.push(create_test_card(2, 0.5)); // Tall

        let cards: Vec<&MediaCard> = cards_data.iter().collect();
        let container = Rect::new(0.0, 0.0, 800.0, 1000.0);

        let placements = masonry_layout(&cards, container);

        assert_eq!(placements.len(), 3);
        // Masonry should place cards in different positions
        assert_ne!(placements[0].bounds.y, placements[1].bounds.y);
    }
}
