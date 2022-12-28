use std::ops;
use instant::Duration;
use winit::dpi::PhysicalSize;

pub trait PhysicsObject {
    fn pos(&self) -> Pos3D;
    fn vel(&self) -> Pos3D;

    fn calculate_forces(&self) -> Pos3D;

    fn calculate_velocity(&self, forces: Pos3D, dt: Duration) -> Pos3D {
        return self.vel() + forces * dt.as_secs_f64() as f64;
    }
    fn calculate_position(&self, dt: Duration) -> Pos3D {
        return self.pos() + self.vel() * dt.as_secs_f64() as f64;
    }
}

pub trait Render {
    fn render(&self, screen: &mut [u8], size: PhysicalSize<u32>, t: f64);
    fn print_point(
        x: i32,
        y: i32,
        r: i32,
        screen: &mut [u8],
        size: PhysicalSize<u32>,
        color: [u8; 4],
    ) {
        for x_off in -r..=r {
            for y_off in -r..=r {
                let x_p = x + x_off;
                let y_p = y + y_off;

                print_coord_in_pixelbuffer(x_p, y_p, screen, size, color)
            }
        }
    }
}

fn print_coord_in_pixelbuffer(
    x: i32,
    y: i32,
    screen: &mut [u8],
    size: PhysicalSize<u32>,
    color: [u8; 4],
) {
    // Calculate the index of the current coordinate
    if x <= size.width as i32 && x >= 0 && y <= size.height as i32 && y >= 0 {
        let i = (y * size.width as i32) as usize + x as usize;

        // Update for every color
        if i * 4 < screen.len() && i * 4 > 0 {
            for c in 0..3 {
                screen[i * 4 + c] = color[c];
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pos3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Mul<f64> for Pos3D {
    type Output = Pos3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Pos3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl ops::Add for Pos3D {
    type Output = Pos3D;

    fn add(self, rhs: Self) -> Self::Output {
        Pos3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Pos3D {
    pub fn project(&self, size: PhysicalSize<u32>, t: f64) -> Pos2D {
        static SCREEN_MATRIX_3D: [[f64; 3]; 2] = [
            [0.866, 0.0, -0.866],
            [-0.5, -1.0, -0.5],
        ];

        let speed: f64 = 0.2;

        let sin: f64 = (t * speed).sin();
        let cos: f64 = (t * speed).cos();

        let rotated = Pos3D {
            x: self.x * cos + self.z * -sin,  
            y: self.y,
            z: self.x * sin + self.z * cos,
        };

        Pos2D {
            x: (
                SCREEN_MATRIX_3D[0][0] * rotated.x + 
                SCREEN_MATRIX_3D[0][1] * rotated.y + 
                SCREEN_MATRIX_3D[0][2] * rotated.z
            ) + size.width as f64 / 2.0,
            y: (
                SCREEN_MATRIX_3D[1][0] * rotated.x + 
                SCREEN_MATRIX_3D[1][1] * rotated.y + 
                SCREEN_MATRIX_3D[1][2] * rotated.z
            ) + 3.0 * size.height as f64 / 4.0,
        }
    }

    pub fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

pub struct Pos2D {
    pub x: f64,
    pub y: f64,
}