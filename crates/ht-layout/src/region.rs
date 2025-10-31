//! Region management for layout system

use crate::{Rect, SplitDirection};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Content that can be displayed in a region
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RegionContent {
    /// Terminal output
    Terminal,
    /// Gallery display
    Gallery { gallery_id: String },
    /// Widget display
    Widget { widget_id: String },
    /// Markdown content
    Markdown { content: String },
    /// Code block with syntax highlighting
    Code { language: String, content: String },
    /// Tree view (JSON/YAML/file structure)
    Tree { data: serde_json::Value },
    /// Data table
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    /// Progress indicator
    Progress { value: f32, label: Option<String> },
    /// Custom content type
    Custom { content_type: String, data: serde_json::Value },
}

/// A region in the layout tree
#[derive(Debug)]
pub struct Region {
    /// Unique identifier
    pub id: String,
    /// Bounds of this region
    pub bounds: Rect,
    /// Content displayed in this region (if leaf)
    pub content: Option<RegionContent>,
    /// Child regions (if split)
    pub children: Option<Vec<Region>>,
    /// Split direction (if split)
    pub split_direction: Option<SplitDirection>,
    /// Split ratio (if split)
    pub split_ratio: Option<f32>,
    /// Whether this region is visible
    pub visible: bool,
    /// Z-index for overlapping regions
    pub z_index: i32,
}

impl Region {
    /// Create a new region
    pub fn new(id: String, bounds: Rect) -> Self {
        Self {
            id,
            bounds,
            content: None,
            children: None,
            split_direction: None,
            split_ratio: None,
            visible: true,
            z_index: 0,
        }
    }

    /// Check if this is a leaf region (can display content)
    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    /// Set the bounds of this region
    pub fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    /// Set the content of this region
    pub fn set_content(&mut self, content: RegionContent) {
        self.content = Some(content);
    }

    /// Split this region into two child regions
    pub fn split(&mut self, direction: SplitDirection, ratio: f32) -> Result<(String, String)> {
        if self.children.is_some() {
            return Err(anyhow::anyhow!("Region is already split"));
        }

        let ratio = ratio.clamp(0.01, 0.99);

        let (bounds1, bounds2) = self.calculate_split_bounds(direction, ratio);

        let id1 = format!("{}_{}", self.id, Uuid::new_v4().simple());
        let id2 = format!("{}_{}", self.id, Uuid::new_v4().simple());

        let mut child1 = Region::new(id1.clone(), bounds1);
        let mut child2 = Region::new(id2.clone(), bounds2);

        // Transfer content to first child if this region had content
        if let Some(content) = self.content.take() {
            child1.content = Some(content);
        }

        self.children = Some(vec![child1, child2]);
        self.split_direction = Some(direction);
        self.split_ratio = Some(ratio);

        Ok((id1, id2))
    }

    /// Remove split and merge back into single region
    pub fn unsplit(&mut self) -> Result<()> {
        if self.children.is_none() {
            return Err(anyhow::anyhow!("Region is not split"));
        }

        self.children = None;
        self.split_direction = None;
        self.split_ratio = None;
        self.content = Some(RegionContent::Terminal);

        Ok(())
    }

    /// Calculate bounds for child regions after split
    fn calculate_split_bounds(&self, direction: SplitDirection, ratio: f32) -> (Rect, Rect) {
        match direction {
            SplitDirection::Horizontal => {
                let split_y = self.bounds.y + self.bounds.height * ratio;
                let height1 = self.bounds.height * ratio;
                let height2 = self.bounds.height * (1.0 - ratio);

                let bounds1 = Rect::new(self.bounds.x, self.bounds.y, self.bounds.width, height1);
                let bounds2 = Rect::new(self.bounds.x, split_y, self.bounds.width, height2);

                (bounds1, bounds2)
            }
            SplitDirection::Vertical => {
                let split_x = self.bounds.x + self.bounds.width * ratio;
                let width1 = self.bounds.width * ratio;
                let width2 = self.bounds.width * (1.0 - ratio);

                let bounds1 = Rect::new(self.bounds.x, self.bounds.y, width1, self.bounds.height);
                let bounds2 = Rect::new(split_x, self.bounds.y, width2, self.bounds.height);

                (bounds1, bounds2)
            }
        }
    }

    /// Recalculate layout for this region and all children
    pub fn recalculate_layout(&mut self) {
        if let Some(ref mut children) = self.children {
            if let (Some(direction), Some(ratio)) = (self.split_direction, self.split_ratio) {
                let (bounds1, bounds2) = self.calculate_split_bounds(direction, ratio);

                if let Some(child1) = children.get_mut(0) {
                    child1.set_bounds(bounds1);
                    child1.recalculate_layout();
                }

                if let Some(child2) = children.get_mut(1) {
                    child2.set_bounds(bounds2);
                    child2.recalculate_layout();
                }
            }
        }
    }

    /// Find a child region by ID
    pub fn find_child(&self, id: &str) -> Option<&Region> {
        if self.id == id {
            return Some(self);
        }

        if let Some(ref children) = self.children {
            for child in children {
                if let Some(found) = child.find_child(id) {
                    return Some(found);
                }
            }
        }

        None
    }

    /// Find a child region by ID (mutable)
    pub fn find_child_mut(&mut self, id: &str) -> Option<&mut Region> {
        if self.id == id {
            return Some(self);
        }

        if let Some(ref mut children) = self.children {
            for child in children {
                if let Some(found) = child.find_child_mut(id) {
                    return Some(found);
                }
            }
        }

        None
    }
}
