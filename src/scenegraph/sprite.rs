use crate::graphics::draw::Draw;
use crate::scenegraph::Node;

pub struct Sprite {
    src: String,
}

impl Node for Sprite {
    fn init(&self) {

    }

    fn event(&self) {

    }

    fn update(&self) {
    }

    fn draw(&self, draw: &mut Draw) {
        draw.draw_texture(&self.src)
    }
}

impl Sprite {
    pub fn new(src: &str) -> Self {
        Sprite {
            src: src.to_string(),
        }
    }

}
