use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    /// Default terminal colors
    Default,
    /// ANSI 16 color palette (0-15)
    Indexed(u8),
    /// RGB color
    Rgb(u8, u8, u8),
}

impl Color {
    pub fn from_ansi_256(index: u8) -> Self {
        Color::Indexed(index)
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(r, g, b)
    }

    pub fn to_rgba(&self) -> [u8; 4] {
        match self {
            Color::Default => [255, 255, 255, 255],
            Color::Indexed(idx) => Self::ansi_256_to_rgb(*idx),
            Color::Rgb(r, g, b) => [*r, *g, *b, 255],
        }
    }

    fn ansi_256_to_rgb(idx: u8) -> [u8; 4] {
        // Standard 16 colors
        const ANSI_COLORS: [[u8; 3]; 16] = [
            [0, 0, 0],       // Black
            [128, 0, 0],     // Red
            [0, 128, 0],     // Green
            [128, 128, 0],   // Yellow
            [0, 0, 128],     // Blue
            [128, 0, 128],   // Magenta
            [0, 128, 128],   // Cyan
            [192, 192, 192], // White
            [128, 128, 128], // Bright Black
            [255, 0, 0],     // Bright Red
            [0, 255, 0],     // Bright Green
            [255, 255, 0],   // Bright Yellow
            [0, 0, 255],     // Bright Blue
            [255, 0, 255],   // Bright Magenta
            [0, 255, 255],   // Bright Cyan
            [255, 255, 255], // Bright White
        ];

        if idx < 16 {
            let rgb = ANSI_COLORS[idx as usize];
            [rgb[0], rgb[1], rgb[2], 255]
        } else if idx < 232 {
            // 216 color cube
            let idx = idx - 16;
            let r = (idx / 36) * 51;
            let g = ((idx % 36) / 6) * 51;
            let b = (idx % 6) * 51;
            [r, g, b, 255]
        } else {
            // Grayscale
            let gray = (idx - 232) * 10 + 8;
            [gray, gray, gray, 255]
        }
    }
}
