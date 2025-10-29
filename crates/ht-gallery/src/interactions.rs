// Interaction handling - mouse, keyboard, and touch events

use crate::types::*;
use crate::card::MediaCard;
use std::collections::HashMap;
use std::time::Instant;

/// Interaction state manager
#[derive(Debug, Default)]
pub struct InteractionState {
    /// Currently hovered card
    pub hovered_card: Option<u32>,

    /// Currently selected cards
    pub selected_cards: Vec<u32>,

    /// Card in fullscreen mode
    pub fullscreen_card: Option<u32>,

    /// Drag state
    pub drag_state: Option<DragState>,

    /// Last click time (for double-click detection)
    pub last_click_time: Option<Instant>,

    /// Last click position
    pub last_click_pos: Option<(f32, f32)>,

    /// Keyboard focus
    pub focused_card: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct DragState {
    pub card_id: u32,
    pub start_pos: (f32, f32),
    pub current_pos: (f32, f32),
    pub started_at: Instant,
}

impl InteractionState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update hover state based on cursor position
    pub fn update_hover(&mut self, cursor_pos: Option<(f32, f32)>, cards: &HashMap<u32, MediaCard>) {
        let new_hovered = if let Some(pos) = cursor_pos {
            self.card_at_point(pos, cards)
        } else {
            None
        };

        // Update old hovered card
        if let Some(old_id) = self.hovered_card {
            if Some(old_id) != new_hovered {
                if let Some(card) = cards.get(&old_id) {
                    // Card no longer hovered
                }
            }
        }

        self.hovered_card = new_hovered;
    }

    /// Handle click event
    pub fn handle_click(&mut self, pos: (f32, f32), cards: &HashMap<u32, MediaCard>) -> Option<CardAction> {
        let now = Instant::now();

        // Check for double-click
        let is_double_click = if let (Some(last_time), Some(last_pos)) = (self.last_click_time, self.last_click_pos) {
            let time_diff = now.duration_since(last_time);
            let distance = ((pos.0 - last_pos.0).powi(2) + (pos.1 - last_pos.1).powi(2)).sqrt();

            time_diff.as_millis() < 300 && distance < 10.0
        } else {
            false
        };

        self.last_click_time = Some(now);
        self.last_click_pos = Some(pos);

        // Find clicked card
        if let Some(card_id) = self.card_at_point(pos, cards) {
            if let Some(card) = cards.get(&card_id) {
                // Check if clicked on action button
                if let Some(action) = card.action_at_point(pos) {
                    return Some(action.clone());
                }

                // Double-click to zoom
                if is_double_click {
                    return Some(CardAction::Zoom);
                }

                // Single click to select
                self.select_card(card_id);
            }
        }

        None
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: KeyCode, cards: &HashMap<u32, MediaCard>) -> Option<CardAction> {
        match key {
            KeyCode::Enter => {
                if let Some(card_id) = self.focused_card {
                    Some(CardAction::Zoom)
                } else {
                    None
                }
            }
            KeyCode::Space => {
                if let Some(card_id) = self.focused_card {
                    self.toggle_selection(card_id);
                }
                None
            }
            KeyCode::Escape => {
                if self.fullscreen_card.is_some() {
                    self.fullscreen_card = None;
                    Some(CardAction::Zoom) // Exit fullscreen
                } else {
                    self.clear_selection();
                    None
                }
            }
            KeyCode::ArrowLeft => {
                self.focus_previous(cards);
                None
            }
            KeyCode::ArrowRight => {
                self.focus_next(cards);
                None
            }
            KeyCode::Delete => {
                if !self.selected_cards.is_empty() {
                    Some(CardAction::Delete)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Find card at point
    fn card_at_point(&self, point: (f32, f32), cards: &HashMap<u32, MediaCard>) -> Option<u32> {
        // Find topmost card at point (iterate in reverse for z-order)
        let mut found = None;
        let mut max_elevation = f32::MIN;

        for (id, card) in cards {
            if card.contains_point(point) && card.state != CardState::Hidden {
                let elevation = card.current_elevation();
                if elevation > max_elevation {
                    max_elevation = elevation;
                    found = Some(*id);
                }
            }
        }

        found
    }

    /// Select a card
    fn select_card(&mut self, card_id: u32) {
        if !self.selected_cards.contains(&card_id) {
            self.selected_cards.push(card_id);
        }
        self.focused_card = Some(card_id);
    }

    /// Toggle card selection
    fn toggle_selection(&mut self, card_id: u32) {
        if let Some(pos) = self.selected_cards.iter().position(|&id| id == card_id) {
            self.selected_cards.remove(pos);
        } else {
            self.selected_cards.push(card_id);
        }
    }

    /// Clear all selections
    fn clear_selection(&mut self) {
        self.selected_cards.clear();
    }

    /// Focus next card
    fn focus_next(&mut self, cards: &HashMap<u32, MediaCard>) {
        let card_ids: Vec<u32> = cards.keys().copied().collect();
        if card_ids.is_empty() {
            return;
        }

        if let Some(current) = self.focused_card {
            if let Some(pos) = card_ids.iter().position(|&id| id == current) {
                let next_pos = (pos + 1) % card_ids.len();
                self.focused_card = Some(card_ids[next_pos]);
                return;
            }
        }

        // No focus or not found - focus first
        self.focused_card = card_ids.first().copied();
    }

    /// Focus previous card
    fn focus_previous(&mut self, cards: &HashMap<u32, MediaCard>) {
        let card_ids: Vec<u32> = cards.keys().copied().collect();
        if card_ids.is_empty() {
            return;
        }

        if let Some(current) = self.focused_card {
            if let Some(pos) = card_ids.iter().position(|&id| id == current) {
                let prev_pos = if pos == 0 {
                    card_ids.len() - 1
                } else {
                    pos - 1
                };
                self.focused_card = Some(card_ids[prev_pos]);
                return;
            }
        }

        // No focus or not found - focus last
        self.focused_card = card_ids.last().copied();
    }

    /// Enter fullscreen mode for a card
    pub fn enter_fullscreen(&mut self, card_id: u32) {
        self.fullscreen_card = Some(card_id);
    }

    /// Exit fullscreen mode
    pub fn exit_fullscreen(&mut self) {
        self.fullscreen_card = None;
    }

    /// Check if in fullscreen mode
    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen_card.is_some()
    }
}

/// Key codes for keyboard handling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Enter,
    Space,
    Escape,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Delete,
    A, // Ctrl+A for select all
    S, // Ctrl+S for save
    C, // Ctrl+C for copy
    V, // Ctrl+V for paste
    Z, // Ctrl+Z for undo
    Plus,  // Zoom in
    Minus, // Zoom out
    F,     // Fullscreen
}

/// Click target for more precise hit testing
#[derive(Debug, Clone, PartialEq)]
pub enum ClickTarget {
    /// Clicked on card body
    CardBody(u32),
    /// Clicked on action button
    ActionButton { card_id: u32, action: CardAction },
    /// Clicked on gallery header/title
    GalleryHeader(String),
    /// Clicked on empty space
    Empty,
}

/// Helper to determine click target with detailed info
pub fn determine_click_target(pos: (f32, f32), cards: &HashMap<u32, MediaCard>) -> ClickTarget {
    // Check cards in order of elevation (highest first)
    let mut sorted_cards: Vec<_> = cards.iter().collect();
    sorted_cards.sort_by(|a, b| {
        b.1.current_elevation()
            .partial_cmp(&a.1.current_elevation())
            .unwrap()
    });

    for (&card_id, card) in sorted_cards {
        if card.state == CardState::Hidden {
            continue;
        }

        // Check action buttons first (they're on top)
        if let Some(action) = card.action_at_point(pos) {
            return ClickTarget::ActionButton {
                card_id,
                action: action.clone(),
            };
        }

        // Check card body
        if card.contains_point(pos) {
            return ClickTarget::CardBody(card_id);
        }
    }

    ClickTarget::Empty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interaction_state_hover() {
        let mut state = InteractionState::new();
        let cards = HashMap::new();

        state.update_hover(Some((100.0, 100.0)), &cards);
        assert_eq!(state.hovered_card, None);
    }

    #[test]
    fn test_select_card() {
        let mut state = InteractionState::new();

        state.select_card(1);
        assert_eq!(state.selected_cards, vec![1]);
        assert_eq!(state.focused_card, Some(1));

        state.select_card(2);
        assert_eq!(state.selected_cards, vec![1, 2]);
    }

    #[test]
    fn test_toggle_selection() {
        let mut state = InteractionState::new();

        state.toggle_selection(1);
        assert_eq!(state.selected_cards, vec![1]);

        state.toggle_selection(1);
        assert_eq!(state.selected_cards, vec![]);
    }

    #[test]
    fn test_fullscreen() {
        let mut state = InteractionState::new();

        assert!(!state.is_fullscreen());

        state.enter_fullscreen(1);
        assert!(state.is_fullscreen());
        assert_eq!(state.fullscreen_card, Some(1));

        state.exit_fullscreen();
        assert!(!state.is_fullscreen());
    }
}
