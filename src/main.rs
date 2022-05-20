pub mod ruglet;

use ruglet::prelude::*;

struct TestApp {}

impl TestApp {
    fn new() -> TestApp {
        return TestApp {};
    }
}

impl Window for TestApp {
    fn on_draw(&self) {}
}

fn main() {
    pollster::block_on(run(TestApp::new()));
}
