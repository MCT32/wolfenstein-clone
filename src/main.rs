use std::num::NonZeroU32;
use std::time::Instant;
use std::f32::consts::PI;
use winit::{
    event::{Event, WindowEvent},
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
mod util;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_resizable(false)
        .build(&event_loop).unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    let mut map = map::Map::new(4, 4);
    map.set(0, 0, 1);
    map.set(1, 0, 1);
    map.set(2, 0, 1);
    map.set(3, 0, 1);
    map.set(3, 1, 1);
    map.set(3, 2, 1);
    map.set(3, 3, 1);
    map.set(3, 3, 1);
    map.set(2, 3, 1);
    map.set(1, 3, 1);
    map.set(0, 3, 1);
    map.set(0, 2, 1);
    map.set(0, 1, 1);

    let mut player = player::Player::new(vector::Vec2::new(2.0, 1.5), 0.0, vector::Vec2::new(0.0, 0.0));

    let mut last = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Close requested, exiting...");
                control_flow.set_exit();
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

                player.update(delta_time);
                player.rotation -= PI * 0.1 * delta_time.as_secs_f32();

                buffer.present().unwrap();
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            _ => ()
        }
    });
}
