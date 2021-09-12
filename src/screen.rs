#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pixel {
    On,
    Off,
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel::Off
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Screen {
    pixels: [[Pixel; 64]; 32],
}

impl Screen {
    pub fn clear(&mut self) {
        for row in self.pixels.iter_mut() {
            row.fill(Pixel::Off);
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            pixels: [[Pixel::default(); 64]; 32],
        }
    }
}
