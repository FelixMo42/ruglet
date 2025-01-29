use wgpu::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [VertexAttribute; 3] = vertex_attr_array![
        // The vertex position
        0 => Float32x3,

        // The face color
        1 => Float32x3,

        // The texture coordinates
        2 => Float32x2,
    ];

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
