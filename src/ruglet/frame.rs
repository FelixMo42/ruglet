use winit::dpi::PhysicalSize;

use crate::ruglet::*;

pub struct Frame {
    pub area: Area,
    pub vertices: Vec<Vertex>,

    pub texture_bytes: Vec<u8>,
    pub texture_dimensions: (u32, u32),
    pub texture_changed: bool,
}

impl Frame {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        return Frame {
            // List of vertices
            vertices: vec![],

            // Size of the screen
            area: Area(
                Vec2::zero(),
                Vec2::new(size.width as f32, size.height as f32),
            ),

            // Default to 1x1px white texture
            texture_bytes: vec![u8::MAX; 4],
            texture_dimensions: (1, 1),
            texture_changed: false,
        };
    }

    pub fn quad(&mut self, area: Area, color: [f32; 3]) {
        self.vertices.push(Vertex {
            position: [area.0.x, area.0.y, 0.0],
            tex_coords: [0., 0.],
            color,
        });
        self.vertices.push(Vertex {
            position: [area.0.x, area.1.y, 0.0],
            tex_coords: [0., 1.],
            color,
        });
        self.vertices.push(Vertex {
            position: [area.1.x, area.1.y, 0.0],
            tex_coords: [1., 1.],
            color,
        });

        self.vertices.push(Vertex {
            position: [area.1.x, area.1.y, 0.0],
            tex_coords: [1., 1.],
            color,
        });
        self.vertices.push(Vertex {
            position: [area.1.x, area.0.y, 0.0],
            tex_coords: [1., 0.],
            color,
        });
        self.vertices.push(Vertex {
            position: [area.0.x, area.0.y, 0.0],
            tex_coords: [0., 0.],
            color,
        });
    }

    pub fn set_texture(&mut self, bytes: Vec<u8>, dimensions: (u32, u32)) {
        self.texture_bytes = bytes;
        self.texture_dimensions = dimensions;
        self.texture_changed = true;
    }
}
