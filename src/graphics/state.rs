use winit::event::WindowEvent;
use winit::window::Window;
use crate::graphics::draw::Draw;
use crate::graphics::draw::DrawCommand;
use crate::scenegraph::Node;
use crate::scenegraph::scene::Scene;
use super::display::Display;
use super::pipeline::Pipeline;


pub struct State {
    root_scene: Scene,
    pub display: Display,
}

impl State {
    pub async fn new(window: Window, scene: Scene) -> Self {
        
        let display = Display::new(window).await;

        scene.init();

        Self {
            root_scene: scene,
            display,
        }
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub(crate) fn update(&mut self) {
        self.root_scene.update();
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        let draw = &mut Draw::new();

        self.root_scene.draw(draw);

        let output = self.display.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let commands = draw.get_commands();

        let mut command_buffers = vec![];

        for command in commands {
            match command {
                DrawCommand::DrawTexture(src) => {
                    let pipeline = Pipeline::for_texture(&self.display, &src);

                    let mut encoder = self.display.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                    {

                        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Render Pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            occlusion_query_set: None,
                            timestamp_writes: None,
                        });
                        
                        render_pass.set_pipeline(&pipeline.render_pipeline);
                        render_pass.set_bind_group(0, &pipeline.diffuse_bind_group, &[]);
                        render_pass.set_vertex_buffer(0, pipeline.vertex_buffer.slice(..));
                        render_pass.set_index_buffer(pipeline.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                        render_pass.draw_indexed(0..pipeline.num_indices, 0, 0..1);

                    }

                    command_buffers.push(encoder.finish())
                }
                _ => {
                    panic!("Unsupported draw command");
                }
            }
        }

        self.display.queue.submit(command_buffers);
        output.present();

        Ok(())
    }
}