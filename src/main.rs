mod layout;
mod ruglet;

use std::{fs, time::Instant};

use layout::*;
use ruglet::*;
use winit::event::MouseButton;

struct MyApp {
    tree: Tree,
    root: usize,
    font: FontAtlas,
    text: TextBook,
    link: Vec<String>,
}

impl MyApp {
    fn new() -> Self {
        let font = FontAtlas::new();
        let mut tree = Tree::new();
        let mut text = TextBook::new();
        let mut link = vec![];

        let chapters = fs::read_dir("./res/files")
            .unwrap()
            .map(|file| file.unwrap())
            .map(|file| {
                let name = &file.file_name().into_string().unwrap();
                let path = file.path().as_path().to_str().unwrap().to_string();

                // show the text
                let tid = text.add(name);
                let text = tree.add(NodeKind::Text(tid), vec![]);

                // make it clickable
                link.push(path);
                return tree.add(NodeKind::Clickable(link.len() - 1), vec![text]);
            })
            .collect();

        let pad = tree.add(NodeKind::Pad(50.0), chapters);

        let root = tree.add(NodeKind::Scroll(0.0), vec![pad]);

        tree.print(root, 0);

        return MyApp {
            tree,
            root,
            font,
            text,
            link,
        };
    }

    fn open(&mut self, selected: usize) {
        let path = &self.link[selected];
        let text = fs::read_to_string(path).unwrap();

        let paragraphs = text
            .lines()
            .map(|option| {
                let tid = self.text.add(&option);
                return self.tree.add(NodeKind::Text(tid), vec![]);
            })
            .collect();

        let pad = self.tree.add(NodeKind::Pad(50.0), paragraphs);
        let root = self.tree.add(NodeKind::Scroll(0.0), vec![pad]);

        self.tree.replace(self.root, root);
    }
}

impl<'a> Application for MyApp {
    fn on_press(&mut self, mouse: Vec2, _button: MouseButton) {
        let click = self.tree.click(self.root, mouse);

        if let Some(selected) = click {
            self.open(selected);
        }
    }

    fn on_mouse_scroll(&mut self, _dx: f32, dy: f32) {
        if let NodeKind::Scroll(scroll) = self.tree.get(self.root) {
            self.tree.update(self.root, NodeKind::Scroll(scroll - dy));
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let now = Instant::now();

        // Render the dom
        self.tree
            .build(self.root, frame, &mut self.font, &self.text);

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}

fn main() {
    let mut app = MyApp::new();

    if let Err(e) = pollster::block_on(app.run()) {
        eprintln!("Render: {:?}", e);
    }
}
