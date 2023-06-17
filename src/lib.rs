mod graphics;

use log::LevelFilter;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use crate::graphics::state::State;

pub fn run(title: &str) {

    let _ = TermLogger::init(LevelFilter::Info, Config::default(),TerminalMode::Mixed, ColorChoice::Auto);

    // create event loop
    let event_loop = EventLoop::new();

    // create window
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    window.set_title(title);

    // create state
    let mut state = pollster::block_on(State::new(window));

    // run event loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id} if window_id == state.window().id() => if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                state.window().request_redraw();
            }
            _ => {}
        }
    });

}