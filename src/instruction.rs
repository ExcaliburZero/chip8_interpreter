use crate::ram::Address;

pub const INSTRUCTION_SIZE_BYTES: u16 = 2;

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    ClearDisplay(),                     // 0x00E0
    Jump(Address),                      // 0x1NNN
    JumpIfEqValue(Register, u8),        // 0x3XNN
    SetRegister(Register, u8),          // 0x6XNN
    IncrementRegister(Register, u8),    // 0x7XNN
    SetIndexRegister(Address),          // 0xANNN
    DrawSprite(Register, Register, u8), // 0xDXYN
}

impl Instruction {
    pub fn from_u16(bytes: u16) -> Result<Instruction, String> {
        use Instruction::*;

        match bytes {
            0x00E0 => Ok(ClearDisplay()),
            _ => {
                let opcode = Instruction::get_opcode(bytes);
                match opcode {
                    0x1 => {
                        let address = Instruction::get_address(bytes);
                        Ok(Jump(address))
                    }
                    0x3 => {
                        let register = Register::from_nibble(Instruction::get_second_nibble(bytes));
                        let value = Instruction::get_value(bytes);

                        Ok(JumpIfEqValue(register, value))
                    }
                    0x6 => {
                        let register = Register::from_nibble(Instruction::get_second_nibble(bytes));
                        let value = Instruction::get_value(bytes);

                        Ok(SetRegister(register, value))
                    }
                    0x7 => {
                        let register = Register::from_nibble(Instruction::get_second_nibble(bytes));
                        let value = Instruction::get_value(bytes);

                        Ok(IncrementRegister(register, value))
                    }
                    0xA => {
                        let address = Instruction::get_address(bytes);
                        Ok(SetIndexRegister(address))
                    }
                    0xD => {
                        let x_register =
                            Register::from_nibble(Instruction::get_second_nibble(bytes));
                        let y_register =
                            Register::from_nibble(Instruction::get_third_nibble(bytes));
                        let height = Instruction::get_fourth_nibble(bytes);

                        Ok(DrawSprite(x_register, y_register, height))
                    }
                    _ => Err(format!("Unrecognized instruction: 0x{:x}", bytes)),
                }
            }
        }
    }

    fn get_opcode(bytes: u16) -> u8 {
        Instruction::get_first_nibble(bytes)
    }

    fn get_address(bytes: u16) -> Address {
        // Get the last three nibbles
        bytes & 0x0FFF
    }

    fn get_first_nibble(bytes: u16) -> u8 {
        // Get the first nibble
        let three_nibbles_len = 4 * 3;
        ((bytes & 0xF000) >> three_nibbles_len) as u8
    }

    fn get_second_nibble(bytes: u16) -> u8 {
        // Get the second nibble
        let two_nibbles_len = 4 * 2;
        ((bytes & 0x0F00) >> two_nibbles_len) as u8
    }

    fn get_third_nibble(bytes: u16) -> u8 {
        // Get the third nibble
        let one_nibble_len = 4;
        ((bytes & 0x00F0) >> one_nibble_len) as u8
    }

    fn get_fourth_nibble(bytes: u16) -> u8 {
        // Get the last nibble
        (bytes & 0x000F) as u8
    }

    fn get_value(bytes: u16) -> u8 {
        // Get the last two nibbles
        (bytes & 0x00FF) as u8
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    Va,
    Vb,
    Vc,
    Vd,
    Ve,
    Vf,
}

impl Register {
    fn from_nibble(nibble: u8) -> Register {
        use Register::*;

        match nibble {
            0x0 => V0,
            0x1 => V1,
            0x2 => V2,
            0x3 => V3,
            0x4 => V4,
            0x5 => V5,
            0x6 => V6,
            0x7 => V7,
            0x8 => V8,
            0x9 => V9,
            0xA => Va,
            0xB => Vb,
            0xC => Vc,
            0xD => Vd,
            0xE => Ve,
            0xF => Vf,
            _ => {
                panic!(
                    "Register id is too large to be a nibble: {} (0x{:x})",
                    nibble, nibble
                )
            }
        }
    }
}

#[test]
fn instruction_from_u16() {
    use Instruction::*;
    use Register::*;

    assert_eq!(Ok(ClearDisplay()), Instruction::from_u16(0x00E0));
    assert_eq!(Ok(SetIndexRegister(0x22A)), Instruction::from_u16(0xA22A));
    assert_eq!(Ok(SetRegister(V1, 0x23)), Instruction::from_u16(0x6123));
}
