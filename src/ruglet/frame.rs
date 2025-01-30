use winit::dpi::PhysicalSize;

use crate::ruglet::*;

use super::texture::TextureData;

pub struct Frame {
    pub area: Area,
    pub vertices: Vec<Vertex>,

    pub texture: TextureData,
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
            texture: TextureData::blank(),
            texture_changed: false,
        };
    }

    pub fn quad(&mut self, area: Area, tex: Area, color: [f32; 3]) {
        self.vertices.push(Vertex {
            position: [area.0.x, area.0.y, 0.0],
            tex_coords: [tex.0.x, tex.0.y],
            color,
        });
        self.vertices.push(Vertex {
            position: [area.0.x, area.1.y, 0.0],
            tex_coords: [tex.0.x, tex.1.y],
            color,
        });
        self.vertices.push(Vertex {
            position: [area.1.x, area.1.y, 0.0],
            tex_coords: [tex.1.x, tex.1.y],
            color,
        });

        self.vertices.push(Vertex {
            position: [area.1.x, area.1.y, 0.0],
            tex_coords: [tex.1.x, tex.1.y],
            color,
        });
        self.vertices.push(Vertex {
            position: [area.1.x, area.0.y, 0.0],
            tex_coords: [tex.1.x, tex.0.y],
            color,
        });
        self.vertices.push(Vertex {
            position: [area.0.x, area.0.y, 0.0],
            tex_coords: [tex.0.x, tex.0.y],
            color,
        });
    }

    pub fn set_texture(&mut self, texture: TextureData) {
        self.texture = texture;
        self.texture_changed = true;
    }
}
