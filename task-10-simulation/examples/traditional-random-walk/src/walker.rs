use piston_image::{ImageBuffer, Rgba};

const STEP_COLOUR: Rgba<u8> = Rgba([0x0, 0x0, 0x0, 0xFF]);

enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3
}

impl Direction {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x0 => Some(Direction::Up),
            0x1 => Some(Direction::Right),
            0x2 => Some(Direction::Down),
            0x3 => Some(Direction::Left),
            _ => None
        }
    }
}

pub struct Walker {
    x: u32,
    y: u32
}

impl Walker {
    pub fn new(x: u32, y: u32) -> Self {
        Walker {
            x, y
        }
    }

    pub fn step(&mut self, canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();

        match Direction::from_u8(rng.gen_range(0..4) as u8).unwrap() {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1
        }

        canvas.put_pixel(self.x, self.y, STEP_COLOUR)
    }
}