/**
 * Chess GUI template.
 * Author: Isak Larsson <isaklar@kth.se>
 * Last updated: 2022-09-29
 */
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::HashMap;

use chess_template::{Colour, Game, PieceType};

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, MouseButton, MouseCursorEvent, PressEvent};

/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 8;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: [f32; 4] = [228.0 / 255.0, 196.0 / 255.0, 108.0 / 255.0, 1.0];
const WHITE: [f32; 4] = [188.0 / 255.0, 140.0 / 255.0, 76.0 / 255.0, 1.0];

pub struct App {
    gl: GlGraphics,                                 // OpenGL drawing backend.
    mouse_pos: [f64; 2],                            // Current mouse postition
    sprites: HashMap<(Colour, PieceType), Texture>, // For easy access to the apropriate PNGs
    board: [[Option<(Colour, PieceType)>; 8]; 8], // Or whatever way you prefer to represent the board (hint: might not be neccesary)
    game: Game, // Save piece positions, which tiles has been clicked, current colour, etc...
}

impl App {
    fn new(opengl: OpenGL) -> App {
        let royal_rank = |colour| {
            [
                Some((colour, PieceType::Rook)),
                Some((colour, PieceType::Knight)),
                Some((colour, PieceType::Bishop)),
                Some((colour, PieceType::Queen)),
                Some((colour, PieceType::King)),
                Some((colour, PieceType::Rook)),
                Some((colour, PieceType::Knight)),
                Some((colour, PieceType::Bishop)),
            ]
        };
        let pawn_rank = |colour| [Some((colour, PieceType::Pawn)); 8];
        let empty_rank = || [None; 8];

        App {
            gl: GlGraphics::new(opengl),
            mouse_pos: [0., 0.],
            board: [
                royal_rank(Colour::Black),
                pawn_rank(Colour::Black),
                empty_rank(),
                empty_rank(),
                empty_rank(),
                empty_rank(),
                pawn_rank(Colour::White),
                royal_rank(Colour::White),
            ],
            game: Game::new(),
            sprites: Self::load_sprites(),
        }
    }
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        use graphics::*; // Now we don't have to use this everytime :D

        let square = rectangle::square(0.0, 0.0, GRID_CELL_SIZE.0 as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // // Clear the screen.
            // clear(GREEN, gl);
            // draw grid
            for row in 0..8 {
                for col in 0..8 {
                    // draw tile
                    rectangle(
                        match col % 2 {
                            0 => {
                                if row % 2 == 0 {
                                    WHITE
                                } else {
                                    BLACK
                                }
                            }
                            _ => {
                                if row % 2 == 0 {
                                    BLACK
                                } else {
                                    WHITE
                                }
                            }
                        },
                        square,
                        c.transform.trans(
                            (col * GRID_CELL_SIZE.0) as f64,
                            (row * GRID_CELL_SIZE.0) as f64,
                        ),
                        gl,
                    );

                    // draw piece
                    if let Some(piece) = self.board[row as usize][col as usize] {
                        let img = Image::new().rect(square);
                        img.draw(
                            self.sprites.get(&piece).unwrap(),
                            &c.draw_state,
                            c.transform.trans(
                                (col * GRID_CELL_SIZE.0) as f64,
                                (row * GRID_CELL_SIZE.0) as f64,
                            ),
                            gl,
                        )
                    }
                }
            }

            // Draw text
            // We do some calculations to center the text
            // Is not exactly in the middle, try to fix it if you want to!
            let state_text = format!("The game is {:?}", self.game.get_game_state());
            let text_size: (f32, f32) = ((24 * state_text.len()) as f32, 24f32);
            let text_postition = c.transform.trans(
                ((SCREEN_SIZE.0 - text_size.0) / 2f32) as f64,
                ((SCREEN_SIZE.1 - text_size.1) / 2f32) as f64,
            );
            text::Text::new(32)
                .draw(&state_text, glyphs, &c.draw_state, text_postition, gl)
                .unwrap();
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Currently empty, but maybe you can find a fun use for it!
    }

    #[rustfmt::skip]
    /// Loads chess piese images into vector.
    fn load_sprites() -> HashMap<(Colour, PieceType), Texture> {

        [
            ((Colour::Black, PieceType::King), "resources/black_king.png".to_string()),
            ((Colour::Black, PieceType::Queen), "resources/black_queen.png".to_string()),
            ((Colour::Black, PieceType::Rook), "resources/black_rook.png".to_string()),
            ((Colour::Black, PieceType::Pawn), "resources/black_pawn.png".to_string()),
            ((Colour::Black, PieceType::Bishop), "resources/black_bishop.png".to_string()),
            ((Colour::Black, PieceType::Knight), "resources/black_knight.png".to_string()),
            ((Colour::White, PieceType::King), "resources/white_king.png".to_string()),
            ((Colour::White, PieceType::Queen), "resources/white_queen.png".to_string()),
            ((Colour::White, PieceType::Rook), "resources/white_rook.png".to_string()),
            ((Colour::White, PieceType::Pawn), "resources/white_pawn.png".to_string()),
            ((Colour::White, PieceType::Bishop), "resources/white_bishop.png".to_string()),
            ((Colour::White, PieceType::Knight), "resources/white_knight.png".to_string())
        ]
            .iter()
            .map(|(piece, path)| {
                (*piece, Texture::from_path(path, &TextureSettings::new()).unwrap())
            })
            .collect::<HashMap<(Colour, PieceType), Texture>>()
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window =
        WindowSettings::new("Chess", [SCREEN_SIZE.0 as f64, SCREEN_SIZE.1 as f64])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Initialize our app state
    let mut app = App::new(opengl);

    // Initialize font
    let mut glyphs = GlyphCache::new(
        "resources/AbyssinicaSIL-Regular.ttf",
        (),
        TextureSettings::new(),
    )
    .unwrap();

    let mut events = Events::new(EventSettings::new());
    // Our "game loop". Will run until we exit the window
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &mut glyphs);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        if let Some(pos) = e.mouse_cursor_args() {
            app.mouse_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Handle mouse press
        }
    }
}
