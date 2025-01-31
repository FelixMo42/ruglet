mod layout;
mod ruglet;

use std::time::Instant;

use layout::*;
use ruglet::*;

struct MyApp<'a> {
    tree: Tree<'a>,
    root: usize,
    font: FontAtlas,

    // State
    scroll: f32,
}

impl<'a> MyApp<'a> {
    fn new(text: &'a str) -> Self {
        let font = FontAtlas::new();
        let mut tree = Tree::new();

        let paragraphs = text
            .lines()
            .map(|line| tree.add(NodeKind::Text(line), vec![]))
            .collect();

        let pad = tree.add(NodeKind::Pad(50.0), paragraphs);

        let root = tree.add(NodeKind::Scroll(0.0), vec![pad]);

        return MyApp {
            tree,
            root,
            font,

            // State
            scroll: 0.,
        };
    }
}

impl<'a> Application for MyApp<'a> {
    fn on_mouse_scroll(&mut self, _dx: f32, dy: f32) {
        self.scroll -= dy;
        // println!("{} + {}", self.scroll, dy);

        self.tree.update(self.root, NodeKind::Scroll(self.scroll));
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let now = Instant::now();

        // Render the dom
        self.tree.build(self.root, frame, &mut self.font);

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}

fn main() {
    let text = include_str!("../res/test.txt");

    let mut app = MyApp::new(text);

    if let Err(e) = pollster::block_on(app.run()) {
        eprintln!("Render: {:?}", e);
    }
}
