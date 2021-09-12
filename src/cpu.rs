use crate::instruction::{Instruction, Register, INSTRUCTION_SIZE_BYTES};
use crate::ram;
use crate::ram::Address;
use crate::screen::{AnyPixelsUnset, Position, Screen};

// From: https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#font
const DEFAULT_FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const FONT_ADDRESS: Address = 0x0050;
const ROM_ADDRESS: Address = 0x0200;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CPU {
    registers: Registers,
    ram: ram::RAM,
    pub screen: Screen,
}

impl CPU {
    pub fn load_default_font(&mut self) -> Result<(), String> {
        self.ram.write_bytes(FONT_ADDRESS, &DEFAULT_FONT)
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), String> {
        self.ram.write_bytes(ROM_ADDRESS, rom)
    }

    pub fn initialize_program_counter(&mut self) {
        self.registers.program_counter = ROM_ADDRESS;
    }

    pub fn step(&mut self) -> Result<ScreenChanged, String> {
        let instruction_bytes = self.fetch()?;
        let instruction = self.decode(instruction_bytes)?;
        self.execute(&instruction)
    }

    fn fetch(&self) -> Result<u16, String> {
        self.ram.read_u16(self.registers.program_counter)
    }

    fn decode(&self, instruction_bytes: u16) -> Result<Instruction, String> {
        Instruction::from_u16(instruction_bytes)
    }

    fn execute(&mut self, instruction: &Instruction) -> Result<ScreenChanged, String> {
        use Instruction::*;

        println!("Inst: {:?}", instruction);

        // Increment the program counter to look at the next instruction. Jump instructions will
        // overwrite this change with their jump destination.
        self.registers.program_counter += INSTRUCTION_SIZE_BYTES;

        match instruction {
            // 0x00E0
            ClearDisplay() => {
                self.screen.clear();
                Ok(ScreenChanged::Changed)
            }
            // 0x6XNN
            SetRegister(register, value) => {
                self.registers.set_register(register, *value);
                Ok(ScreenChanged::NoChange)
            }
            // 0x7XNN
            IncrementRegister(register, increment) => {
                let prev_value = self.registers.get_register(register);
                self.registers
                    .set_register(register, prev_value + increment);
                Ok(ScreenChanged::NoChange)
            }
            // 0xANNN
            SetIndexRegister(address) => {
                self.registers.index_register = *address;
                Ok(ScreenChanged::NoChange)
            }
            // 0xDXYN
            DrawSprite(x_register, y_register, height) => {
                let x = self.registers.get_register(x_register);
                let y = self.registers.get_register(y_register);
                let position = Position::new(x, y);

                let bytes = self
                    .ram
                    .read_sprite(self.registers.index_register, *height)?;

                let any_pixels_unset = self.screen.draw_sprite(&position, &bytes)?;
                self.registers.vf = match any_pixels_unset {
                    AnyPixelsUnset::Yes => 1,
                    AnyPixelsUnset::No => 0,
                };

                Ok(ScreenChanged::Changed)
            }
            i => panic!("Unhandled instruction: {:?}", i),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ScreenChanged {
    Changed,
    NoChange,
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Registers {
    program_counter: Address,
    index_register: Address,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
}

impl Registers {
    fn set_register(&mut self, register: &Register, value: u8) {
        use Register::*;

        match register {
            V0 => self.v0 = value,
            V1 => self.v1 = value,
            V2 => self.v2 = value,
            V3 => self.v3 = value,
            V4 => self.v4 = value,
            V5 => self.v5 = value,
            V6 => self.v6 = value,
            V7 => self.v7 = value,
            V8 => self.v8 = value,
            V9 => self.v9 = value,
            Va => self.va = value,
            Vb => self.vb = value,
            Vc => self.vc = value,
            Vd => self.vd = value,
            Ve => self.ve = value,
            Vf => self.vf = value,
        }
    }

    fn get_register(&self, register: &Register) -> u8 {
        use Register::*;

        match register {
            V0 => self.v0,
            V1 => self.v1,
            V2 => self.v2,
            V3 => self.v3,
            V4 => self.v4,
            V5 => self.v5,
            V6 => self.v6,
            V7 => self.v7,
            V8 => self.v8,
            V9 => self.v9,
            Va => self.va,
            Vb => self.vb,
            Vc => self.vc,
            Vd => self.vd,
            Ve => self.ve,
            Vf => self.vf,
        }
    }
}

#[test]
fn cpu_load_default_font() {
    let mut cpu = CPU::default();

    assert_eq!(Ok(0x00), cpu.ram.read_byte(FONT_ADDRESS));

    assert_eq!(Ok(()), cpu.load_default_font());

    for i in 0..80 {
        assert_eq!(
            Ok(DEFAULT_FONT[i]),
            cpu.ram.read_byte(FONT_ADDRESS + (i as u16))
        );
    }
}
