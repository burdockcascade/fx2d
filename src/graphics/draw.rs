#[derive(Clone, Debug)]
pub enum DrawCommand {
    DrawTexture(String),
}

pub struct Draw {
    commands: Vec<DrawCommand>,
}

impl Draw {

    pub fn new() -> Self {
        Draw {
            commands: vec![],
        }
    }

    pub fn draw_texture(&mut self, src: &str) {
        self.commands.push(DrawCommand::DrawTexture(src.to_string()));
    }

    pub fn get_commands(&self) -> Vec<DrawCommand> {
        self.commands.clone()
    }

}