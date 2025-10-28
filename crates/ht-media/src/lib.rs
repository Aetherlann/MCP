pub mod decoder;
pub mod surface;

use anyhow::Result;
use ht_vt::GraphicsCommand;

pub struct MediaManager {
    surfaces: Vec<MediaSurface>,
}

pub struct MediaSurface {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub placement: SurfacePlacement,
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
        }
    }

    pub fn handle_graphics(&mut self, cmd: GraphicsCommand) -> Result<()> {
        match cmd {
            GraphicsCommand::Kitty(kitty) => {
                tracing::info!("Kitty graphics: {}x{} bytes",
                    kitty.width.unwrap_or(0),
                    kitty.payload.len()
                );

                let surface = MediaSurface {
                    id: kitty.image_id.unwrap_or(0),
                    width: kitty.width.unwrap_or(800),
                    height: kitty.height.unwrap_or(600),
                    data: kitty.payload,
                    placement: match kitty.placement {
                        ht_vt::KittyPlacement::Inline => SurfacePlacement::Inline { row: 0, col: 0 },
                        ht_vt::KittyPlacement::Overlay => SurfacePlacement::Overlay { x: 0, y: 0 },
                    },
                };

                self.surfaces.push(surface);
            }
            GraphicsCommand::Iterm(iterm) => {
                tracing::info!("iTerm image: {} bytes", iterm.payload.len());

                let surface = MediaSurface {
                    id: 0,
                    width: 800,
                    height: 600,
                    data: iterm.payload,
                    placement: if iterm.inline {
                        SurfacePlacement::Inline { row: 0, col: 0 }
                    } else {
                        SurfacePlacement::Overlay { x: 0, y: 0 }
                    },
                };

                self.surfaces.push(surface);
            }
            GraphicsCommand::Sixel(_) => {
                tracing::warn!("Sixel graphics not yet implemented");
            }
        }

        Ok(())
    }

    pub fn surfaces(&self) -> &[MediaSurface] {
        &self.surfaces
    }
}

impl Default for MediaManager {
    fn default() -> Self {
        Self::new()
    }
}
