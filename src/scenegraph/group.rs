use crate::graphics::draw::Draw;
use crate::scenegraph::{Node, Parent};

pub struct GroupNode {
    children: Vec<Box<dyn Node>>,
}

impl GroupNode {
    pub fn new() -> Self {
        GroupNode {
            children: vec![],
        }
    }

    pub fn with_children(children: Vec<Box<dyn Node>>) -> Self {
        GroupNode {
            children,
        }
    }
}

impl Node for GroupNode {
    fn init(&self) {
        self.children.iter().for_each(|c| c.init());
    }

    fn event(&self) {
        self.children.iter().for_each(|c| c.event());
    }

    fn update(&self) {
        self.children.iter().for_each(|c| c.update());
    }

    fn draw(&self, draw: &mut Draw) {
        self.children.iter().for_each(|c| c.draw(draw));
    }
}

impl Parent for GroupNode {
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.push(child);
    }

    fn remove_child(&mut self, child: Box<dyn Node>) {
        self.children.retain(|c| !std::ptr::eq(c, &child));
    }
}
