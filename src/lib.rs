use std::time::{Duration, Instant};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use graphics::texture;
use crate::graphics::state::State;
use crate::scenegraph::scene::Scene;


mod graphics;
pub mod scenegraph;

const TARGET_FPS: u32 = 60;


#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run(scene: Scene) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut state = State::new(window, scene).await;

    let mut last_frame_time = Instant::now();
    let mut frame_count = 0;

    event_loop.run(move |event, _target, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id} if window_id == state.display.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.display.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.display.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.display.window().id() => {
                
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.display.resize(state.display.size)
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }

                // Calculate FPS
                frame_count += 1;
                let now = Instant::now();
                let elapsed = now - last_frame_time;
                if elapsed >= Duration::from_secs(1) {
                    let fps = frame_count as f64 / elapsed.as_secs_f64();
                    println!("FPS: {}", fps);
                    frame_count = 0;
                    last_frame_time = now;
                }

            }
            Event::MainEventsCleared => {
                state.display.window().request_redraw();
            }
            _ => {}
        }
    });
}
