use crate::{object::{Pos3D, Render}, color::Color};

#[derive(Clone, Copy)]
pub struct Trail {
    pub pos: Pos3D,
    pub color: Color,
    pub age: f64,
    pub is_alive: bool,
}

impl Render for Trail {
    fn render(&self, screen: &mut [u8], size: winit::dpi::PhysicalSize<u32>, t: f64) {
        // Only render alive stars
        if !self.is_alive {return;}

        let proj = self.pos.project(size, t);

        let r = (self.age / 1.5) as i32;

        let c = self.color.get_rgb();
        let rgba = [c[0], c[1], c[2], 0xff];

        Self::print_point(proj.x as i32, proj.y as i32, r, screen, size, rgba);
    }
}

impl Trail {
    pub fn new(pos: Pos3D, color: Color) -> Self {
        Self { pos, color, age: 1.0, is_alive: true }
    }
}