use std::num::NonZeroU32;

pub use crate::ruglet::texture;
pub use crate::ruglet::vertices::Vertex;

use winit::dpi::PhysicalSize;
use winit::window::Window;

use wgpu::util::DeviceExt;
use wgpu::*;

pub struct State {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: RenderPipeline,
    pub bind_groups: [Binding; 2],
}

pub struct Binding {
    layout: BindGroupLayout,
    group: BindGroup,
}

fn create_texture_bindgroup(device: &Device, queue: &Queue) -> Binding {
    let bytes = include_bytes!("../tree.png");
    let texture = texture::Texture::from_bytes(device, queue, bytes, "tree.png").unwrap();

    let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("texture_bind_group_layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    multisampled: false,
                    view_dimension: TextureViewDimension::D2,
                    sample_type: TextureSampleType::Float { filterable: true },
                },
                count: NonZeroU32::new(1),
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                // This should match the filterable field of the
                // corresponding Texture entry above.
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: NonZeroU32::new(1),
            },
        ],
    });

    return Binding {
        group: device.create_bind_group(&BindGroupDescriptor {
            label: Some("texture_bind_group"),
            layout: &layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureViewArray(&[&texture.view]),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::SamplerArray(&[&texture.sampler]),
                },
            ],
        }),
        layout,
    };
}

fn create_screen_size_bindgroup(device: &Device, size: PhysicalSize<u32>) -> Binding {
    let screen_size = ScreenSizeUniform {
        width: size.width as f32 / 2.0,
        height: size.height as f32 / 2.0,
    };

    let screen_size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Screen size Buffer"),
        contents: bytemuck::cast_slice(&[screen_size]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let screen_size_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("screen_size_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let screen_size_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &screen_size_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: screen_size_buffer.as_entire_binding(),
        }],
        label: Some("screen_size_bind_group"),
    });

    return Binding {
        layout: screen_size_bind_group_layout,
        group: screen_size_bind_group,
    };
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ScreenSizeUniform {
    width: f32,
    height: f32,
}

impl ScreenSizeUniform {
    fn new(width: u32, height: u32) -> ScreenSizeUniform {
        return ScreenSizeUniform {
            width: width as f32 / 2.0,
            height: height as f32 / 2.0,
        };
    }
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        let instance = Instance::new(InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(window).expect("ERR; surface?!?") };

        //
        let adapter = instance
            .enumerate_adapters(Backends::all())
            .filter(|adapter| adapter.is_surface_supported(&surface))
            .next()
            .unwrap();

        // WebGL doesn't support all of wgpu's features, so if
        // we're building for the web we'll have to disable some.
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: Features::TEXTURE_BINDING_ARRAY,
                    limits: if cfg!(target_arch = "wasm32") {
                        Limits::downlevel_webgl2_defaults()
                    } else {
                        Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            // .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            alpha_mode: CompositeAlphaMode::Opaque,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

        let bind_groups = [
            create_screen_size_bindgroup(&device, size),
            create_texture_bindgroup(&device, &queue),
        ];

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_groups[0].layout, &bind_groups[1].layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        return State {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            bind_groups,
        };
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            let screen_size_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Screen size Buffer"),
                        contents: bytemuck::cast_slice(&[ScreenSizeUniform::new(
                            new_size.width,
                            new_size.height,
                        )]),
                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    });

            self.bind_groups[0].group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.bind_groups[0].layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: screen_size_buffer.as_entire_binding(),
                }],
                label: Some("screen_size_bind_group"),
            });
        }
    }

    pub fn render(&mut self, vertices: &[Vertex]) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: BufferUsages::VERTEX,
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_groups[0].group, &[]);
            render_pass.set_bind_group(1, &self.bind_groups[1].group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.draw(0..vertices.len() as u32, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
