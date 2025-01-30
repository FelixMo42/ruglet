use wgpu::util::DeviceExt;
use wgpu::*;
use winit::dpi::PhysicalSize;

use super::texture::{create_texture, Texture};

pub struct Binding {
    pub layout: BindGroupLayout,
    pub group: BindGroup,
}

const SCREEN_SIZE_BIND_GROUP: usize = 0;
const TEXTURE_BIND_GROUP: usize = 1;

pub type Bindings = [Binding; 2];

pub fn create_bindings(device: &Device, queue: &Queue) -> Bindings {
    return [
        create_screen_size_bindgroup(device),
        create_texture_bindgroup(device, queue),
    ];
}

pub fn update_screen_size_bindgroup(
    device: &Device,
    bindings: &mut Bindings,
    new_size: PhysicalSize<u32>,
) {
    let screen_size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[ScreenSizeUniform::new(new_size.width, new_size.height)]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    bindings[SCREEN_SIZE_BIND_GROUP].group = device.create_bind_group(&BindGroupDescriptor {
        layout: &bindings[SCREEN_SIZE_BIND_GROUP].layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: screen_size_buffer.as_entire_binding(),
        }],
        label: None,
    });
}

pub fn update_texture_bindgroup(device: &Device, bindings: &mut Bindings, texture: Texture) {
    bindings[TEXTURE_BIND_GROUP].group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("texture_bind_group"),
        layout: &bindings[TEXTURE_BIND_GROUP].layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&texture.view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(&texture.sampler),
            },
        ],
    })
}

pub fn create_texture_bindgroup(device: &Device, queue: &Queue) -> Binding {
    // Create blank 1x1px texture
    let texture = create_texture(device, queue, &[u8::max_value(); 4], (1, 1));

    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                // This should match the filterable field of the
                // corresponding Texture entry above.
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("texture_bind_group_layout"),
    });

    return Binding {
        group: device.create_bind_group(&BindGroupDescriptor {
            label: Some("texture_bind_group"),
            layout: &layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&texture.sampler),
                },
            ],
        }),
        layout,
    };
}

fn create_screen_size_bindgroup(device: &Device) -> Binding {
    let screen_size = ScreenSizeUniform {
        width: 1.,
        height: 1.,
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
