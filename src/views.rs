use std::io::Write;

use crate::screen::{Pixel, Screen};

pub trait View {
    fn open(&mut self, screen: &Screen);
    fn close(&mut self);
    fn update(&mut self, screen: &Screen);
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
}
