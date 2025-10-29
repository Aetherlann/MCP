use anyhow::Result;
use ht_config::Config;
use ht_pty::{Pty, PtySize, PtyEvent};
use ht_vt::{VtParser, VtToken, Grid, GalleryControl};
use ht_renderer::Renderer;
use ht_media::MediaManager;
use ht_gallery::{GalleryManager, GalleryConfig, GalleryMode, MediaContent, CardMetadata, ImageData, ImageFormat};
use winit::keyboard::KeyCode;
use tokio::sync::mpsc;
use std::time::SystemTime;

pub struct Terminal {
    window: winit::window::Window,
    config: Config,
    pty_tx: mpsc::UnboundedSender<Vec<u8>>,
    pty_rx: mpsc::UnboundedReceiver<PtyEvent>,
    parser: VtParser,
    grid: Grid,
    renderer: Renderer,
    media: MediaManager,
    gallery: GalleryManager,

    // Gallery state
    current_gallery: Option<String>,
    pending_media_metadata: Option<CardMetadata>,
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

        // Create renderer first to get cell size
        let renderer = Renderer::new(&window).await?;
        let (cell_width, cell_height) = renderer.cell_size();

        let size = window.inner_size();
        let cols = ((size.width as f32 / cell_width).floor() as u16).max(80);
        let rows = ((size.height as f32 / cell_height).floor() as u16).max(24);

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

        // Create channels for PTY communication
        let (input_tx, mut input_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let (event_tx, event_rx) = mpsc::unbounded_channel::<PtyEvent>();

        // Spawn PTY I/O task
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // Handle input from terminal
                    Some(data) = input_rx.recv() => {
                        if let Err(e) = pty.write(&data).await {
                            tracing::error!("PTY write error: {}", e);
                            break;
                        }
                    }
                    // Handle output from PTY
                    event_result = pty.read_event() => {
                        match event_result {
                            Ok(event) => {
                                let is_exit = matches!(event, PtyEvent::Exit(_));
                                if event_tx.send(event).is_err() {
                                    tracing::error!("Failed to send PTY event");
                                    break;
                                }
                                if is_exit {
                                    break;
                                }
                            }
                            Err(e) => {
                                tracing::error!("PTY read error: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
            tracing::info!("PTY task exiting");
        });

        Ok(Self {
            window,
            config,
            pty_tx: input_tx,
            pty_rx: event_rx,
            parser: VtParser::new(),
            grid,
            renderer,
            media: MediaManager::new(),
            gallery: GalleryManager::new(),
            current_gallery: None,
            pending_media_metadata: None,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        // Update renderer
        self.renderer.resize(width, height);

        // Calculate new grid size based on cell dimensions
        let (cell_width, cell_height) = self.renderer.cell_size();
        let cols = ((width as f32 / cell_width).floor() as u16).max(80);
        let rows = ((height as f32 / cell_height).floor() as u16).max(24);

        // Resize grid
        self.grid.resize(rows as usize, cols as usize);

        tracing::debug!("Resized to {}x{} ({}x{} px)", cols, rows, width, height);
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

        if let Err(e) = self.pty_tx.send(data) {
            tracing::error!("Failed to send to PTY: {}", e);
        }
    }

    pub fn handle_char(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        let bytes = ch.encode_utf8(&mut buf).as_bytes().to_vec();

        if let Err(e) = self.pty_tx.send(bytes) {
            tracing::error!("Failed to send char to PTY: {}", e);
        }
    }

    pub fn process_pty(&mut self) {
        // Process all available PTY events
        while let Ok(event) = self.pty_rx.try_recv() {
            match event {
                PtyEvent::Data(data) => {
                    // Parse VT sequences
                    let tokens = self.parser.parse(&data);

                    // Apply tokens to grid
                    for token in tokens {
                        self.apply_token(token);
                    }
                }
                PtyEvent::Exit(code) => {
                    tracing::info!("Shell exited with code: {:?}", code);
                    // Could set a flag to close the terminal
                }
            }
        }
    }

    pub fn render(&mut self) -> Result<()> {
        self.renderer.render(&self.grid, &self.media)?;
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
                // Get cursor position for anchoring media
                let (col, row) = self.grid.cursor_pos();

                match self.media.handle_graphics(cmd, row, col) {
                    Ok(Some(id)) => {
                        // Upload texture to GPU
                        if let Some(surface) = self.media.get_surface(id) {
                            self.renderer.upload_media_texture(
                                id,
                                &surface.rgba_data,
                                surface.width,
                                surface.height,
                            );
                            tracing::info!("Uploaded media texture {} to GPU", id);

                            // If there's pending metadata and an active gallery, add to gallery
                            if let (Some(ref gallery_id), Some(metadata)) = (&self.current_gallery, self.pending_media_metadata.take()) {
                                let content = MediaContent::Image(ImageData {
                                    rgba_data: surface.rgba_data.clone(),
                                    width: surface.width,
                                    height: surface.height,
                                    format: ImageFormat::Png, // Assume PNG for now
                                    size_bytes: surface.rgba_data.len(),
                                });

                                if let Err(e) = self.gallery.add_media_to_gallery(gallery_id, content, metadata) {
                                    tracing::error!("Failed to add media to gallery: {}", e);
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        tracing::debug!("Graphics command handled but no surface created");
                    }
                    Err(e) => {
                        tracing::error!("Failed to handle graphics: {}", e);
                    }
                }
            }
            VtToken::GalleryCommand(cmd) => {
                self.handle_gallery_command(cmd);
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

    fn handle_gallery_command(&mut self, cmd: GalleryControl) {
        use ht_gallery::Rect;

        match cmd {
            GalleryControl::Start { id, mode, title, columns, spacing } => {
                tracing::info!("Gallery start: {} (mode: {:?})", id, mode);

                // Parse mode
                let gallery_mode = mode.as_ref()
                    .and_then(|m| match m.as_str() {
                        "grid" => Some(GalleryMode::Grid),
                        "masonry" => Some(GalleryMode::Masonry),
                        "filmstrip" => Some(GalleryMode::Filmstrip),
                        "comparison" => Some(GalleryMode::Comparison),
                        "deck" => Some(GalleryMode::Deck),
                        "file_explorer" => Some(GalleryMode::FileExplorer),
                        "auto" | _ => Some(GalleryMode::Auto),
                    })
                    .unwrap_or(GalleryMode::Auto);

                // Create gallery config
                let config = GalleryConfig {
                    mode: gallery_mode,
                    title,
                    columns,
                    spacing: spacing.unwrap_or(12.0),
                    padding: 16.0,
                };

                // Create gallery
                if let Err(e) = self.gallery.create_gallery(id.clone(), config) {
                    tracing::error!("Failed to create gallery: {}", e);
                } else {
                    self.current_gallery = Some(id);
                }
            }
            GalleryControl::End { id } => {
                tracing::info!("Gallery end: {}", id);

                // Finalize gallery layout
                let size = self.window.inner_size();
                let container = Rect::new(0.0, 0.0, size.width as f32, size.height as f32);

                if let Err(e) = self.gallery.finalize_gallery(&id, container) {
                    tracing::error!("Failed to finalize gallery: {}", e);
                }

                // Clear current gallery if it matches
                if self.current_gallery.as_ref() == Some(&id) {
                    self.current_gallery = None;
                }
            }
            GalleryControl::MediaItem { gallery_id, title, description, tags, author } => {
                tracing::debug!("Gallery media item: {} -> {:?}", gallery_id, title);

                // Store metadata for next Graphics token
                let mut metadata = CardMetadata::default();
                metadata.title = title;
                metadata.description = description;
                metadata.tags = tags;
                metadata.author = author;
                metadata.timestamp = Some(SystemTime::now());
                metadata.source = "claude".to_string(); // Could be detected from context

                self.pending_media_metadata = Some(metadata);

                // Ensure we're tracking this gallery
                if self.current_gallery.is_none() {
                    self.current_gallery = Some(gallery_id);
                }
            }
            GalleryControl::FileItem { gallery_id, path, title, description, language } => {
                tracing::debug!("Gallery file item: {} -> {}", gallery_id, path);

                // For file items, we'd need to read the file content
                // For now, just log it
                tracing::info!("File item not yet fully implemented: {}", path);
            }
            GalleryControl::CloseAll => {
                tracing::info!("Gallery close all");
                // Would need to add a close_all method to GalleryManager
                self.current_gallery = None;
            }
            GalleryControl::Status => {
                tracing::info!("Gallery status requested");
                // Could log current gallery state
            }
        }
    }
}
