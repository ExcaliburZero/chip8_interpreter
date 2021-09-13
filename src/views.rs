use std::io::Write;

use crate::screen::{Pixel, Screen};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputState {
    Pressed,
    NotPressed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inputs {
    zero: InputState,
    eight: InputState,
}

impl Inputs {
    pub fn get_input(&self, key_id: u8) -> Result<InputState, String> {
        match key_id {
            0 => Ok(self.zero),
            8 => Ok(self.eight),
            _ => Err(format!("Unrecognized key id: 0x{:02x}", key_id)),
        }
    }
}

impl Default for Inputs {
    fn default() -> Self {
        Inputs {
            zero: InputState::NotPressed,
            eight: InputState::NotPressed,
        }
    }
}

pub trait View {
    fn open(&mut self, screen: &Screen);
    fn close(&mut self);
    fn update(&mut self, screen: &Screen);
    fn get_inputs(&mut self) -> Inputs;
}

pub struct CliView<W: Write> {
    output: W,
}

impl<W: Write> CliView<W> {
    pub fn new(output: W) -> Self {
        CliView { output }
    }

    fn display_screen(&mut self, screen: &Screen) {
        for row in screen.pixels.iter() {
            write!(self.output, "|");
            for p in row.iter() {
                match p {
                    Pixel::On => write!(self.output, "#"),
                    Pixel::Off => write!(self.output, " "),
                }
                .unwrap();
            }
            writeln!(self.output, "|");
        }
        self.output.flush().unwrap();
    }
}

impl<W: Write> View for CliView<W> {
    fn open(&mut self, screen: &Screen) {
        self.display_screen(screen)
    }

    fn close(&mut self) {}

    fn update(&mut self, screen: &Screen) {
        self.display_screen(screen)
    }

    fn get_inputs(&mut self) -> Inputs {
        Inputs {
            zero: InputState::NotPressed,
            eight: InputState::NotPressed,
        }
    }
}
