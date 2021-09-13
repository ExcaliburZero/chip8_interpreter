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

        // Increment the program counter to look at the next instruction. Jump instructions will
        // overwrite this change with their jump destination.
        self.registers.program_counter += INSTRUCTION_SIZE_BYTES;

        match instruction {
            // 0x00E0
            ClearDisplay() => {
                self.screen.clear();
                Ok(ScreenChanged::Changed)
            }
            // 0x00EE
            Return() => match self.registers.stack.pop() {
                Some(address) => {
                    self.registers.program_counter = address;
                    Ok(ScreenChanged::NoChange)
                }
                None => Err("No address on the stack to return to.".to_string()),
            },
            // 0x1NNN
            Jump(address) => {
                self.registers.program_counter = *address;
                Ok(ScreenChanged::NoChange)
            }
            // 0x2NNN
            Call(address) => {
                self.registers.stack.push(self.registers.program_counter);
                self.registers.program_counter = *address;
                Ok(ScreenChanged::NoChange)
            }
            // 0x3XNN
            JumpIfEqValue(register, value) => {
                if self.registers.get_register(register) == *value {
                    self.registers.program_counter += INSTRUCTION_SIZE_BYTES;
                }

                Ok(ScreenChanged::NoChange)
            }
            // 0x4XNN
            JumpIfNotEqValue(register, value) => {
                if self.registers.get_register(register) != *value {
                    self.registers.program_counter += INSTRUCTION_SIZE_BYTES;
                }

                Ok(ScreenChanged::NoChange)
            }
            // 0x5XY0
            JumpIfRegistersEq(first_register, second_register) => {
                let a = self.registers.get_register(first_register);
                let b = self.registers.get_register(second_register);
                if a == b {
                    self.registers.program_counter += INSTRUCTION_SIZE_BYTES;
                }

                Ok(ScreenChanged::NoChange)
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
            // 0x8XY0
            CopyRegister(first_register, second_register) => {
                let value = self.registers.get_register(second_register);
                self.registers.set_register(first_register, value);
                Ok(ScreenChanged::NoChange)
            }
            // 0x8XY1
            BitwiseOr(first_register, second_register) => {
                let a = self.registers.get_register(first_register);
                let b = self.registers.get_register(second_register);

                self.registers.set_register(first_register, a | b);
                Ok(ScreenChanged::NoChange)
            }
            // 0x8XY2
            BitwiseAnd(first_register, second_register) => {
                let a = self.registers.get_register(first_register);
                let b = self.registers.get_register(second_register);

                self.registers.set_register(first_register, a & b);
                Ok(ScreenChanged::NoChange)
            }
            // 0x8XY3
            BitwiseXor(first_register, second_register) => {
                let a = self.registers.get_register(first_register);
                let b = self.registers.get_register(second_register);

                self.registers.set_register(first_register, a ^ b);
                Ok(ScreenChanged::NoChange)
            }
            // 0x8XY4
            IncrementByRegister(first_register, second_register) => {
                let a = self.registers.get_register(first_register);
                let b = self.registers.get_register(second_register);

                let (value, overflowed) = u8::overflowing_add(a, b);
                self.registers.set_register(first_register, value);

                self.registers.vf = match overflowed {
                    true => 1,
                    false => 0,
                };

                Ok(ScreenChanged::NoChange)
            }
            // 0x8XY5
            DecrementByRegister(first_register, second_register) => {
                let a = self.registers.get_register(first_register);
                let b = self.registers.get_register(second_register);

                let (value, overflowed) = u8::overflowing_sub(a, b);
                self.registers.set_register(first_register, value);

                self.registers.vf = match overflowed {
                    true => 0,
                    false => 1,
                };

                Ok(ScreenChanged::NoChange)
            }
            // 0x8XY6
            RightShift(register) => {
                let a = self.registers.get_register(register);
                let value = a >> 1;
                let underflowed_bit = (a & 0x01) as u8;

                self.registers.set_register(register, value);
                self.registers.vf = underflowed_bit;

                Ok(ScreenChanged::NoChange)
            }
            // 0x8XY5
            DecrementByRegisterRev(first_register, second_register) => {
                let a = self.registers.get_register(first_register);
                let b = self.registers.get_register(second_register);

                let (value, overflowed) = u8::overflowing_sub(b, a);
                self.registers.set_register(first_register, value);

                self.registers.vf = match overflowed {
                    true => 0,
                    false => 1,
                };

                Ok(ScreenChanged::NoChange)
            }
            // 0x8XYE
            LeftShift(register) => {
                let a = self.registers.get_register(register);
                let value = a << 1;
                let overflowed_bit = ((a & 0x80) >> 7) as u8;

                self.registers.set_register(register, value);
                self.registers.vf = overflowed_bit;

                Ok(ScreenChanged::NoChange)
            }
            // 0x9XY0
            JumpIfRegistersNotEq(first_register, second_register) => {
                let a = self.registers.get_register(first_register);
                let b = self.registers.get_register(second_register);
                if a != b {
                    self.registers.program_counter += INSTRUCTION_SIZE_BYTES;
                }

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
            // 0xFX15
            SetDelayTimer(register) => {
                let value = self.registers.get_register(register);

                self.registers.delay_timer = value;
                Ok(ScreenChanged::NoChange)
            }
            // 0xFX1E
            IncrementIndexByRegister(register) => {
                let value = self.registers.get_register(register);

                self.registers.index_register += value as u16;
                Ok(ScreenChanged::NoChange)
            }
            // 0xFX29
            GetFontCharacter(register) => {
                let value = self.registers.get_register(register) as u16;

                let character_address = FONT_ADDRESS + value * 5;

                self.registers.index_register = character_address;
                Ok(ScreenChanged::NoChange)
            }
            // 0xFX33
            StoreBinCodedDec(register) => {
                let value = self.registers.get_register(register);
                let base_address = self.registers.index_register;

                let hundreds_place = value / 100;
                let tens_place = (value - hundreds_place * 100) / 10;
                let ones_place = value % 10;

                self.ram.write_byte(base_address, hundreds_place)?;
                self.ram.write_byte(base_address + 1, tens_place)?;
                self.ram.write_byte(base_address + 2, ones_place)?;

                Ok(ScreenChanged::NoChange)
            }
            // 0xFX55
            DumpRegisters(last_register) => {
                let base_address = self.registers.index_register;

                for (i, register) in Register::inclusive_range(&Register::V0, last_register)?
                    .iter()
                    .enumerate()
                {
                    let value = self.registers.get_register(register);

                    let dest_address = base_address + (i as u16);
                    self.ram.write_byte(dest_address, value)?;
                }

                Ok(ScreenChanged::NoChange)
            }
            // 0xFX65
            LoadRegisters(last_register) => {
                let base_address = self.registers.index_register;

                for (i, register) in Register::inclusive_range(&Register::V0, last_register)?
                    .iter()
                    .enumerate()
                {
                    let src_address = base_address + (i as u16);
                    let value = self.ram.read_byte(src_address)?;

                    self.registers.set_register(register, value);
                }

                Ok(ScreenChanged::NoChange)
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

#[test]
fn cpu_dump_load_registers() {
    let mut cpu = CPU::default();

    assert_eq!(
        Ok(ScreenChanged::NoChange),
        cpu.execute(&Instruction::SetRegister(Register::V1, 48))
    );

    assert_eq!(
        Ok(ScreenChanged::NoChange),
        cpu.execute(&Instruction::SetIndexRegister(0x0400))
    );

    assert_eq!(
        Ok(ScreenChanged::NoChange),
        cpu.execute(&Instruction::DumpRegisters(Register::V1))
    );

    assert_eq!(
        Ok(ScreenChanged::NoChange),
        cpu.execute(&Instruction::SetIndexRegister(0x0401))
    );

    assert_eq!(
        Ok(ScreenChanged::NoChange),
        cpu.execute(&Instruction::LoadRegisters(Register::V0))
    );

    assert_eq!(Ok(0), cpu.ram.read_byte(0x0400));
    assert_eq!(Ok(48), cpu.ram.read_byte(0x0401));

    assert_eq!(48, cpu.registers.v1);
    assert_eq!(48, cpu.registers.v0);
}
