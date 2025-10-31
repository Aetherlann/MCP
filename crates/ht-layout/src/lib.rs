//! Layout Manager for flexible terminal splitting and region management
//!
//! This crate provides a powerful layout system that allows LLMs to split
//! the terminal into arbitrary regions and display different content in each.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod rect;
pub mod split;
pub mod region;

pub use rect::Rect;
pub use split::{SplitDirection, Split};
pub use region::{Region, RegionContent};

/// Layout manager that handles terminal splitting and region management
#[derive(Debug)]
pub struct LayoutManager {
    /// Root region (entire terminal)
    root: Box<Region>,
    /// Map of region IDs to regions for quick access
    regions: HashMap<String, *mut Region>,
    /// Current terminal dimensions
    terminal_size: (u32, u32),
}

impl LayoutManager {
    /// Create a new layout manager
    pub fn new(width: u32, height: u32) -> Self {
        let root = Box::new(Region::new(
            "root".to_string(),
            Rect::new(0.0, 0.0, width as f32, height as f32),
        ));

        let mut regions = HashMap::new();
        regions.insert("root".to_string(), &*root as *const Region as *mut Region);

        Self {
            root,
            regions,
            terminal_size: (width, height),
        }
    }

    /// Update terminal size and recalculate layouts
    pub fn resize(&mut self, width: u32, height: u32) {
        self.terminal_size = (width, height);
        self.root.set_bounds(Rect::new(0.0, 0.0, width as f32, height as f32));
        self.recalculate_layout();
    }

    /// Split a region horizontally or vertically
    pub fn split_region(
        &mut self,
        region_id: &str,
        direction: SplitDirection,
        ratio: f32,
    ) -> Result<(String, String)> {
        let region = self.get_region_mut(region_id)?;
        let (id1, id2) = region.split(direction, ratio)?;

        // Register new regions
        unsafe {
            if let Some(children) = &mut region.children {
                for child in children.iter_mut() {
                    self.regions.insert(child.id.clone(), child as *mut Region);
                }
            }
        }

        Ok((id1, id2))
    }

    /// Remove a split and merge regions
    pub fn unsplit_region(&mut self, region_id: &str) -> Result<()> {
        let region = self.get_region_mut(region_id)?;
        region.unsplit()?;

        // Clean up region map
        self.regions.retain(|id, _| {
            // Keep only regions that still exist in tree
            self.region_exists(id)
        });

        Ok(())
    }

    /// Set content for a specific region
    pub fn set_region_content(&mut self, region_id: &str, content: RegionContent) -> Result<()> {
        let region = self.get_region_mut(region_id)?;
        region.set_content(content);
        Ok(())
    }

    /// Get a region by ID
    pub fn get_region(&self, region_id: &str) -> Result<&Region> {
        self.regions
            .get(region_id)
            .map(|ptr| unsafe { &**ptr })
            .ok_or_else(|| anyhow::anyhow!("Region not found: {}", region_id))
    }

    /// Get a mutable region by ID
    fn get_region_mut(&mut self, region_id: &str) -> Result<&mut Region> {
        self.regions
            .get(region_id)
            .map(|ptr| unsafe { &mut **ptr })
            .ok_or_else(|| anyhow::anyhow!("Region not found: {}", region_id))
    }

    /// Get all leaf regions (regions that can actually display content)
    pub fn get_leaf_regions(&self) -> Vec<&Region> {
        let mut leaves = Vec::new();
        self.collect_leaves(&*self.root, &mut leaves);
        leaves
    }

    fn collect_leaves<'a>(&self, region: &'a Region, leaves: &mut Vec<&'a Region>) {
        if region.is_leaf() {
            leaves.push(region);
        } else if let Some(ref children) = region.children {
            for child in children {
                self.collect_leaves(child, leaves);
            }
        }
    }

    /// Check if a region exists in the tree
    fn region_exists(&self, region_id: &str) -> bool {
        self.find_region_in_tree(&*self.root, region_id).is_some()
    }

    fn find_region_in_tree<'a>(&self, region: &'a Region, id: &str) -> Option<&'a Region> {
        if region.id == id {
            return Some(region);
        }

        if let Some(ref children) = region.children {
            for child in children {
                if let Some(found) = self.find_region_in_tree(child, id) {
                    return Some(found);
                }
            }
        }

        None
    }

    /// Recalculate all region layouts after resize
    fn recalculate_layout(&mut self) {
        self.root.recalculate_layout();
    }

    /// Get the root region
    pub fn root(&self) -> &Region {
        &*self.root
    }

    /// Create a complex layout from specification
    pub fn apply_layout(&mut self, spec: LayoutSpec) -> Result<()> {
        // Clear current layout
        self.root.unsplit()?;

        // Apply new layout
        self.apply_layout_recursive(&spec, "root")?;

        Ok(())
    }

    fn apply_layout_recursive(&mut self, spec: &LayoutSpec, region_id: &str) -> Result<()> {
        match spec {
            LayoutSpec::Single { content } => {
                if let Some(content) = content {
                    self.set_region_content(region_id, content.clone())?;
                }
            }
            LayoutSpec::Split { direction, ratio, first, second } => {
                let (id1, id2) = self.split_region(region_id, *direction, *ratio)?;
                self.apply_layout_recursive(first, &id1)?;
                self.apply_layout_recursive(second, &id2)?;
            }
        }
        Ok(())
    }

    /// Export current layout as a specification
    pub fn export_layout(&self) -> LayoutSpec {
        self.export_region_layout(&*self.root)
    }

    fn export_region_layout(&self, region: &Region) -> LayoutSpec {
        if region.is_leaf() {
            LayoutSpec::Single {
                content: region.content.clone(),
            }
        } else if let Some(ref children) = region.children {
            if children.len() == 2 {
                LayoutSpec::Split {
                    direction: region.split_direction.unwrap_or(SplitDirection::Horizontal),
                    ratio: region.split_ratio.unwrap_or(0.5),
                    first: Box::new(self.export_region_layout(&children[0])),
                    second: Box::new(self.export_region_layout(&children[1])),
                }
            } else {
                LayoutSpec::Single { content: None }
            }
        } else {
            LayoutSpec::Single { content: None }
        }
    }
}

/// Layout specification for creating complex layouts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LayoutSpec {
    Single {
        content: Option<RegionContent>,
    },
    Split {
        direction: SplitDirection,
        ratio: f32,
        first: Box<LayoutSpec>,
        second: Box<LayoutSpec>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_split() {
        let mut layout = LayoutManager::new(800, 600);

        let result = layout.split_region("root", SplitDirection::Horizontal, 0.5);
        assert!(result.is_ok());

        let leaves = layout.get_leaf_regions();
        assert_eq!(leaves.len(), 2);
    }

    #[test]
    fn test_nested_split() {
        let mut layout = LayoutManager::new(800, 600);

        let (left, right) = layout.split_region("root", SplitDirection::Vertical, 0.5).unwrap();
        let (top_left, _) = layout.split_region(&left, SplitDirection::Horizontal, 0.5).unwrap();

        let leaves = layout.get_leaf_regions();
        assert_eq!(leaves.len(), 3);

        let region = layout.get_region(&top_left).unwrap();
        assert!(region.is_leaf());
    }

    #[test]
    fn test_layout_spec() {
        let spec = LayoutSpec::Split {
            direction: SplitDirection::Horizontal,
            ratio: 0.3,
            first: Box::new(LayoutSpec::Single { content: None }),
            second: Box::new(LayoutSpec::Split {
                direction: SplitDirection::Vertical,
                ratio: 0.5,
                first: Box::new(LayoutSpec::Single { content: None }),
                second: Box::new(LayoutSpec::Single { content: None }),
            }),
        };

        let mut layout = LayoutManager::new(800, 600);
        let result = layout.apply_layout(spec);
        assert!(result.is_ok());

        let leaves = layout.get_leaf_regions();
        assert_eq!(leaves.len(), 3);
    }
}
