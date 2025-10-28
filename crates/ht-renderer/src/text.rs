use wgpu;
use anyhow::Result;
use ab_glyph::{FontArc, PxScale, ScaleFont};
use std::collections::HashMap;

const ATLAS_SIZE: u32 = 1024;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct GlyphInfo {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    bearing_x: f32,
    bearing_y: f32,
    advance: f32,
}

pub struct TextRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    glyph_cache: HashMap<char, GlyphInfo>,
    atlas_texture: wgpu::Texture,
    atlas_data: Vec<u8>,
    atlas_x: u32,
    atlas_y: u32,
    atlas_row_height: u32,
    font: FontArc,
    font_size: f32,
    cell_width: f32,
    cell_height: f32,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Result<Self> {
        // Load embedded font
        let font = FontArc::try_from_slice(include_bytes!("../assets/DejaVuSansMono.ttf"))
            .expect("Failed to load embedded font");

        let font_size = 16.0;
        let scaled_font = font.as_scaled(PxScale::from(font_size));

        // Calculate cell dimensions
        let glyph = scaled_font.scaled_glyph('M');
        let cell_width = scaled_font.h_advance(glyph.id);
        let cell_height = scaled_font.height();

        // Create atlas texture
        let atlas_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Glyph Atlas"),
            size: wgpu::Extent3d {
                width: ATLAS_SIZE,
                height: ATLAS_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let atlas_view = atlas_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create uniform buffer for screen size
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Screen Size Uniform"),
            size: 16, // vec2<f32> with padding
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Text Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&atlas_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Text Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("text.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Text Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
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
            cache: None,
        });

        // Create vertex and index buffers (will be updated per frame)
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: 1024 * 1024, // 1MB buffer
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: 512 * 1024, // 512KB buffer
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            bind_group,
            uniform_buffer,
            glyph_cache: HashMap::new(),
            atlas_texture,
            atlas_data: vec![0; (ATLAS_SIZE * ATLAS_SIZE) as usize],
            atlas_x: 0,
            atlas_y: 0,
            atlas_row_height: 0,
            font,
            font_size,
            cell_width,
            cell_height,
        })
    }

    fn cache_glyph(&mut self, ch: char, queue: &wgpu::Queue) -> GlyphInfo {
        if let Some(&info) = self.glyph_cache.get(&ch) {
            return info;
        }

        let scaled_font = self.font.as_scaled(PxScale::from(self.font_size));
        let glyph = scaled_font.scaled_glyph(ch);

        if let Some(outlined) = self.font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            let width = bounds.width().ceil() as u32;
            let height = bounds.height().ceil() as u32;

            // Check if we need to move to next row
            if self.atlas_x + width > ATLAS_SIZE {
                self.atlas_x = 0;
                self.atlas_y += self.atlas_row_height;
                self.atlas_row_height = 0;
            }

            // Rasterize glyph
            let mut glyph_data = vec![0u8; (width * height) as usize];
            outlined.draw(|x, y, coverage| {
                if x < width && y < height {
                    glyph_data[(y * width + x) as usize] = (coverage * 255.0) as u8;
                }
            });

            // Upload to atlas
            let x = self.atlas_x;
            let y = self.atlas_y;

            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &self.atlas_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x, y, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                &glyph_data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(width),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );

            let info = GlyphInfo {
                x,
                y,
                width,
                height,
                bearing_x: bounds.min.x,
                bearing_y: -bounds.min.y,
                advance: scaled_font.h_advance(glyph.id),
            };

            self.atlas_x += width + 2; // 2px padding
            self.atlas_row_height = self.atlas_row_height.max(height + 2);

            self.glyph_cache.insert(ch, info);
            info
        } else {
            // Return empty glyph info for characters without outline
            GlyphInfo {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
                bearing_x: 0.0,
                bearing_y: 0.0,
                advance: self.cell_width,
            }
        }
    }

    pub fn render(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        grid: &ht_vt::Grid,
        queue: &wgpu::Queue,
        screen_width: f32,
        screen_height: f32,
    ) -> Result<()> {
        // Update screen size uniform
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[screen_width, screen_height]));

        let (cols, rows) = grid.size();
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                if let Some(cell) = grid.get_cell(col, row) {
                    if cell.ch == ' ' {
                        continue;
                    }

                    let glyph_info = self.cache_glyph(cell.ch, queue);

                    let x = col as f32 * self.cell_width;
                    let y = row as f32 * self.cell_height;

                    let glyph_x = x + glyph_info.bearing_x;
                    let glyph_y = y + glyph_info.bearing_y;
                    let glyph_w = glyph_info.width as f32;
                    let glyph_h = glyph_info.height as f32;

                    // Texture coordinates
                    let tx0 = glyph_info.x as f32 / ATLAS_SIZE as f32;
                    let ty0 = glyph_info.y as f32 / ATLAS_SIZE as f32;
                    let tx1 = (glyph_info.x + glyph_info.width) as f32 / ATLAS_SIZE as f32;
                    let ty1 = (glyph_info.y + glyph_info.height) as f32 / ATLAS_SIZE as f32;

                    // Color
                    let color_rgba = cell.attrs.foreground.to_rgba();
                    let color = [
                        color_rgba[0] as f32 / 255.0,
                        color_rgba[1] as f32 / 255.0,
                        color_rgba[2] as f32 / 255.0,
                        color_rgba[3] as f32 / 255.0,
                    ];

                    let base_index = vertices.len() as u32;

                    // Create quad
                    vertices.push(Vertex {
                        position: [glyph_x, glyph_y],
                        tex_coords: [tx0, ty0],
                        color,
                    });
                    vertices.push(Vertex {
                        position: [glyph_x + glyph_w, glyph_y],
                        tex_coords: [tx1, ty0],
                        color,
                    });
                    vertices.push(Vertex {
                        position: [glyph_x + glyph_w, glyph_y + glyph_h],
                        tex_coords: [tx1, ty1],
                        color,
                    });
                    vertices.push(Vertex {
                        position: [glyph_x, glyph_y + glyph_h],
                        tex_coords: [tx0, ty1],
                        color,
                    });

                    // Two triangles per quad
                    indices.extend_from_slice(&[
                        base_index,
                        base_index + 1,
                        base_index + 2,
                        base_index,
                        base_index + 2,
                        base_index + 3,
                    ]);
                }
            }
        }

        if vertices.is_empty() {
            return Ok(());
        }

        // Upload vertex and index data
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));

        // Render
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);

        Ok(())
    }

    pub fn cell_size(&self) -> (f32, f32) {
        (self.cell_width, self.cell_height)
    }
}
