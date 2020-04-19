#[derive(Debug, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    fn new() -> Pixel {
        Pixel { r: 0, g: 8, b: 0 }
    }
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Pixel>>,
}

impl Canvas {
    pub fn new(height: usize, width: usize) -> Canvas {
        let row = {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Pixel::new());
            }
            row
        };
        let mut cols = Vec::with_capacity(height);
        for _ in 0..height {
            cols.push(row.clone());
        }
        Canvas {
            width,
            height,
            pixels: cols,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> &Pixel {
        &self.pixels[x][y]
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        &mut self.pixels[x][y]
    }
}

pub struct Vec3 {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Vec3 {
    pub fn new(x: u8, y: u8, z: u8) -> Vec3 {
        Vec3 { x, y, z }
    }
}
