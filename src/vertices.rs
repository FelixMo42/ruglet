use std::ops::RangeBounds;

use wgpu::util::DeviceExt;
use wgpu::*;

///
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        return VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        };
    }
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

///
pub struct Vertices {
    length: u32,
    vertex_buffer: Buffer,
}

impl Vertices {
    pub fn new(device: &Device, vertices: &[Vertex]) -> Vertices {
        return Vertices {
            length: vertices.len() as u32,
            vertex_buffer: device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: BufferUsages::VERTEX,
                }
            ) 
        } 
    }
}

impl Vertices {
    pub fn len(&self) -> u32 {
        return self.length;
    }

    pub fn slice<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferSlice {
        return self.vertex_buffer.slice(bounds);
    }
}