pub mod window;
pub mod glass;

pub use window::WindowManager;
pub use glass::GlassEffect;

use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use anyhow::Result;

pub fn create_window(event_loop: &EventLoop<()>) -> Result<winit::window::Window> {
    let window = WindowBuilder::new()
        .with_title("Hyper Terminal")
        .with_inner_size(winit::dpi::LogicalSize::new(1200, 800))
        .with_transparent(true)
        .with_decorations(true)
        .build(event_loop)?;

    Ok(window)
}
