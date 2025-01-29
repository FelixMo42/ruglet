use winit::dpi::PhysicalSize;

use crate::ruglet::*;

pub struct Frame {
    pub area: Area,
    pub vertices: Vec<Vertex>,
}

impl Frame {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        return Frame {
            vertices: vec![],
            area: Area(
                Vec2::zero(),
                Vec2::new(size.width as f32, size.height as f32),
            ),
        };
    }

    pub fn quad(&mut self, area: Area, color: [f32; 3]) {
        let tex_coords = [0., 0.];
        self.vertices.push(Vertex {
            position: [area.0.x, area.0.y, 0.0],
            tex_coords,
            color,
        });
        self.vertices.push(Vertex {
            position: [area.0.x, area.1.y, 0.0],
            tex_coords,
            color,
        });
        self.vertices.push(Vertex {
            position: [area.1.x, area.1.y, 0.0],
            tex_coords,
            color,
        });

        self.vertices.push(Vertex {
            position: [area.1.x, area.1.y, 0.0],
            tex_coords,
            color,
        });
        self.vertices.push(Vertex {
            position: [area.1.x, area.0.y, 0.0],
            tex_coords,
            color,
        });
        self.vertices.push(Vertex {
            position: [area.0.x, area.0.y, 0.0],
            tex_coords,
            color,
        });
    }
}
