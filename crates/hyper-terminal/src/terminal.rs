use anyhow::Result;
use ht_config::Config;
use ht_pty::{Pty, PtySize, PtyEvent};
use ht_vt::{VtParser, VtToken, Grid};
use ht_renderer::Renderer;
use ht_media::MediaManager;
use winit::keyboard::KeyCode;

pub struct Terminal {
    window: winit::window::Window,
    config: Config,
    pty: Box<dyn Pty>,
    parser: VtParser,
    grid: Grid,
    renderer: Renderer,
    media: MediaManager,
}

impl Terminal {
    pub async fn new(window: winit::window::Window, config: Config) -> Result<Self> {
        // Create PTY
        #[cfg(windows)]
        let mut pty: Box<dyn Pty> = Box::new(ht_pty::conpty::ConPty::new());

        #[cfg(unix)]
        let mut pty: Box<dyn Pty> = Box::new(ht_pty::openpty::OpenPty::new());

        // Spawn shell from first profile
        let profile = config.profiles.first()
            .ok_or_else(|| anyhow::anyhow!("No profiles configured"))?;

        let size = window.inner_size();
        let cols = (size.width / 10).max(80) as u16;  // Rough estimate
        let rows = (size.height / 20).max(24) as u16;

        pty.spawn(
            &profile.shell,
            &profile.args,
            &profile.env,
            profile.cwd.clone(),
            PtySize::new(rows, cols),
        ).await?;

        tracing::info!("PTY spawned: {} ({}x{})", profile.name, cols, rows);

        // Create grid
        let grid = Grid::new(
            rows as usize,
            cols as usize,
            config.behavior.scrollback,
        );

        // Create renderer
        let renderer = Renderer::new(&window).await?;

        Ok(Self {
            window,
            config,
            pty,
            parser: VtParser::new(),
            grid,
            renderer,
            media: MediaManager::new(),
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        // Update renderer
        self.renderer.resize(width, height);

        // Calculate new grid size
        let cols = (width / 10).max(80) as u16;
        let rows = (height / 20).max(24) as u16;

        // Resize grid
        self.grid.resize(rows as usize, cols as usize);

        // Resize PTY
        let size = PtySize::new(rows, cols);
        if let Err(e) = tokio::runtime::Handle::current().block_on(self.pty.resize(size)) {
            tracing::error!("Failed to resize PTY: {}", e);
        }

        tracing::debug!("Resized to {}x{}", cols, rows);
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        let data = match key {
            KeyCode::Enter => b"\r".to_vec(),
            KeyCode::Backspace => b"\x7f".to_vec(),
            KeyCode::Tab => b"\t".to_vec(),
            KeyCode::Escape => b"\x1b".to_vec(),
            KeyCode::ArrowUp => b"\x1b[A".to_vec(),
            KeyCode::ArrowDown => b"\x1b[B".to_vec(),
            KeyCode::ArrowRight => b"\x1b[C".to_vec(),
            KeyCode::ArrowLeft => b"\x1b[D".to_vec(),
            KeyCode::Home => b"\x1b[H".to_vec(),
            KeyCode::End => b"\x1b[F".to_vec(),
            KeyCode::PageUp => b"\x1b[5~".to_vec(),
            KeyCode::PageDown => b"\x1b[6~".to_vec(),
            KeyCode::Delete => b"\x1b[3~".to_vec(),
            _ => return,
        };

        if let Err(e) = tokio::runtime::Handle::current().block_on(self.pty.write(&data)) {
            tracing::error!("Failed to write to PTY: {}", e);
        }
    }

    pub fn process_pty(&mut self) {
        // Non-blocking read from PTY
        // In a real implementation, this would use async channels
        // For now, we'll skip the actual reading to keep it simple
    }

    pub fn render(&mut self) -> Result<()> {
        self.renderer.render(&self.grid)?;
        Ok(())
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    fn apply_token(&mut self, token: VtToken) {
        match token {
            VtToken::Print(ch) => self.grid.put_char(ch),
            VtToken::CarriageReturn => self.grid.carriage_return(),
            VtToken::LineFeed => self.grid.line_feed(),
            VtToken::Backspace => self.grid.backspace(),
            VtToken::Tab => self.grid.tab(),
            VtToken::CursorMove(x, y) => self.grid.move_cursor(x, y),
            VtToken::CursorRelative(dx, dy) => self.grid.move_cursor_relative(dx, dy),
            VtToken::SetGraphics(attrs) => self.grid.set_attributes(attrs),
            VtToken::ClearScreen => self.grid.clear_screen(),
            VtToken::ClearLine => self.grid.clear_line(),
            VtToken::Graphics(cmd) => {
                if let Err(e) = self.media.handle_graphics(cmd) {
                    tracing::error!("Failed to handle graphics: {}", e);
                }
            }
            VtToken::Hyperlink { url, id } => {
                tracing::debug!("Hyperlink: {} (id: {:?})", url, id);
            }
            VtToken::Clipboard(data) => {
                tracing::debug!("Clipboard: {}", data);
            }
            VtToken::Bell => {
                tracing::debug!("Bell");
            }
            VtToken::Unknown => {}
        }
    }
}
