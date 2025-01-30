use super::{
    bindings::*,
    texture::{create_texture, TextureData},
    vertex::Vertex,
    Frame,
};
use wgpu::util::DeviceExt;
use wgpu::*;
use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer<'a> {
    // winit trackers
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: &'a Window,

    // core wgpu
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub render_pipeline: RenderPipeline,

    // wgpu utils
    pub bindings: Bindings,
}

impl<'a> Renderer<'a> {
    pub async fn new(window: &'a Window) -> Renderer<'a> {
        // Get the size of the window
        let size = window.inner_size();

        // Attach our seleves to the current window
        let instance = Instance::new(&InstanceDescriptor::default());
        let surface = instance.create_surface(window).unwrap();

        // What GPU do we have?
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // These are used to communicate with the GPU
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default(), None)
            .await
            .unwrap();

        // Configure the surface
        let config = create_surface_config(&surface, &adapter, size);

        // Get the bindgroups for the shader
        let bindings = create_bindings(&device, &queue);

        // Create the shader + it's render pipeline
        // If you edit the shader, you need to update this function
        let render_pipeline = create_render_pipeline(&device, &config, &bindings);

        return Self {
            // winit trackers
            window,
            size,

            // core wgpu
            surface,
            device,
            queue,
            config,
            render_pipeline,

            // wgpu utils
            bindings,
        };
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            // Reconfigure the surface with the new size
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            // Update the bindgroup for the texture
            update_screen_size_bindgroup(&self.device, &mut self.bindings, new_size);
        }
    }

    pub fn render(&mut self, frame: Frame) -> Result<(), SurfaceError> {
        // Get the current texture to render to
        let output = self.surface.get_current_texture()?;

        // Create the actual commands to send to the GPU
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        {
            // What are we doing this frame?
            let mut render_pass = create_render_pass(&output, &mut encoder);

            // Add the shader
            render_pass.set_pipeline(&self.render_pipeline);

            if frame.texture_changed {
                update_texture_bindgroup(
                    &self.device,
                    &mut self.bindings,
                    create_texture(&self.device, &self.queue, frame.texture),
                );
            }

            // Set the bind groups
            render_pass.set_bind_group(0, &self.bindings[0].group, &[]);
            render_pass.set_bind_group(1, &self.bindings[1].group, &[]);

            // Draw the vertices
            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&frame.vertices.as_slice()),
                    usage: BufferUsages::VERTEX,
                });

            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.draw(0..(frame.vertices.len() as u32), 0..1);
        }

        // Send the commands to the GPU and show the output
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        return Ok(());
    }
}

fn create_render_pass<'a>(
    output: &'a SurfaceTexture,
    encoder: &'a mut CommandEncoder,
) -> RenderPass<'a> {
    let view = output
        .texture
        .create_view(&TextureViewDescriptor::default());

    return encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: Operations::default(),
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
    });
}

fn create_surface_config(
    surface: &Surface,
    adapter: &Adapter,
    size: PhysicalSize<u32>,
) -> SurfaceConfiguration {
    let surface_caps = surface.get_capabilities(&adapter);

    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);

    return SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
}

fn create_render_pipeline(
    device: &Device,
    config: &SurfaceConfiguration,
    bindings: &Bindings,
) -> RenderPipeline {
    // Load the shader
    let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

    // What needs to be passed in to the shader?
    let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &bindings.iter().map(|b| &b.layout).collect::<Vec<_>>(),
        push_constant_ranges: &[],
    });

    // Create the render pipeline for the shader
    return device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::desc()],
            compilation_options: PipelineCompilationOptions::default(),
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(ColorTargetState {
                format: config.format,
                blend: Some(BlendState::ALPHA_BLENDING),
                write_mask: ColorWrites::ALL,
            })],
            compilation_options: PipelineCompilationOptions::default(),
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
        cache: None,
    });
}
