pub mod decoder;
pub mod surface;
pub mod texture;

use anyhow::Result;
use ht_vt::GraphicsCommand;
use decoder::{decode_image, DecodedImage};

pub struct MediaManager {
    surfaces: Vec<MediaSurface>,
    next_id: u32,
}

pub struct MediaSurface {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub rgba_data: Vec<u8>,
    pub placement: SurfacePlacement,
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfacePlacement {
    Inline { row: usize, col: usize },
    Overlay { x: i32, y: i32 },
    Docked { side: DockSide },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockSide {
    Left,
    Right,
    Top,
    Bottom,
}

impl MediaManager {
    pub fn new() -> Self {
        Self {
            surfaces: Vec::new(),
            next_id: 0,
        }
    }

    pub fn handle_graphics(&mut self, cmd: GraphicsCommand, cursor_row: usize, cursor_col: usize) -> Result<Option<u32>> {
        match cmd {
            GraphicsCommand::Kitty(kitty) => {
                tracing::info!("Processing Kitty graphics: {} bytes", kitty.payload.len());

                // Decode image
                let decoded = decode_image(&kitty.payload)?;

                let id = self.next_id;
                self.next_id += 1;

                // Determine placement
                let placement = match kitty.placement {
                    ht_vt::KittyPlacement::Inline => SurfacePlacement::Inline {
                        row: cursor_row,
                        col: cursor_col,
                    },
                    ht_vt::KittyPlacement::Overlay => SurfacePlacement::Overlay { x: 0, y: 0 },
                };

                let surface = MediaSurface {
                    id,
                    width: decoded.width,
                    height: decoded.height,
                    rgba_data: decoded.rgba_data,
                    placement,
                    row: cursor_row,
                    col: cursor_col,
                };

                tracing::info!("Created media surface {} ({}x{})", id, decoded.width, decoded.height);

                self.surfaces.push(surface);
                Ok(Some(id))
            }
            GraphicsCommand::Iterm(iterm) => {
                tracing::info!("Processing iTerm2 image: {} bytes", iterm.payload.len());

                let decoded = decode_image(&iterm.payload)?;

                let id = self.next_id;
                self.next_id += 1;

                let placement = if iterm.inline {
                    SurfacePlacement::Inline {
                        row: cursor_row,
                        col: cursor_col,
                    }
                } else {
                    SurfacePlacement::Overlay { x: 0, y: 0 }
                };

                let surface = MediaSurface {
                    id,
                    width: decoded.width,
                    height: decoded.height,
                    rgba_data: decoded.rgba_data,
                    placement,
                    row: cursor_row,
                    col: cursor_col,
                };

                tracing::info!("Created media surface {} ({}x{})", id, decoded.width, decoded.height);

                self.surfaces.push(surface);
                Ok(Some(id))
            }
            GraphicsCommand::Sixel(_) => {
                tracing::warn!("Sixel graphics not yet implemented");
                Ok(None)
            }
        }
    }

    pub fn surfaces(&self) -> &[MediaSurface] {
        &self.surfaces
    }

    pub fn get_surface(&self, id: u32) -> Option<&MediaSurface> {
        self.surfaces.iter().find(|s| s.id == id)
    }

    pub fn clear_surfaces(&mut self) {
        self.surfaces.clear();
    }

    pub fn remove_surface(&mut self, id: u32) -> bool {
        if let Some(pos) = self.surfaces.iter().position(|s| s.id == id) {
            self.surfaces.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for MediaManager {
    fn default() -> Self {
        Self::new()
    }
}
