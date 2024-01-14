use crate::graphics::draw::Draw;
use crate::scenegraph::{Node, Parent};

pub struct Scene {
    children: Vec<Box<dyn Node>>,
}

impl Default for Scene {
    fn default() -> Self {
        Scene { children: vec![] }
    }
}

impl Node for Scene {
    fn init(&self) {
        self.children.iter().for_each(|c| c.init());
    }

    fn event(&self) {
        println!("Scene event");
    }

    fn update(&self) {
        self.children.iter().for_each(|c| c.update());
    }

    fn draw(&self, draw: &mut Draw) {
        self.children.iter().for_each(|c| c.draw(draw));
    }
}

impl Parent for Scene {
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.push(child);
    }

    fn remove_child(&mut self, child: Box<dyn Node>) {
        self.children.retain(|c| !std::ptr::eq(c, &child));
    }
}

impl Scene {
    pub fn new() -> Self {
        Scene::default()
    }

    pub fn run() {
        todo!()
    }
}