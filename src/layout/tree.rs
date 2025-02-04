use crate::ruglet::*;

use super::{FontAtlas, TextBook};

const PX: f32 = 60.;
const LH: f32 = 80.;
const WS: f32 = 30.;

//////////
// NODE //
//////////

#[derive(Debug, Clone)]
pub enum NodeKind {
    None,
    Text(usize),
    Pad(f32),
    Scroll(f32),
    Clickable(usize),
}

#[derive(Clone)]
struct Node {
    kind: NodeKind,
    child: usize,
    next: usize,
    area: Area,
}

//////////
// TREE //
//////////

pub struct Tree {
    nodes: Vec<Node>,
}

// Build function
impl Tree {
    pub fn new() -> Self {
        return Tree { nodes: vec![] };
    }

    pub fn add(&mut self, kind: NodeKind, children: Vec<usize>) -> usize {
        let id = self.nodes.len();

        let child_id = if children.len() > 0 {
            // Each child should form a list
            for i in 0..children.len() - 1 {
                self.nodes[children[i]].next = children[i + 1];
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

    pub fn get(&self, id: usize) -> NodeKind {
        return self.nodes[id].kind.clone();
    }

    pub fn update(&mut self, node: usize, kind: NodeKind) {
        self.nodes[node].kind = kind
    }

    pub fn replace(&mut self, a: usize, b: usize) {
        self.delete(a);
        self.nodes[a] = self.nodes[b].clone();
    }

    fn delete(&mut self, node: usize) {
        self.nodes[node].kind = NodeKind::None;

        if self.nodes[node].child != usize::MAX {
            self.delete(self.nodes[node].child);
        }

        if self.nodes[node].next != usize::MAX {
            self.delete(self.nodes[node].next);
        }
    }
}

// Event functions
impl Tree {
    pub fn click(&self, node: usize, mouse: Vec2) -> Option<usize> {
        if !mouse.inside(self.nodes[node].area) {
            return None;
        }

        match self.nodes[node].kind {
            NodeKind::Clickable(event_id) => {
                return Some(event_id);
            }
            _ => {
                let mut child = self.nodes[node].child;

                while child != usize::MAX {
                    let res = self.click(child, mouse);

                    if res.is_some() {
                        return res;
                    }

                    child = self.nodes[child].next;
                }

                return None;
            }
        };
    }
}

// Debug functions
impl Tree {
    pub fn print(&self, node: usize, tab: usize) {
        println!("{}{:?}", "| ".repeat(tab), self.nodes[node].kind);

        let mut child = self.nodes[node].child;
        while child != usize::MAX {
            child = self.nodes[child].next;
        }
    }
}

// Render functions
impl Tree {
    pub fn build(
        &mut self,
        root: usize,
        frame: &mut Frame,
        atlas: &mut FontAtlas,
        text: &TextBook,
    ) {
        // layout
        self.layout(root, frame.area, atlas, text);
        self.nodes[root].area = frame.area;

        // Update the texture if new glyphs have been added to the font atlas
        if atlas.texture_changed() {
            frame.set_texture(atlas.build_texture());
        }

        // render
        self.render(frame, atlas, text);
    }

    fn render(&self, frame: &mut Frame, atlas: &mut FontAtlas, text: &TextBook) {
        for node in &self.nodes {
            if !frame.area.contains(node.area) || frame.area.is_zero() {
                continue;
            }

            match node.kind {
                NodeKind::Text(tid) => {
                    let mut x = node.area.0.x;
                    let mut y = node.area.0.y;

                    for word in text.get(tid).split_whitespace() {
                        let w: f32 = word.chars().map(|c| atlas.size(c, PX).x).sum();
                        if x + w + WS > node.area.1.x {
                            x = node.area.0.x;
                            y += LH;
                        }

                        for c in word.chars() {
                            let texture = atlas.get(c, PX);
                            let metrics = atlas.metrics(texture);

                            let gx = x + metrics.xmin as f32;
                            let gy = y + LH - metrics.ymin as f32 - metrics.height as f32;
                            frame.quad(
                                Area(
                                    Vec2::new(gx, gy),
                                    Vec2::new(
                                        gx + metrics.width as f32,
                                        gy + metrics.height as f32,
                                    ),
                                ),
                                atlas.texture_area(texture),
                                [1., 1., 1.],
                            );

                            x += metrics.advance_width;
                        }

                        x += WS;
                    }
                }
                _ => {}
            }
        }
    }

    fn layout(&mut self, node: usize, area: Area, atlas: &mut FontAtlas, text: &TextBook) -> Vec2 {
        match self.nodes[node].kind {
            NodeKind::None => {
                unreachable!()
            }
            NodeKind::Clickable(_) => {
                let child = self.nodes[node].child;
                let size = self.layout(child, area, atlas, text);

                // Set area
                self.nodes[child].area.0.x = area.0.x;
                self.nodes[child].area.0.y = area.0.y;
                self.nodes[child].area.1.x = area.0.x + size.x;
                self.nodes[child].area.1.y = area.0.y + size.y;

                return size;
            }
            NodeKind::Pad(padding) => {
                let mut child_area = Area(
                    Vec2::new(area.0.x + padding, area.0.y + padding),
                    Vec2::new(area.1.x - padding, f32::MAX),
                );

                let mut child = self.nodes[node].child;
                while child != usize::MAX {
                    let size = self.layout(child, child_area, atlas, text);

                    // Set area
                    self.nodes[child].area.0.x = child_area.0.x;
                    self.nodes[child].area.0.y = child_area.0.y;
                    self.nodes[child].area.1.x = child_area.1.x;
                    self.nodes[child].area.1.y = child_area.0.y + size.y;

                    // Move on, Mr. y
                    child_area.0.y += size.y + LH;

                    if child_area.0.y > area.1.y {
                        break;
                    }

                    // Get next child
                    child = self.nodes[child].next;
                }

                // Scroll should just take up the whole area
                return Vec2::new(area.w(), child_area.0.y + padding * 2.);
            }
            NodeKind::Scroll(scroll) => {
                let child_area = Area(
                    Vec2::new(area.0.x, area.0.y - scroll),
                    Vec2::new(area.1.x, f32::MAX),
                );

                // Layout all them children
                let mut child = self.nodes[node].child;
                let mut y = child_area.0.y;
                while child != usize::MAX {
                    let size = self.layout(child, child_area, atlas, text);

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
                return area.size();
            }
            NodeKind::Text(tid) => {
                // Just remeber the cache
                if self.nodes[node].area.w() == area.w() {
                    return self.nodes[node].area.size();
                }

                let mut h = LH;

                let mut row = 0.;
                for word in text.get(tid).split_whitespace() {
                    let w = word.chars().map(|c| atlas.size(c, PX).x).sum();

                    if area.0.x + row + w + WS > area.1.x {
                        h += LH;
                        row = w;
                    } else {
                        row += w + WS;
                    }
                }

                return Vec2::new(area.w(), h);
            }
        }
    }
}
