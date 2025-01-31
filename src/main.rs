mod layout;
mod ruglet;

use std::{fs, time::Instant};

use layout::*;
use ruglet::*;
use winit::event::MouseButton;

struct MyApp<'a> {
    tree: Tree<'a>,
    root: usize,
    font: FontAtlas,

    // State
    scroll: f32,
}

impl<'a> MyApp<'a> {
    fn new(options: &'a [String]) -> Self {
        let font = FontAtlas::new();
        let mut tree = Tree::new();

        // let paragraphs = text
        //     .lines()
        //     .map(|line| tree.add(NodeKind::Text(line), vec![]))
        //     .collect();

        // let pad = tree.add(NodeKind::Pad(50.0), paragraphs);

        // let root = tree.add(NodeKind::Scroll(0.0), vec![pad]);

        let paragraphs = options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let text = tree.add(NodeKind::Text(&option), vec![]);
                return tree.add(NodeKind::Clickable(i), vec![text]);
            })
            .collect();

        let pad = tree.add(NodeKind::Pad(50.0), paragraphs);

        let root = tree.add(NodeKind::Scroll(0.0), vec![pad]);

        tree.print(root, 0);

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
    fn on_press(&mut self, mouse: Vec2, _button: MouseButton) {
        let click = self.tree.click(self.root, mouse);
        println!("{:?}", click);
    }

    fn on_mouse_scroll(&mut self, _dx: f32, dy: f32) {
        self.scroll -= dy;
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
    let chapters = fs::read_dir("./res/files")
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>();

    let mut app = MyApp::new(chapters.as_slice());

    if let Err(e) = pollster::block_on(app.run()) {
        eprintln!("Render: {:?}", e);
    }
}
