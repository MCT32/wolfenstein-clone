use std::num::NonZeroU32;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
    dpi::LogicalSize,
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 200;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_resizable(false)
        .build(&event_loop).unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

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
                surface
                    .resize(
                        NonZeroU32::new(WIDTH).unwrap(),
                        NonZeroU32::new(HEIGHT).unwrap(),
                    )
                    .unwrap();
                
                let mut buffer = surface.buffer_mut().unwrap();
                for index in 0..(WIDTH * HEIGHT) {
                    let y = index / WIDTH;
                    let x = index % WIDTH;
                    let red = x % 200;
                    let green = y % 200;
                    let blue = (x * y) % 200;

                    buffer[index as usize] = blue | (green << 8) | (red << 16);
                }

                buffer.present().unwrap();
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            _ => ()
        }
    });
}
