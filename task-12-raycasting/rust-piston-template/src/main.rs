extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use piston_window::*;
use vecmath::*;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn render(canvas: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>) {
    // Test draw
    for i in 0..WINDOW_WIDTH {
        canvas.put_pixel(i, WINDOW_HEIGHT / 2, im::Rgba([255; 4]));
    };
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("Raycasting", (WINDOW_WIDTH, WINDOW_HEIGHT))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut canvas = im::ImageBuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };
    let mut texture: G2dTexture = Texture::from_image(
            &mut texture_context,
            &canvas,
            &TextureSettings::new()
        ).unwrap();

    while let Some(e) = window.next() {
        if e.render_args().is_some() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([0.0; 4], g);

                image(&texture, c.transform, g);
            });
        }

        render(&mut canvas);
    }
}