// Hyper Terminal Gallery - Beautiful media display for the LLM era
//
// This crate provides a world-class gallery system for displaying media content
// in the terminal. It's designed specifically for LLM interactions where AIs
// generate images, files, charts, and other rich content that deserves beautiful
// presentation.

pub mod types;
pub mod layout;
pub mod card;
pub mod interactions;
pub mod animations;
pub mod renderer;
pub mod theme;

pub use types::*;
pub use layout::*;
pub use card::*;
pub use interactions::*;
pub use animations::*;
pub use renderer::*;
pub use theme::*;

use anyhow::Result;
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};

/// Main gallery manager - orchestrates all gallery operations
pub struct GalleryManager {
    /// Active galleries
    galleries: HashMap<String, Gallery>,

    /// All media cards across all galleries
    cards: HashMap<u32, MediaCard>,

    /// Next available card ID
    next_card_id: u32,

    /// Interaction state
    interaction_state: InteractionState,

    /// Animation system
    animation_system: AnimationSystem,

    /// Theme
    theme: GalleryTheme,
}

impl GalleryManager {
    pub fn new() -> Self {
        Self {
            galleries: HashMap::new(),
            cards: HashMap::new(),
            next_card_id: 0,
            interaction_state: InteractionState::default(),
            animation_system: AnimationSystem::new(),
            theme: GalleryTheme::default(),
        }
    }

    /// Start a new gallery
    pub fn create_gallery(&mut self, id: String, config: GalleryConfig) -> Result<()> {
        let gallery = Gallery::new(id.clone(), config);
        self.galleries.insert(id, gallery);
        Ok(())
    }

    /// Add a media item to a gallery
    pub fn add_media_to_gallery(
        &mut self,
        gallery_id: &str,
        content: MediaContent,
        metadata: CardMetadata,
    ) -> Result<u32> {
        let card_id = self.next_card_id;
        self.next_card_id += 1;

        let card = MediaCard::new(card_id, content, metadata);
        self.cards.insert(card_id, card);

        if let Some(gallery) = self.galleries.get_mut(gallery_id) {
            gallery.add_card(card_id);
        }

        Ok(card_id)
    }

    /// End a gallery and trigger layout calculation
    pub fn finalize_gallery(&mut self, gallery_id: &str, container: Rect) -> Result<()> {
        if let Some(gallery) = self.galleries.get_mut(gallery_id) {
            // Get cards for this gallery
            let gallery_cards: Vec<&MediaCard> = gallery
                .card_ids
                .iter()
                .filter_map(|id| self.cards.get(id))
                .collect();

            // Calculate layout
            let placements = layout_gallery(&gallery_cards, container, gallery.config.mode);

            // Update card positions
            for (card_id, placement) in gallery.card_ids.iter().zip(placements.iter()) {
                if let Some(card) = self.cards.get_mut(card_id) {
                    card.layout.bounds = placement.bounds;
                }
            }

            // Trigger entrance animations
            self.animation_system.animate_gallery_entrance(&gallery_cards);
        }

        Ok(())
    }

    /// Get all visible galleries in viewport
    pub fn get_visible_galleries(&self, viewport: Rect) -> Vec<&Gallery> {
        self.galleries
            .values()
            .filter(|g| viewport.intersects(&g.bounds))
            .collect()
    }

    /// Get all cards in a gallery
    pub fn get_gallery_cards(&self, gallery_id: &str) -> Vec<&MediaCard> {
        if let Some(gallery) = self.galleries.get(gallery_id) {
            gallery
                .card_ids
                .iter()
                .filter_map(|id| self.cards.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Update hover state
    pub fn update_hover(&mut self, cursor_pos: Option<(f32, f32)>) {
        self.interaction_state.update_hover(cursor_pos, &self.cards);
    }

    /// Handle click event
    pub fn handle_click(&mut self, pos: (f32, f32)) -> Option<CardAction> {
        self.interaction_state.handle_click(pos, &self.cards)
    }

    /// Update animations
    pub fn update_animations(&mut self, delta_time: Duration) {
        self.animation_system.update(delta_time, &mut self.cards);
    }

    /// Get theme
    pub fn theme(&self) -> &GalleryTheme {
        &self.theme
    }

    /// Set theme
    pub fn set_theme(&mut self, theme: GalleryTheme) {
        self.theme = theme;
    }
}

impl Default for GalleryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// A gallery contains multiple media cards
#[derive(Debug, Clone)]
pub struct Gallery {
    pub id: String,
    pub config: GalleryConfig,
    pub card_ids: Vec<u32>,
    pub created: SystemTime,
    pub bounds: Rect,
}

impl Gallery {
    pub fn new(id: String, config: GalleryConfig) -> Self {
        Self {
            id,
            config,
            card_ids: Vec::new(),
            created: SystemTime::now(),
            bounds: Rect::default(),
        }
    }

    pub fn add_card(&mut self, card_id: u32) {
        self.card_ids.push(card_id);
    }
}

/// Gallery configuration
#[derive(Debug, Clone)]
pub struct GalleryConfig {
    pub mode: GalleryMode,
    pub title: Option<String>,
    pub columns: Option<u32>,
    pub spacing: f32,
    pub padding: f32,
}

impl Default for GalleryConfig {
    fn default() -> Self {
        Self {
            mode: GalleryMode::Auto,
            title: None,
            columns: None,
            spacing: 12.0,
            padding: 16.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_gallery() {
        let mut manager = GalleryManager::new();
        let config = GalleryConfig::default();

        manager.create_gallery("test".to_string(), config).unwrap();
        assert!(manager.galleries.contains_key("test"));
    }

    #[test]
    fn test_add_media_to_gallery() {
        let mut manager = GalleryManager::new();
        let config = GalleryConfig::default();

        manager.create_gallery("test".to_string(), config).unwrap();

        let content = MediaContent::Text("Hello".to_string());
        let metadata = CardMetadata::default();

        let card_id = manager
            .add_media_to_gallery("test", content, metadata)
            .unwrap();

        assert_eq!(card_id, 0);
        assert!(manager.cards.contains_key(&card_id));
    }
}
