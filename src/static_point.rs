use rand::Rng;

use crate::object::{Pos3D, Render, Pos2D};
use crate::color::Color;
use winit::dpi::PhysicalSize;

pub struct StaticPoint {
    pos: Pos3D,
    color: Color,
}

impl Render for StaticPoint {
    fn render(&self, screen: &mut [u8], size: PhysicalSize<u32>, t: f64) {
        let proj: Pos2D = self.pos.project(size, t);

        let radius: i32 = 0;

        let [r, g, b] = self.color.get_rgb();
        let rgba: [u8; 4] = [r, g, b, 0xff];

        Self::print_point(proj.x as i32, proj.y as i32, radius, screen, size, rgba);
    }
}

impl StaticPoint {
    pub fn new(pos: Pos3D, color: Color) -> Self {
        let mut rng = rand::thread_rng();

        let pos = Pos3D {
            x:  pos.x,
            y:  pos.y + rng.gen_range(-10.0..=10.0),
            z:  pos.z,
        };

        Self { pos, color }
    }
}