use instant::Duration;
use rand::Rng;
use winit::dpi::PhysicalSize;

use crate::{
    object::{
        Pos3D, 
        PhysicsObject, 
        Render
    }, 
    color::Color, 
};

const GRAVITY: f64 = -9.81; 

pub struct Star {
    pub pos: Pos3D,
    pub vel: Pos3D,
    pub color: Color,
    pub age: f64,
    pub is_alive: bool,
    pub trail: Vec<Trail>
}

impl PhysicsObject for Star {
    fn pos(&self) -> Pos3D {
        self.pos
    }

    fn vel(&self) -> Pos3D {
        self.vel
    }

    fn calculate_forces(&self) -> Pos3D {
        let k = 0.01;

        let air_res = Pos3D {
            x: - k * self.vel().x.powi(2),
            y: - k * self.vel().y.powi(2),
            z: - k * self.vel().z.powi(2),
        };

        let gravity = Pos3D {
            x: 0.0,
            y: GRAVITY * 0.2,
            z: 0.0,
        };

        air_res + gravity
    }
}

impl Render for Star {
    fn render(&self, screen: &mut [u8], size: PhysicalSize<u32>, t: f64) {
        // Only render alive stars
        if !self.is_alive {return;}


        let proj = self.pos.project(size, t);

        let r = (self.age / 1.5) as i32;

        let c = self.color.get_rgb();
        let rgba = [c[0], c[1], c[2], 0xff];

        Self::print_point(proj.x as i32, proj.y as i32, r, screen, size, rgba);
    }
}

impl Star {
    pub fn fly(&mut self, dt: Duration) {
        self.age -= dt.as_secs_f64();

        if self.age <= 0.0 {
                self.is_alive = false;
        }
        
        // Move the star
        let forces = self.calculate_forces();
        self.vel = self.calculate_velocity(forces, dt);
        self.pos = self.calculate_position(dt);

    pub fn new(pos: Pos3D, _r_vel: Pos3D, color: Color) -> Self {
        let mut rng = rand::thread_rng();

        let vel = Pos3D { 
            x: rng.gen_range(-20.0..=20.0),
            y: rng.gen_range(-20.0..=20.0), 
            z: rng.gen_range(-20.0..=20.0), 
        }; // + _r_vel * 0.1;

        let age = rng.gen_range(1.5..=2.0);


        Self { pos, vel, color, age, is_alive: true, trail }
    }
}