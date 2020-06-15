#[derive(Clone)]
pub struct Pixel {
    data: [u8; 4],
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel { data: [0, 0, 0, 0] }
    }

    pub fn red(&self) -> u8 {
        self.data[0]
    }

    pub fn set_red(&mut self, r: u8) {
        self.data[0] = r;
    }

    pub fn blue(&self) -> u8 {
        self.data[1]
    }

    pub fn set_blue(&mut self, b: u8) {
        self.data[1] = b;
    }

    pub fn green(&self) -> u8 {
        self.data[2]
    }

    pub fn set_green(&mut self, g: u8) {
        self.data[2] = g;
    }

    pub fn alpha(&self) -> u8 {
        self.data[3]
    }

    #[allow(dead_code)]
    pub fn set_alpha(&mut self, a: u8) {
        self.data[3] = a;
    }

    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.set_red(r);
        self.set_green(g);
        self.set_blue(b);
    }
}

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<Vec<Pixel>>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            bytes: vec![vec![Pixel::new(); width as usize]; height as usize],
        }
    }

    pub fn at(&self, x: u32, y: u32) -> &Pixel {
        &self.bytes[y as usize][x as usize]
    }

    pub fn at_mut(&mut self, x: u32, y: u32) -> &mut Pixel {
        &mut self.bytes[y as usize][x as usize]
    }

    pub fn rgba_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![0 as u8; (self.width * self.height) as usize * 4];
        let mut offset = 0;
        for i in (0..self.height).rev() {
            for j in 0..self.width {
                let p = self.at(j, i);
                buffer[offset] = p.red();
                buffer[offset + 1] = p.green();
                buffer[offset + 2] = p.blue();
                buffer[offset + 3] = p.alpha();
                offset += 4
            }
        }
        buffer
    }
}
