use crate::bit_operations;

const SPRITE_WIDTH: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Position {
        Position { x, y }
    }

    fn shifted(&self, x_delta: u8, y_delta: u8) -> Position {
        // TODO: do we wrap if we go over the edge?
        let new_x = self.x + x_delta;
        let new_y = self.y + y_delta;

        Position::new(new_x, new_y)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pixel {
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
    // TODO: Make this private and require accessor methods to view its contents
    pub pixels: [[Pixel; 64]; 32],
}

impl Screen {
    pub fn clear(&mut self) {
        for row in self.pixels.iter_mut() {
            row.fill(Pixel::Off);
        }
    }

    pub fn draw_sprite(
        &mut self,
        position: &Position,
        bytes: &[u8],
    ) -> Result<AnyPixelsUnset, String> {
        let height = bytes.len() as u8;

        let mut any_unset = AnyPixelsUnset::No;
        for yi in 0..height {
            for xi in 0..8 {
                let offset = Position::new(xi, yi);
                let new_position = position.shifted(xi, yi);

                if self.validate_position(&new_position).is_ok() {
                    let sprite_pixel = self.get_sprite_pixel(bytes, &offset)?;

                    if sprite_pixel == Pixel::On {
                        let old_value = self.get_value(&new_position)?;

                        match old_value {
                            Pixel::Off => self.set_value(&new_position, Pixel::On)?,
                            Pixel::On => {
                                self.set_value(&new_position, Pixel::Off)?;
                                any_unset = AnyPixelsUnset::Yes;
                            }
                        }
                    }
                }
            }
        }

        Ok(any_unset)
    }

    fn get_sprite_pixel(&self, bytes: &[u8], position: &Position) -> Result<Pixel, String> {
        if position.x >= SPRITE_WIDTH as u8 {
            return Err(format!(
                "x offset into sprite is too large: {:?} (Sprite width: {})",
                position, SPRITE_WIDTH,
            ));
        }
        if position.y >= bytes.len() as u8 {
            return Err(format!(
                "y offset into sprite is too large: {:?} (Sprite height: {})",
                position,
                bytes.len(),
            ));
        }

        // Convert from a left=0 index to a right=0 index so we can use a classic mask+shift
        // algorithm to get the correct bit
        let bit_reverse_index = 7 - position.x;

        match bit_operations::get_nth_bit(bit_reverse_index, bytes[position.y as usize])? {
            true => Ok(Pixel::On),
            false => Ok(Pixel::Off),
        }
    }

    fn validate_position(&self, position: &Position) -> Result<(), String> {
        if position.x > (self.get_width() - 1) {
            return Err(format!(
                "Screen position x value is too large: {:?}",
                position
            ));
        }

        if position.y > (self.get_height() - 1) {
            return Err(format!(
                "Screen position y value is too large: {:?}",
                position
            ));
        }
        Ok(())
    }

    pub fn get_value(&self, position: &Position) -> Result<Pixel, String> {
        self.validate_position(position)?;

        Ok(self.pixels[position.y as usize][position.x as usize])
    }

    pub fn set_value(&mut self, position: &Position, value: Pixel) -> Result<(), String> {
        self.validate_position(position)?;

        self.pixels[position.y as usize][position.x as usize] = value;

        Ok(())
    }

    pub fn get_width(&self) -> u8 {
        64
    }

    pub fn get_height(&self) -> u8 {
        32
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            pixels: [[Pixel::default(); 64]; 32],
        }
    }
}

pub enum AnyPixelsUnset {
    Yes,
    No,
}
