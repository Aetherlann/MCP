// GPU renderer for gallery

use crate::types::*;
use crate::card::MediaCard;
use crate::theme::GalleryTheme;
use anyhow::Result;

/// Gallery renderer interface
pub struct GalleryRenderer {
    theme: GalleryTheme,
    card_pipeline: Option<CardRenderPipeline>,
}

impl GalleryRenderer {
    pub fn new(theme: GalleryTheme) -> Self {
        Self {
            theme,
            card_pipeline: None,
        }
    }

    /// Initialize GPU resources
    pub fn init(&mut self, device: &wgpu::Device, format: wgpu::TextureFormat) -> Result<()> {
        // Create card rendering pipeline
        self.card_pipeline = Some(CardRenderPipeline::new(device, format)?);
        Ok(())
    }

    /// Render all visible cards
    pub fn render_cards(
        &self,
        cards: &[&MediaCard],
        render_pass: &mut wgpu::RenderPass,
        queue: &wgpu::Queue,
        viewport: Rect,
    ) -> Result<()> {
        if let Some(pipeline) = &self.card_pipeline {
            for card in cards {
                // Cull cards outside viewport
                if !viewport.intersects(&card.layout.bounds) {
                    continue;
                }

                pipeline.render_card(card, render_pass, queue, &self.theme)?;
            }
        }

        Ok(())
    }

    /// Update theme
    pub fn set_theme(&mut self, theme: GalleryTheme) {
        self.theme = theme;
    }
}

/// Card rendering pipeline
struct CardRenderPipeline {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl CardRenderPipeline {
    fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Result<Self> {
        // Shader source
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Card Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/card.wgsl").into()),
        });

        // Bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Card Bind Group Layout"),
            entries: &[
                // Uniforms (MVP, colors, etc.)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // Pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Card Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Card Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[CardVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Create buffers
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Card Vertex Buffer"),
            size: 1024 * std::mem::size_of::<CardVertex>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Card Index Buffer"),
            size: 2048 * std::mem::size_of::<u32>() as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Card Uniform Buffer"),
            size: std::mem::size_of::<CardUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            bind_group_layout,
        })
    }

    fn render_card(
        &self,
        card: &MediaCard,
        render_pass: &mut wgpu::RenderPass,
        queue: &wgpu::Queue,
        theme: &GalleryTheme,
    ) -> Result<()> {
        // Build card geometry
        let vertices = self.build_card_vertices(card, theme);
        let indices = vec![0, 1, 2, 2, 3, 0]; // Two triangles for quad

        // Upload to GPU
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));

        // Update uniforms
        let uniforms = CardUniforms {
            elevation: card.current_elevation(),
            scale: card.current_scale(),
            opacity: 1.0,
            padding: 0.0,
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

        // Render
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..6, 0, 0..1);

        Ok(())
    }

    fn build_card_vertices(&self, card: &MediaCard, theme: &GalleryTheme) -> Vec<CardVertex> {
        let bounds = card.layout.bounds;
        let border_radius = card.layout.border_radius;

        // Determine card color based on state
        let background = match card.state {
            CardState::Hovered => {
                let base = theme.card_background;
                let overlay = theme.card_hover_overlay;
                // Blend colors
                Color {
                    r: base.r + overlay.r * overlay.a,
                    g: base.g + overlay.g * overlay.a,
                    b: base.b + overlay.b * overlay.a,
                    a: base.a,
                }
            }
            CardState::Selected => theme.card_background,
            _ => theme.card_background,
        };

        let border = if card.selected {
            theme.card_selected_border
        } else {
            theme.card_border
        };

        // Create quad vertices
        vec![
            CardVertex {
                position: [bounds.x, bounds.y],
                tex_coords: [0.0, 0.0],
                color: background.as_array(),
                border_color: border.as_array(),
                border_radius,
                elevation: card.current_elevation(),
            },
            CardVertex {
                position: [bounds.x + bounds.width, bounds.y],
                tex_coords: [1.0, 0.0],
                color: background.as_array(),
                border_color: border.as_array(),
                border_radius,
                elevation: card.current_elevation(),
            },
            CardVertex {
                position: [bounds.x + bounds.width, bounds.y + bounds.height],
                tex_coords: [1.0, 1.0],
                color: background.as_array(),
                border_color: border.as_array(),
                border_radius,
                elevation: card.current_elevation(),
            },
            CardVertex {
                position: [bounds.x, bounds.y + bounds.height],
                tex_coords: [0.0, 1.0],
                color: background.as_array(),
                border_color: border.as_array(),
                border_radius,
                elevation: card.current_elevation(),
            },
        ]
    }
}

/// Card vertex data
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CardVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: [f32; 4],
    border_color: [f32; 4],
    border_radius: f32,
    elevation: f32,
}

impl CardVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CardVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Tex coords
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // Border color
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 4]>() * 2) as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // Border radius
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 4]>() * 3) as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
                // Elevation
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 4]>() * 3 + std::mem::size_of::<f32>()) as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

/// Card uniforms
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CardUniforms {
    elevation: f32,
    scale: f32,
    opacity: f32,
    padding: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_vertex_size() {
        // Ensure proper alignment for GPU
        assert_eq!(std::mem::size_of::<CardVertex>() % 16, 0);
    }

    #[test]
    fn test_card_uniforms_size() {
        assert_eq!(std::mem::size_of::<CardUniforms>(), 16);
    }
}
