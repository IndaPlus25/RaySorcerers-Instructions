use crate::math::random::{self, PerlinProfile};
use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::math::Matrix2d;
use std::f64::consts::PI;

const RADIUS: f64 = 16.0;
const STEP_COLOUR: [f32; 4] = [1.0; 4];
const STEP_FACTOR: f64 = 100.0;
const TIME_STEP: f64 = PI / 100.0;

pub struct Walker {
    profile_x: PerlinProfile,
    profile_y: PerlinProfile,
    time: f64,
    transform_x: f64,
    transform_y: f64,
    x: f64,
    y: f64
}

impl Walker {
    pub fn new(x: f64, y: f64) -> Self {
        Walker {
            profile_x: random::generate_profile(),
            profile_y: random::generate_profile(),
            time: 0.0,
            transform_x: x,
            transform_y: y,
            x, y
        }
    }

    pub fn step(&mut self) {
        self.time += TIME_STEP;
        self.x = self.next_position(&self.profile_x, self.transform_x);
        self.y = self.next_position(&self.profile_y, self.transform_y);
    }

    fn next_position(&self, profile: &PerlinProfile, transform: f64) -> f64 {
        random::perlin_noise(self.time, profile) * (STEP_FACTOR - transform / 2.0) + transform
    }

    pub fn render(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {
        let square = piston_window::ellipse::circle(self.x, self.y, RADIUS);
        piston_window::ellipse(STEP_COLOUR, square, transform, graphics);
    }
}