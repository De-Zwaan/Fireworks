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

const MAX_FIREWORK: i32 = 20;

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
    // Create an empty vec to store all rockets
    let mut fireworks: Vec<Firework> = Vec::new();
    let mut t: f64 = 0.0;
    let mut last_render_time = instant::Instant::now();

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
                // Calculate the framedelta
                let now = instant::Instant::now();
                let dt = now.duration_since(last_render_time);
                last_render_time = now;

                // Update time since start to update rotation
                t += dt.as_secs_f64();
                
                // Get the window size
                let size = window.inner_size();

                // Get pixelbuffer
                let screen = pixels.get_frame_mut();
                for (_i, p) in screen.chunks_exact_mut(4).enumerate() {
                    p.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
                }

                fireworks.iter_mut().for_each(|r| {
                    // Update firework physics
                    r.fly(dt);

                    // Draw rockets and their stars and trails
                    r.render(screen, size, t);
                });

                // Remove all rockets without stars
                fireworks.retain(|r| r.stars_alive != 0 || r.is_alive);                

                // Add new rockets until there are FIREWORK_NUM rockets
                (0..(MAX_FIREWORK as usize - fireworks.len())).for_each(|_| {
                    if rand::thread_rng().gen_bool(0.01) {
                        let color = *colors.choose(&mut rand::thread_rng()).unwrap();
                        fireworks.push(Firework::new(color));
                    }
                });

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