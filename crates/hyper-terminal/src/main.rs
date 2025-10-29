use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use winit::event::{Event, WindowEvent, ElementState, MouseButton};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};

mod terminal;

use terminal::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hyper_terminal=info,ht_pty=info,ht_vt=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Hyper Terminal starting...");

    // Load configuration
    let config = ht_config::Config::load()?;
    tracing::info!("Configuration loaded from {:?}", ht_config::Config::config_path()?);

    // Create event loop and window
    let event_loop = EventLoop::new()?;
    let window = ht_ui::create_window(&event_loop)?;

    // Apply glass effect
    let glass = ht_ui::GlassEffect::new(
        config.appearance.glass.opacity,
        config.appearance.glass.blur_radius,
    );

    if config.appearance.glass.enabled {
        if let Err(e) = glass.apply(&window) {
            tracing::warn!("Failed to apply glass effect: {}", e);
        }
    }

    // Create terminal
    let mut terminal = Terminal::new(window, config).await?;

    tracing::info!("Terminal initialized, entering event loop");

    // Track cursor position for mouse clicks
    let mut cursor_pos = (0.0_f32, 0.0_f32);

    // Track time for animations
    let mut last_update = std::time::Instant::now();

    // Run event loop
    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    tracing::info!("Close requested, exiting");
                    elwt.exit();
                }
                WindowEvent::Resized(size) => {
                    terminal.resize(size.width, size.height);
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state.is_pressed() {
                        // Handle special keys first
                        if let PhysicalKey::Code(code) = event.physical_key {
                            // Try gallery navigation first
                            let handled_by_gallery = terminal.handle_gallery_navigation(code);

                            if !handled_by_gallery {
                                // Check if this is a special key
                                let is_special = matches!(code,
                                    KeyCode::Enter | KeyCode::Backspace | KeyCode::Tab |
                                    KeyCode::Escape | KeyCode::ArrowUp | KeyCode::ArrowDown |
                                    KeyCode::ArrowLeft | KeyCode::ArrowRight | KeyCode::Home |
                                    KeyCode::End | KeyCode::PageUp | KeyCode::PageDown |
                                    KeyCode::Delete
                                );

                                if is_special {
                                    terminal.handle_key(code);
                                } else if let Some(ref text) = event.text {
                                    // Handle text input
                                    for ch in text.chars() {
                                        terminal.handle_char(ch);
                                    }
                                }
                            }
                        } else if let Some(ref text) = event.text {
                            // Handle text input for non-physical keys
                            for ch in text.chars() {
                                terminal.handle_char(ch);
                            }
                        }
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    cursor_pos = (position.x as f32, position.y as f32);
                    terminal.handle_cursor_moved(cursor_pos.0, cursor_pos.1);
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left && state == ElementState::Pressed {
                        terminal.handle_mouse_click(cursor_pos.0, cursor_pos.1);
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Err(e) = terminal.render() {
                        tracing::error!("Render error: {}", e);
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                // Update animations
                let now = std::time::Instant::now();
                let delta_time = now.duration_since(last_update);
                last_update = now;
                terminal.update_animations(delta_time);

                // Process PTY output
                terminal.process_pty();

                // Request redraw
                terminal.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}
