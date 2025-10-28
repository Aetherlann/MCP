pub mod text;
pub mod glyph_cache;
pub mod media;

use wgpu;
use anyhow::Result;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    text_renderer: text::TextRenderer,
    media_renderer: media::MediaRenderer,
}

impl Renderer {
    pub async fn new(window: &winit::window::Window) -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find suitable adapter"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Hyper Terminal Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await?;

        let size = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let text_renderer = text::TextRenderer::new(&device, &queue, surface_config.format)?;
        let media_renderer = media::MediaRenderer::new(&device, &queue, surface_config.format)?;

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            text_renderer,
            media_renderer,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn cell_size(&self) -> (f32, f32) {
        self.text_renderer.cell_size()
    }

    pub fn render(&mut self, grid: &ht_vt::Grid, media_manager: &ht_media::MediaManager) -> Result<()> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.06,
                            b: 0.1,
                            a: 0.9,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Render text first
            self.text_renderer.render(
                &mut render_pass,
                grid,
                &self.queue,
                self.surface_config.width as f32,
                self.surface_config.height as f32,
            )?;

            // Render media on top
            let (cell_width, cell_height) = self.text_renderer.cell_size();
            let media_surfaces: Vec<_> = media_manager.surfaces().iter().collect();

            self.media_renderer.render(
                &mut render_pass,
                &self.queue,
                self.surface_config.width as f32,
                self.surface_config.height as f32,
                &media_surfaces,
                cell_width,
                cell_height,
            )?;
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn upload_media_texture(&mut self, id: u32, rgba_data: &[u8], width: u32, height: u32) {
        self.media_renderer.upload_texture(&self.device, &self.queue, id, rgba_data, width, height);
    }
}
