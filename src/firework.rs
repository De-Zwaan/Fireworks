use rand::Rng;

use crate::object::{Pos3D, PhysicsObject, Render};
use crate::color::Color;
use crate::star::Star;
use instant::Duration;
use winit::dpi::PhysicalSize;

// const GRAVITY: f64 = -9.81;

pub struct Firework {
    pos: Pos3D,
    vel: Pos3D,
    color: Color,
    age: f64,

    pub is_alive: bool,
    pub stars_alive: i32,
    pub stars: Vec<Star>,
    pub trail: Vec<Trail>,
}

impl PhysicsObject for Firework {
    fn pos(&self) -> Pos3D {self.pos}
    fn vel(&self) -> Pos3D {self.vel}

    fn calculate_forces(&self) -> Pos3D {
        let thrust: f64 = 25.0;
        
        let mut rng = rand::thread_rng();

        Pos3D {
            x: rng.gen_range(-2.0..=2.0) * thrust,
            y: thrust * self.age / 5.0,
            z: rng.gen_range(-2.0..=2.0) * thrust,
        }
    }
}

impl Render for Firework {
    fn render(&self, screen: &mut [u8], size: PhysicalSize<u32>, t: f64) {
        // Render all stars from current rocket
        self.stars.iter().for_each(|s| {
            s.render(screen, size, t);
        });
        // Only render rockets that are alive
        if !self.is_alive {return;}

        let proj = self.pos.project(size, t);

        let r = 0;

        let c = self.color.get_rgb();
        let rgba = [c[0], c[1], c[2], 0xff];

        Self::print_point(proj.x as i32, proj.y as i32, r, screen, size, rgba);
    }
}

impl Firework {
    pub fn fly(&mut self, dt: Duration) {
        // Decrease the time to live
        self.age -= dt.as_secs_f64() as f64;
        
        // Test if the fireworks are too old or out of bounds
        if self.age > 0.0 && self.pos.y < 350.0 && self.is_alive {
            // Move the rocket
            let forces = self.calculate_forces();
            self.vel = self.calculate_velocity(forces, dt);
            self.pos = self.calculate_position(dt);

        } else {
            self.is_alive = false;
        }

        // If the rocket is not alive anymore
        if !self.is_alive {
            // If the rocket is dead for the first tick
            if self.stars_alive == -1 {

                // Initialise a number of stars and push them to the list
                (0..50).for_each(|_i| {
                    Vec::push(&mut self.stars, Star::new(self.pos, self.vel, self.color));
                });
            }

            // Update the amount of alive stars that came from this rocket
            self.stars_alive = self.stars.len() as i32;

            // Iterate over all stars
            self.stars.iter_mut().for_each(|s| {
                s.fly(dt);
            });

            self.stars.retain(|s| s.is_alive);
        }
    }

    pub fn new(color: Color) -> Self {
        let mut rng = rand::thread_rng();

        let pos = Pos3D {
            x:  rng.gen_range(-70.0..=70.0),
            y:  -2.0,
            z:  rng.gen_range(-70.0..=70.0),
        };

        let vel = Pos3D { 
            x: rng.gen_range(-1.0..=1.0), 
            y: 5.0, 
            z: rng.gen_range(-1.0..=1.0),
        };

        let age = rng.gen_range(4.0..9.0);

        let stars = Vec::new();

        Self { pos, vel, age, color, is_alive: true, stars_alive: -1, stars, trail }
    }
}