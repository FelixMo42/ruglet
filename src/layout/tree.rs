use crate::ruglet::*;

use super::FontAtlas;

const PX: f32 = 60.;
const LH: f32 = 80.;

//////////
// NODE //
//////////

pub enum NodeKind<'a> {
    Text(&'a str),
    Pad(f32),
    Scroll,
}

struct Node<'a> {
    kind: NodeKind<'a>,
    child: usize,
    next: usize,
    area: Area,
}

//////////
// TREE //
//////////

pub struct Tree<'a> {
    nodes: Vec<Node<'a>>,
}

impl<'a> Tree<'a> {
    pub fn new() -> Self {
        return Tree { nodes: vec![] };
    }

    pub fn add(&mut self, kind: NodeKind<'a>, children: Vec<usize>) -> usize {
        let id = self.nodes.len();

        let child_id = if children.len() > 0 {
            // Each child should form a list
            for i in 0..children.len() - 1 {
                self.nodes[children[i]].next = children[children[i + 1]];
            }

            // Are child id should point towards the first child
            children[0]
        } else {
            usize::MAX
        };

        self.nodes.push(Node {
            kind,
            child: child_id,
            next: usize::MAX,
            area: Area::zero(),
        });

        return id;
    }
}

impl<'a> Tree<'a> {
    pub fn build(&mut self, root: usize, frame: &mut Frame, atlas: &mut FontAtlas) {
        // layout
        self.layout(root, frame.area, atlas);
        self.nodes[root].area = frame.area;

        // Update the texture if new glyphs have been added to the font atlas
        if atlas.texture_changed() {
            frame.set_texture(atlas.build_texture());
        }

        // render
        self.render(frame, atlas);
    }

    fn render(&self, frame: &mut Frame, atlas: &mut FontAtlas) {
        for node in &self.nodes {
            match node.kind {
                NodeKind::Text(text) => {
                    let mut x = node.area.0.x;
                    let mut y = node.area.0.y;

                    for c in text.chars() {
                        let texture = atlas.get(c, PX);
                        let metrics = atlas.metrics(texture);

                        if x + metrics.advance_width > node.area.1.x {
                            x = node.area.0.x;
                            y += LH;
                        }

                        let gx = x + metrics.xmin as f32;
                        let gy = y + LH - metrics.ymin as f32 - metrics.height as f32;
                        frame.quad(
                            Area(
                                Vec2::new(gx, gy),
                                Vec2::new(gx + metrics.width as f32, gy + metrics.height as f32),
                            ),
                            atlas.texture_area(texture),
                            [1., 1., 1.],
                        );

                        x += metrics.advance_width;
                    }
                }
                _ => {}
            }
        }
    }

    fn layout(&mut self, node: usize, area: Area, atlas: &mut FontAtlas) -> Vec2 {
        match self.nodes[node].kind {
            NodeKind::Pad(padding) => {
                let child_area = Area(
                    Vec2::new(area.0.x + padding, area.0.y + padding),
                    Vec2::new(area.1.x - padding, f32::MAX),
                );

                let mut child = self.nodes[node].child;
                let mut y = child_area.0.y;
                while child != usize::MAX {
                    let size = self.layout(child, child_area, atlas);

                    // Set area
                    self.nodes[child].area.0.x = child_area.0.x;
                    self.nodes[child].area.0.y = y;
                    self.nodes[child].area.1.x = child_area.1.x;
                    self.nodes[child].area.1.y = y + size.y;

                    // Move on, Mr. y
                    y += size.y + LH;

                    // Get next child
                    child = self.nodes[child].next;
                }

                // Scroll should just take up the whole area
                return Vec2::new(area.w(), y + padding * 2.);
            }
            NodeKind::Scroll => {
                // Layout all them children
                let mut child = self.nodes[node].child;
                let mut y = area.0.y;
                while child != usize::MAX {
                    let size = self.layout(child, area, atlas);

                    // Set area
                    self.nodes[child].area.0.x = area.0.x;
                    self.nodes[child].area.0.y = y;
                    self.nodes[child].area.1.x = area.1.x;
                    self.nodes[child].area.1.y = y + size.y;

                    // Move on, Mr. y
                    y += size.y + LH;

                    // Get next child
                    child = self.nodes[child].next;
                }

                // Scroll should just take up the whole area
                return area.size();
            }
            NodeKind::Text(text) => {
                let w = area.w();
                let mut h = LH;

                let mut row = 0.;
                for c in text.chars() {
                    let size = atlas.size(c, PX);
                    if area.0.x + row + size.x > area.1.x {
                        h += LH;
                        row = size.x;
                    } else {
                        row += size.x;
                    }
                }

                return Vec2::new(w, h);
            }
        }
    }
}
