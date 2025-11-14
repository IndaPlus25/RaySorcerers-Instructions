/**
 * Traditional Random Walk
 * - The Nature of Code, Example I.1
 * See: https://natureofcode.com/book/introduction/ 
 * 
 * Author: Viola Söderlund <violaso@kth.se>
 * Last updated: 2022-11-14
 */

mod walker;

extern crate piston_window;
extern crate image as piston_image;
extern crate opengl_graphics;

use gfx_device_gl::{CommandBuffer, Factory, Resources};
use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings
};
use piston_image::{ImageBuffer, Rgba};
use piston_window::{Event, G2dTexture, PistonWindow, Texture, TextureContext, TextureSettings};
use walker::Walker;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

const WHITE_COLOUR: [f32; 4] = [1.0; 4];

pub struct App {
    canvas: ImageBuffer<Rgba<u8>, Vec<u8>>,
    texture: G2dTexture,
    texture_context: TextureContext<Factory, Resources, CommandBuffer>,
    walker: Walker
}

impl App {
    fn new(window: &mut PistonWindow) -> Self {
        // Create image for rendering.
        let canvas = ImageBuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        // Create texture interface for window.
        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into()
        };

        // Create texture interface between window interface and the canvas.
        let texture = Texture::from_image(
            &mut texture_context,
            &canvas,
            &TextureSettings::new()
        );

        App {
            canvas,
            texture: texture.unwrap(), 
            texture_context,
            walker: Walker::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)
        }
    }

    fn render(&mut self, event: &Event, window: &mut PistonWindow) {
        // Update texture interfaces.
        self.texture.update(&mut self.texture_context, &self.canvas).unwrap();

        // Update application window.
        window.draw_2d(event, |context, graphics, device| {
            // Submit the updated texture commands to the GPU.
            self.texture_context.encoder.flush(device);

            // Fill the window with white colour.
            piston_window::clear(WHITE_COLOUR, graphics);
            // Draw the texture.
            piston_window::image(&self.texture, context.transform, graphics);
        });
    }

    fn update(&mut self) {
        self.walker.step(&mut self.canvas);
    }
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Example I.1: Traditional random walk", (SCREEN_WIDTH, SCREEN_HEIGHT))
            .exit_on_esc(true)
            .graphics_api(OpenGL::V3_2)
            .build()
            .unwrap();

    let mut app = App::new(&mut window);

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            app.render(&event, &mut window);
        }
        if let Some(_) = event.update_args() {
            app.update();
        }
    }
}