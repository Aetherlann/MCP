//! Split direction and logic

use serde::{Deserialize, Serialize};

/// Direction for splitting a region
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    /// Split horizontally (top/bottom)
    Horizontal,
    /// Split vertically (left/right)
    Vertical,
}

/// Represents a split configuration
#[derive(Debug, Clone)]
pub struct Split {
    pub direction: SplitDirection,
    pub ratio: f32, // 0.0 to 1.0, where 0.5 is 50/50
}

impl Split {
    pub fn new(direction: SplitDirection, ratio: f32) -> Self {
        Self {
            direction,
            ratio: ratio.clamp(0.01, 0.99),
        }
    }

    pub fn horizontal(ratio: f32) -> Self {
        Self::new(SplitDirection::Horizontal, ratio)
    }

    pub fn vertical(ratio: f32) -> Self {
        Self::new(SplitDirection::Vertical, ratio)
    }
}
