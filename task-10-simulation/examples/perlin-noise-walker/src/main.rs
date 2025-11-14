/**
 * Perlin Noise Walker
 * - The Nature of Code, Example I.5
 * See: https://natureofcode.com/book/introduction/ 
 * 
 * Author: Viola Söderlund <violaso@kth.se>
 * Last updated: 2022-11-23
 */

mod math;
mod walker;

extern crate gfx_graphics;
extern crate opengl_graphics;
extern crate piston_window;

use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings
};
use piston_window::{Event, PistonWindow};
use walker::Walker;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

const BLACK_COLOUR: [f32; 4] = [0.0; 4];

pub struct App {
    walker: Walker
}

impl App {
    fn new() -> Self {
        App {
            walker: Walker::new(SCREEN_WIDTH as f64 / 2.0, SCREEN_HEIGHT as f64 / 2.0)
        }
    }

    fn render(&mut self, event: &Event, window: &mut PistonWindow) {
        // Update application window.
        window.draw_2d(event, |context, graphics, _| {
            // Fill the window with white colour.
            piston_window::clear(BLACK_COLOUR, graphics);

            self.walker.render(graphics, context.transform);
        });
    }

    fn update(&mut self) {
        self.walker.step();
    }
}

fn main() {
    use std::{thread, time};

    let mut window: PistonWindow =
        WindowSettings::new("Example I.5: Perlin Noise Walker", (SCREEN_WIDTH, SCREEN_HEIGHT))
            .exit_on_esc(true)
            .graphics_api(OpenGL::V3_2)
            .build()
            .unwrap();

    let mut app = App::new();

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            app.render(&event, &mut window);
        }
        if let Some(_) = event.update_args() {
            app.update();
        }

        let duration = time::Duration::from_millis(10);
        thread::sleep(duration);
    }
}