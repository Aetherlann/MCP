// Core type definitions for the gallery system

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

/// Gallery display modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GalleryMode {
    /// Automatic mode selection based on content
    Auto,
    /// Responsive grid layout
    Grid,
    /// Pinterest-style masonry layout
    Masonry,
    /// Horizontal filmstrip for sequential content
    Filmstrip,
    /// Side-by-side comparison
    Comparison,
    /// Stackable cards with focus
    Deck,
    /// File tree explorer with previews
    FileExplorer,
}

/// Media content types
#[derive(Debug, Clone)]
pub enum MediaContent {
    /// Image data (RGBA format)
    Image(ImageData),
    /// Video data
    Video(VideoData),
    /// Audio data
    Audio(AudioData),
    /// File data (text, code, etc.)
    File(FileData),
    /// Plain text
    Text(String),
    /// Mixed content (composite card)
    Mixed(Vec<MediaContent>),
}

/// Image data
#[derive(Debug, Clone)]
pub struct ImageData {
    pub rgba_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub size_bytes: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    WebP,
    Bmp,
    Svg,
}

impl ImageFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "gif" => Some(Self::Gif),
            "webp" => Some(Self::WebP),
            "bmp" => Some(Self::Bmp),
            "svg" => Some(Self::Svg),
            _ => None,
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Gif => "image/gif",
            Self::WebP => "image/webp",
            Self::Bmp => "image/bmp",
            Self::Svg => "image/svg+xml",
        }
    }
}

/// Video data
#[derive(Debug, Clone)]
pub struct VideoData {
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub duration: f64,
    pub format: String,
}

/// Audio data
#[derive(Debug, Clone)]
pub struct AudioData {
    pub path: PathBuf,
    pub duration: f64,
    pub format: String,
    pub sample_rate: u32,
}

/// File data
#[derive(Debug, Clone)]
pub struct FileData {
    pub path: PathBuf,
    pub content: String,
    pub file_type: FileType,
    pub size_bytes: usize,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Code,
    Text,
    Json,
    Xml,
    Yaml,
    Markdown,
    Config,
    Binary,
}

/// Card metadata
#[derive(Debug, Clone, Default)]
pub struct CardMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub timestamp: Option<SystemTime>,
    pub source: String,
    pub author: Option<String>,
    pub annotations: Vec<Annotation>,
}

/// Annotation for marking up content
#[derive(Debug, Clone)]
pub struct Annotation {
    pub label: String,
    pub position: (f32, f32),
    pub style: AnnotationStyle,
}

#[derive(Debug, Clone)]
pub enum AnnotationStyle {
    Point,
    Box(f32, f32),  // width, height
    Arrow { from: (f32, f32), to: (f32, f32) },
}

/// Card layout information
#[derive(Debug, Clone)]
pub struct CardLayout {
    pub bounds: Rect,
    pub size: CardSize,
    pub aspect_ratio: f32,
    pub padding: f32,
    pub border_radius: f32,
    pub elevation: f32,
}

impl Default for CardLayout {
    fn default() -> Self {
        Self {
            bounds: Rect::default(),
            size: CardSize::Medium,
            aspect_ratio: 1.0,
            padding: 12.0,
            border_radius: 8.0,
            elevation: 1.0,
        }
    }
}

/// Card size presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardSize {
    Thumbnail,  // 64px
    Small,      // 128px
    Medium,     // 256px
    Large,      // 512px
    Full,       // Fit to container
    Auto,       // Based on content
}

impl CardSize {
    pub fn base_dimension(&self) -> u32 {
        match self {
            Self::Thumbnail => 64,
            Self::Small => 128,
            Self::Medium => 256,
            Self::Large => 512,
            Self::Full => 0,  // Calculated
            Self::Auto => 0,  // Calculated
        }
    }
}

/// Card interaction configuration
#[derive(Debug, Clone)]
pub struct CardInteractions {
    pub hoverable: bool,
    pub clickable: bool,
    pub draggable: bool,
    pub selectable: bool,
    pub actions: Vec<CardAction>,
}

impl Default for CardInteractions {
    fn default() -> Self {
        Self {
            hoverable: true,
            clickable: true,
            draggable: false,
            selectable: true,
            actions: vec![
                CardAction::Save,
                CardAction::Copy,
                CardAction::Zoom,
            ],
        }
    }
}

/// Card actions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardAction {
    Save,
    Copy,
    Share,
    Zoom,
    Edit,
    Delete,
    ViewDetails,
    OpenExternal,
}

impl CardAction {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Save => "💾",
            Self::Copy => "📋",
            Self::Share => "🔗",
            Self::Zoom => "🔍",
            Self::Edit => "✏️",
            Self::Delete => "🗑️",
            Self::ViewDetails => "ℹ️",
            Self::OpenExternal => "↗️",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Save => "Save",
            Self::Copy => "Copy",
            Self::Share => "Share",
            Self::Zoom => "Zoom",
            Self::Edit => "Edit",
            Self::Delete => "Delete",
            Self::ViewDetails => "Details",
            Self::OpenExternal => "Open",
        }
    }
}

/// Card state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardState {
    Normal,
    Hovered,
    Selected,
    Fullscreen,
    Hidden,
}

impl Default for CardState {
    fn default() -> Self {
        Self::Normal
    }
}

/// Rectangle for bounds and layout
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, point: (f32, f32)) -> bool {
        point.0 >= self.x
            && point.0 <= self.x + self.width
            && point.1 >= self.y
            && point.1 <= self.y + self.height
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    pub fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    pub fn aspect_ratio(&self) -> f32 {
        if self.height > 0.0 {
            self.width / self.height
        } else {
            1.0
        }
    }
}

/// Size structure
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn aspect_ratio(&self) -> f32 {
        if self.height > 0.0 {
            self.width / self.height
        } else {
            1.0
        }
    }
}

/// Color with alpha
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a,
        }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        if hex.starts_with('#') && hex.len() == 7 {
            let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
            let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
            let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
            Some(Self::rgb(r, g, b))
        } else {
            None
        }
    }

    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(10.0, 10.0, 20.0, 20.0);
        assert!(rect.contains((15.0, 15.0)));
        assert!(!rect.contains((5.0, 5.0)));
    }

    #[test]
    fn test_rect_intersects() {
        let rect1 = Rect::new(0.0, 0.0, 10.0, 10.0);
        let rect2 = Rect::new(5.0, 5.0, 10.0, 10.0);
        let rect3 = Rect::new(20.0, 20.0, 10.0, 10.0);

        assert!(rect1.intersects(&rect2));
        assert!(!rect1.intersects(&rect3));
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#FF0000").unwrap();
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.0);
        assert_eq!(color.b, 0.0);
    }
}
