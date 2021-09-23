extern crate minifb;

use std::io::Write;

use minifb::{Key, Window, WindowOptions};

use crate::screen::{Pixel, Screen};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputState {
    Pressed,
    NotPressed,
}

impl InputState {
    pub fn from_bool(value: bool) -> InputState {
        match value {
            true => InputState::Pressed,
            false => InputState::NotPressed,
        }
    }
}

impl Default for InputState {
    fn default() -> Self {
        InputState::NotPressed
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Inputs {
    zero: InputState,
    seven: InputState,
    eight: InputState,
    nine: InputState,
}

impl Inputs {
    fn set_input(&mut self, input: &InputKey, value: InputState) {
        use InputKey::*;

        match input {
            Zero => self.zero = value,
            Seven => self.seven = value,
            Eight => self.eight = value,
            Nine => self.nine = value,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InputKey {
    Zero,
    Eight,
}

impl Inputs {
    pub fn get_input(&self, key_id: u8) -> Result<InputState, String> {
        match key_id {
            0 => Ok(self.zero),
            7 => Ok(self.seven),
            8 => Ok(self.eight),
            9 => Ok(self.nine),
            _ => Err(format!("Unrecognized key id: 0x{:02x}", key_id)),
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum ViewState {
    Open,
    Closed,
}

pub trait View {
    fn open(&mut self, screen: &Screen);
    fn close(&mut self);
    fn update(&mut self, screen: &Screen) -> ViewState;
    fn get_inputs(&mut self) -> Result<Inputs, String>;
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

    fn update(&mut self, screen: &Screen) -> ViewState {
        self.display_screen(screen);

        ViewState::Open
    }

    fn get_inputs(&mut self) -> Result<Inputs, String> {
        // TODO: look into
        // https://www.reddit.com/r/rust/comments/c8076q/check_if_a_key_is_pressed/
        // https://github.com/redox-os/termion/blob/master/examples/keys.rs
        Ok(Inputs {
            zero: InputState::NotPressed,
            seven: InputState::NotPressed,
            eight: InputState::NotPressed,
            nine: InputState::NotPressed,
        })
    }
}

pub struct MiniFbView {
    name: String,
    width: usize,
    height: usize,
    window_options: WindowOptions,
    window: Option<Window>,
}

impl MiniFbView {
    pub fn new(
        name: String,
        width: usize,
        height: usize,
        window_options: WindowOptions,
    ) -> MiniFbView {
        MiniFbView {
            name,
            width,
            height,
            window_options,
            window: None,
        }
    }

    fn update_display(&mut self, screen: &Screen) {
        // 0x00RRGGBB
        let white = 0x00FFFFFF;
        let black = 0x00000000;

        let mut buffer: Vec<u32> = vec![0; self.width * self.height];

        for (i, b) in buffer.iter_mut().enumerate() {
            let row = (i / (64 * 2)) as usize / 2;
            let column = (i % (64 * 2)) as usize / 2;

            //println!("{} => {}, {}", i, row, column);

            *b = match screen.pixels[row][column] {
                Pixel::On => black,
                Pixel::Off => white,
            };
        }

        self.window
            .as_mut()
            .unwrap()
            .update_with_buffer(&buffer, self.width, self.height)
            .unwrap();
    }
}

impl View for MiniFbView {
    fn open(&mut self, screen: &Screen) {
        self.window =
            Some(Window::new(&self.name, self.width, self.height, self.window_options).unwrap());
    }

    fn close(&mut self) {
        self.window = None;
    }

    fn update(&mut self, screen: &Screen) -> ViewState {
        if self.window.is_none() {
            return ViewState::Closed;
        }
        if !self.window.as_ref().unwrap().is_open() {
            return ViewState::Closed;
        }

        self.update_display(screen);

        ViewState::Open
    }

    fn get_inputs(&mut self) -> Result<Inputs, String> {
        Ok(Inputs {
            zero: InputState::from_bool(self.window.as_ref().unwrap().is_key_down(Key::Key0)),
            seven: InputState::from_bool(self.window.as_ref().unwrap().is_key_down(Key::Key7)),
            eight: InputState::from_bool(self.window.as_ref().unwrap().is_key_down(Key::Key8)),
            nine: InputState::from_bool(self.window.as_ref().unwrap().is_key_down(Key::Key9)),
        })
    }
}
