#![windows_subsystem = "windows"] 

pub mod firework;
pub mod color;
pub mod star;
pub mod object;
pub mod static_point;
pub mod trail;

use color::Color;
use firework::Firework;
use object::{Render, Pos3D};
use pixels::{Error, PixelsBuilder, SurfaceTexture};

use rand::{seq::SliceRandom, Rng};
use static_point::StaticPoint;
use winit::{
    dpi::{LogicalSize},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{WindowBuilder},
};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 800;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();

    // Initialise the window
    let window = WindowBuilder::new()
        .with_title("Fireworks!")
        // .with_decorations(false)
        // .with_transparent(true)
        .with_always_on_top(true)
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_min_inner_size(LogicalSize::new(100, 100))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    // Create a surface texture to render to
    let surface_texture = SurfaceTexture::new(
        window.inner_size().width,
        window.inner_size().height,
        &window,
    );

    // Create a pixelarray
    let mut pixels: pixels::Pixels = PixelsBuilder::new(WIDTH, HEIGHT, surface_texture).build()?;
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // println!("Window closed");
                control_flow.set_exit();
            }
            // Event::WindowEvent {
            //     event: WindowEvent::Resized(new_size),
            //     ..
            // } => {
            //     // println!("Window resized");
            //     pixels.resize_buffer(new_size.width, new_size.height);
            //     pixels.resize_surface(new_size.width, new_size.height);
            // }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {

                // Render changed pixelbuffer to screen
                if pixels
                    .render()
                    .map_err(|e| println!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    control_flow.set_exit();
                };
            }
            _ => (),
        }
    })
}