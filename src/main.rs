use std::num::NonZeroU32;
use std::time::Instant;
use std::f32::consts::PI;
use winit::{
    event::{Event, WindowEvent, DeviceEvent, ElementState, VirtualKeyCode},
    event_loop::EventLoop,
    window::WindowBuilder,
    dpi::LogicalSize,
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 200;

mod map;
mod player;
mod vector;
mod render;
mod color;
mod util;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_resizable(false)
        .build(&event_loop).unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    let map = map::Map::load("test.map");

    let mut player = player::Player::new(vector::Vec2::new(2.0, 1.5), 0.0, vector::Vec2::new(0.0, 0.0));

    let mut last = Instant::now();

    let mut keys_pressed = Vec::new();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event,
                ..
            } => {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("Close requested, exiting...");
                        control_flow.set_exit();
                    }
                    WindowEvent::KeyboardInput {
                        input,
                        ..
                    } => {
                        if let Some(keycode) = input.virtual_keycode {
                            match input.state {
                                ElementState::Pressed => {
                                    if !keys_pressed.contains(&keycode) {
                                        keys_pressed.push(keycode);
                                    }
                                }
                                ElementState::Released => {
                                    keys_pressed.retain(|&k| k != keycode);
                                }
                            }
                        }
                    },
                    _ => ()
                }
            },
            Event::RedrawRequested(_) => {
                let delta_time = last.elapsed();
                last = Instant::now();

                surface
                    .resize(
                        NonZeroU32::new(WIDTH).unwrap(),
                        NonZeroU32::new(HEIGHT).unwrap(),
                    )
                    .unwrap();
                
                let mut buffer = surface.buffer_mut().unwrap();
                buffer.fill(0xffffff);

                render::render(&map, &player, &mut buffer);

                player.set_velocity(vector::Vec2::new(
                    if keys_pressed.contains(&VirtualKeyCode::D) { 1.0 } else { 0.0 }
                    + if keys_pressed.contains(&VirtualKeyCode::A) { -1.0 } else { 0.0 },
                    if keys_pressed.contains(&VirtualKeyCode::W) { 1.0 } else { 0.0 }
                    + if keys_pressed.contains(&VirtualKeyCode::S) { -1.0 } else { 0.0 }
                ).rotate(player.rotation));
                player.update(delta_time);

                buffer.present().unwrap();
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::DeviceEvent { device_id, event } => {
                match event {
                    DeviceEvent::MouseMotion { delta } => {
                        player.rotation -= PI * 0.001 * delta.0 as f32;
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    });
}
