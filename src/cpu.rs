use crate::instruction::{Instruction, INSTRUCTION_SIZE_BYTES};
use crate::ram;
use crate::ram::Address;
use crate::screen::Screen;

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
    screen: Screen,
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

    pub fn step(&mut self) -> Result<(), String> {
        let instruction_bytes = self.fetch()?;
        let instruction = self.decode(instruction_bytes)?;
        self.execute(&instruction)?;

        Ok(())
    }

    fn fetch(&self) -> Result<u16, String> {
        self.ram.read_u16(self.registers.program_counter)
    }

    fn decode(&self, instruction_bytes: u16) -> Result<Instruction, String> {
        Instruction::from_u16(instruction_bytes)
    }

    fn execute(&mut self, instruction: &Instruction) -> Result<(), String> {
        use Instruction::*;

        println!("Inst: {:?}", instruction);

        // Increment the program counter to look at the next instruction. Jump instructions will
        // overwrite this change with their jump destination.
        self.registers.program_counter += INSTRUCTION_SIZE_BYTES;

        match instruction {
            ClearDisplay() => self.screen.clear(),
            i => panic!("Unhandled instruction: {:?}", i),
        }

        Ok(())
    }
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
