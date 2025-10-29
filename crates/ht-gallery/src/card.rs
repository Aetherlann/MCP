// Media card implementation

use crate::types::*;
use std::time::Instant;

/// A media card represents a single piece of content in the gallery
#[derive(Debug, Clone)]
pub struct MediaCard {
    pub id: u32,
    pub content: MediaContent,
    pub metadata: CardMetadata,
    pub layout: CardLayout,
    pub interactions: CardInteractions,
    pub state: CardState,

    // Runtime state
    pub hover_start: Option<Instant>,
    pub selected: bool,
    pub texture_id: Option<u32>,
}

impl MediaCard {
    pub fn new(id: u32, content: MediaContent, metadata: CardMetadata) -> Self {
        let layout = Self::calculate_layout(&content);

        Self {
            id,
            content,
            metadata,
            layout,
            interactions: CardInteractions::default(),
            state: CardState::Normal,
            hover_start: None,
            selected: false,
            texture_id: None,
        }
    }

    /// Calculate optimal layout for content
    fn calculate_layout(content: &MediaContent) -> CardLayout {
        let mut layout = CardLayout::default();

        match content {
            MediaContent::Image(img) => {
                layout.aspect_ratio = img.width as f32 / img.height as f32;
                layout.size = Self::size_for_image_dimensions(img.width, img.height);
            }
            MediaContent::Video(video) => {
                layout.aspect_ratio = video.width as f32 / video.height as f32;
                layout.size = CardSize::Large;
            }
            MediaContent::Text(_) => {
                layout.aspect_ratio = 1.5; // Wide card for text
                layout.size = CardSize::Medium;
            }
            MediaContent::File(_) => {
                layout.aspect_ratio = 1.33;
                layout.size = CardSize::Medium;
            }
            _ => {
                layout.size = CardSize::Medium;
            }
        }

        layout
    }

    fn size_for_image_dimensions(width: u32, height: u32) -> CardSize {
        let max_dim = width.max(height);

        if max_dim <= 128 {
            CardSize::Small
        } else if max_dim <= 512 {
            CardSize::Medium
        } else {
            CardSize::Large
        }
    }

    /// Get the title for this card
    pub fn title(&self) -> String {
        if let Some(ref title) = self.metadata.title {
            title.clone()
        } else {
            match &self.content {
                MediaContent::Image(img) => {
                    format!("Image ({}×{})", img.width, img.height)
                }
                MediaContent::Video(video) => {
                    format!("Video ({}×{})", video.width, video.height)
                }
                MediaContent::File(file) => {
                    file.path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("File")
                        .to_string()
                }
                MediaContent::Text(_) => "Text".to_string(),
                _ => format!("Media {}", self.id),
            }
        }
    }

    /// Get subtitle/metadata text
    pub fn subtitle(&self) -> Option<String> {
        match &self.content {
            MediaContent::Image(img) => Some(format!(
                "{} • {}",
                format_size(img.size_bytes),
                format!("{:?}", img.format)
            )),
            MediaContent::Video(video) => Some(format!(
                "{} • {}s",
                video.format,
                format_duration(video.duration)
            )),
            MediaContent::File(file) => {
                Some(format!("{:?} • {}", file.file_type, format_size(file.size_bytes)))
            }
            _ => None,
        }
    }

    /// Get description text
    pub fn description(&self) -> Option<&str> {
        self.metadata.description.as_deref()
    }

    /// Get tags
    pub fn tags(&self) -> &[String] {
        &self.metadata.tags
    }

    /// Check if point is over card
    pub fn contains_point(&self, point: (f32, f32)) -> bool {
        self.layout.bounds.contains(point)
    }

    /// Get action button bounds
    pub fn action_button_bounds(&self, action_index: usize) -> Option<Rect> {
        if action_index >= self.interactions.actions.len() {
            return None;
        }

        let button_size = 32.0;
        let button_spacing = 8.0;
        let total_width =
            (button_size + button_spacing) * self.interactions.actions.len() as f32 - button_spacing;

        let start_x = self.layout.bounds.x + (self.layout.bounds.width - total_width) / 2.0;
        let y = self.layout.bounds.y + self.layout.bounds.height - button_size - 12.0;

        let x = start_x + (button_size + button_spacing) * action_index as f32;

        Some(Rect::new(x, y, button_size, button_size))
    }

    /// Update hover state
    pub fn update_hover(&mut self, is_hovered: bool) {
        if is_hovered {
            if self.hover_start.is_none() {
                self.hover_start = Some(Instant::now());
            }
            self.state = CardState::Hovered;
        } else {
            self.hover_start = None;
            if self.state == CardState::Hovered {
                self.state = CardState::Normal;
            }
        }
    }

    /// Get hover duration
    pub fn hover_duration(&self) -> Option<std::time::Duration> {
        self.hover_start.map(|start| start.elapsed())
    }

    /// Calculate current elevation based on state and hover
    pub fn current_elevation(&self) -> f32 {
        let base = self.layout.elevation;

        match self.state {
            CardState::Normal => base,
            CardState::Hovered => {
                let duration = self.hover_duration().unwrap_or_default().as_secs_f32();
                let t = (duration * 4.0).min(1.0); // Fast transition
                base + t * 1.0 // Elevate by 1 level
            }
            CardState::Selected => base + 2.0,
            CardState::Fullscreen => 4.0,
            CardState::Hidden => 0.0,
        }
    }

    /// Calculate current scale based on state
    pub fn current_scale(&self) -> f32 {
        match self.state {
            CardState::Hovered => {
                let duration = self.hover_duration().unwrap_or_default().as_secs_f32();
                let t = (duration * 4.0).min(1.0);
                1.0 + t * 0.03 // 3% scale increase
            }
            CardState::Selected => 1.02,
            _ => 1.0,
        }
    }

    /// Get action at point (if any)
    pub fn action_at_point(&self, point: (f32, f32)) -> Option<&CardAction> {
        for (i, action) in self.interactions.actions.iter().enumerate() {
            if let Some(bounds) = self.action_button_bounds(i) {
                if bounds.contains(point) {
                    return Some(action);
                }
            }
        }
        None
    }
}

/// Format file size for display
fn format_size(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Format duration for display
fn format_duration(seconds: f64) -> String {
    let total_seconds = seconds as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1024 * 1024), "1.0 MB");
        assert_eq!(format_size(1536 * 1024), "1.5 MB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0.0), "0:00");
        assert_eq!(format_duration(30.0), "0:30");
        assert_eq!(format_duration(90.0), "1:30");
        assert_eq!(format_duration(3661.0), "1:01:01");
    }

    #[test]
    fn test_card_contains_point() {
        let mut card = MediaCard::new(
            0,
            MediaContent::Text("test".to_string()),
            CardMetadata::default(),
        );
        card.layout.bounds = Rect::new(10.0, 10.0, 100.0, 100.0);

        assert!(card.contains_point((50.0, 50.0)));
        assert!(!card.contains_point((5.0, 5.0)));
    }
}
