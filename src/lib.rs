mod utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pixel {
    Black = 0,
    White = 1,
}

impl Pixel {
    pub fn toggle(&mut self) {
        *self = match *self {
            Pixel::Black => Pixel::White,
            Pixel::White => Pixel::Black,
        }
    }

    pub fn random() -> Pixel {
        if js_sys::Math::random() < 0.5 {
            Pixel::Black
        } else {
            Pixel::White
        }
    }
}

#[wasm_bindgen]
pub struct PovBoard {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
    queue: Vec<(i32, i32)>,
    cleanups: Vec<(i32, i32)>,
}

#[wasm_bindgen]
impl PovBoard {
    pub fn new(width: u32, height: u32) -> PovBoard {
        utils::set_panic_hook();
        let mut pixels = Vec::with_capacity((width * height) as usize);
        for _ in 0..=(width * height) {
            pixels.push(Pixel::random());
        }
        PovBoard {
            width,
            height,
            pixels,
            cleanups: Vec::new(),
            queue: Vec::new(),
        }
    }

    pub fn randomize(&mut self) {
        for pixel in &mut self.pixels {
            *pixel = Pixel::random();
        }
    }

    pub fn black(&mut self) {
        self.pixels = Vec::with_capacity((self.width * self.height) as usize);
        for _ in 0..=(self.width * self.height) {
            self.pixels.push(Pixel::Black);
        }
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }

    pub fn tick(&mut self) {
        for point in &self.cleanups {
            self.pixels[(point.1 * self.width as i32 + point.0) as usize].toggle();
        }
        self.cleanups.clear();

        while !self.queue.is_empty() {
            let point = self.queue.remove(0);
            if point.0.is_negative() || point.1.is_negative() {
                continue;
            }
            if point.0 >= self.width as i32 || point.1 >= self.height as i32 {
                continue;
            }

            self.cleanups.push(point);
            self.pixels[(point.1 * self.width as i32 + point.0) as usize].toggle();
        }
    }

    pub fn draw_cube(&mut self, x1: i32, y1: i32, r: i32, angle: f32) {
        let x1 = x1;
        let y1 = y1;

        let mut cube = utils::points_for_cube(x1, y1, r);
        if angle > 0.0 {
            utils::rotate_shape(&mut cube, angle)
        }

        for point in cube {
            self.queue.push(point);
        }
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let points = utils::points_for_line(x1, y1, x2, y2);
        for point in points {
            self.queue.push(point);
        }
    }

    pub fn draw_vertical_line(&mut self, x: i32, y1: i32, y2: i32) {
        self.draw_line(x, y1, x, y2);
    }

    pub fn draw_horizontal_line(&mut self, y: i32, x1: i32, x2: i32) {
        self.draw_line(x1, y, x2, y);
    }
}
