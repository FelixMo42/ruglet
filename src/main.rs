mod layout;
mod ruglet;

use layout::*;
use ruglet::*;

struct MyApp<'a> {
    tree: Tree<'a>,
    root: usize,
    font: FontAtlas,
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

        let root = tree.add(NodeKind::Scroll, vec![pad]);

        return MyApp { tree, root, font };
    }
}

impl<'a> Application for MyApp<'a> {
    fn on_draw(&mut self, frame: &mut Frame) {
        // Render the dom
        self.tree.build(self.root, frame, &mut self.font);
    }
}

fn main() {
    let text = include_str!("../res/test.txt");

    let mut app = MyApp::new(text);

    if let Err(e) = pollster::block_on(app.run()) {
        eprintln!("Error: {:?}", e);
    }
}
