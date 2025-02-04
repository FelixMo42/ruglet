pub struct TextBook {
    blocks: Vec<String>,
}

impl TextBook {
    pub fn new() -> Self {
        return TextBook { blocks: vec![] };
    }

    pub fn add(&mut self, text: &str) -> usize {
        self.blocks.push(text.to_string());
        return self.blocks.len() - 1;
    }

    pub fn get(&self, id: usize) -> &str {
        return &self.blocks[id];
    }
}
