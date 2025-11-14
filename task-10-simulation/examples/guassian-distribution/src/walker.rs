use crate::math;
use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::math::Matrix2d;
use std::collections::VecDeque;

const STEP_LIMIT: usize = 100;
const RADIUS: f64 = 16.0;
const STEP_COLOUR: [f32; 4] = [1.0, 1.0, 1.0, 0.1];
const STD_DEVIATION: f64 = 60.0;

pub struct Walker {
    steps: VecDeque<f64>,
    x: f64,
    y: f64
}

impl Walker {
    pub fn new(x: f64, y: f64) -> Self {
        Walker {
            steps: VecDeque::with_capacity(STEP_LIMIT),
            x, y
        }
    }

    pub fn step(&mut self) {
        let step = math::random::random_guassian(self.x, STD_DEVIATION);

        if self.steps.len() == STEP_LIMIT {
            self.steps.pop_front();
        }
        self.steps.push_back(step);
    }

    pub fn render(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {
        for step in &self.steps {
            let square = piston_window::ellipse::circle(*step, self.y, RADIUS);
            piston_window::ellipse(STEP_COLOUR, square, transform, graphics);
        }
    }
}