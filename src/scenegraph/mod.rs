use crate::graphics::draw::Draw;

pub mod group;
pub mod scene;
pub mod sprite;

pub trait Node {
    fn init(&self);
    fn event(&self);
    fn update(&self);
    fn draw(&self, draw: &mut Draw);
}

pub trait Parent {
    fn add_child(&mut self, child: Box<dyn Node>);
    fn remove_child(&mut self, child: Box<dyn Node>);
}

pub trait Script {
    fn on_init(&mut self, f: fn());
    fn on_event(&mut self, f: fn());
    fn on_update(&mut self, f: fn());
    fn on_draw(&mut self, f: fn());
}
