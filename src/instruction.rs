use crate::ram::Address;

pub const INSTRUCTION_SIZE_BYTES: u16 = 2;

type InstructionNibble = (u8, u8, u8, u8);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    ClearDisplay(),                           // 0x00E0
    Return(),                                 // 0x00EE
    Jump(Address),                            // 0x1NNN
    Call(Address),                            // 0x2NNN
    JumpIfEqValue(Register, u8),              // 0x3XNN
    JumpIfNotEqValue(Register, u8),           // 0x4XNN
    JumpIfRegistersEq(Register, Register),    // 0x5XY0
    SetRegister(Register, u8),                // 0x6XNN
    IncrementRegister(Register, u8),          // 0x7XNN
    CopyRegister(Register, Register),         // 0x8XY0
    BitwiseOr(Register, Register),            // 0x8XY1
    BitwiseAnd(Register, Register),           // 0x8XY2
    BitwiseXor(Register, Register),           // 0x8XY3
    IncrementByRegister(Register, Register),  // 0x8XY4
    DecrementByRegister(Register, Register),  // 0x8XY5
    RightShift(Register),                     // 0x8XY6
    LeftShift(Register),                      // 0x8XYE
    JumpIfRegistersNotEq(Register, Register), // 0x9XY0
    SetIndexRegister(Address),                // 0xANNN
    DrawSprite(Register, Register, u8),       // 0xDXYN
    SetDelayTimer(Register),                  // 0xFX15
    DumpRegisters(Register),                  // 0xFX55
    LoadRegisters(Register),                  // 0xFX65
}

impl Instruction {
    pub fn from_u16(bytes: u16) -> Result<Instruction, String> {
        use Instruction::*;

        match Instruction::break_into_nibbles(bytes) {
            (0x0, 0x0, 0xE, 0x0) => Ok(ClearDisplay()),
            (0x0, 0x0, 0xE, 0xE) => Ok(Return()),
            (0x1, _, _, _) => {
                let address = Instruction::get_address(bytes);
                Ok(Jump(address))
            }
            (0x2, _, _, _) => {
                let address = Instruction::get_address(bytes);
                Ok(Call(address))
            }
            (0x3, a, _, _) => {
                let register = Register::from_nibble(a);
                let value = Instruction::get_value(bytes);

                Ok(JumpIfEqValue(register, value))
            }
            (0x4, a, _, _) => {
                let register = Register::from_nibble(a);
                let value = Instruction::get_value(bytes);

                Ok(JumpIfNotEqValue(register, value))
            }
            (0x5, a, b, 0x0) => {
                let first_register = Register::from_nibble(a);
                let second_register = Register::from_nibble(b);

                Ok(JumpIfRegistersEq(first_register, second_register))
            }
            (0x6, a, _, _) => {
                let register = Register::from_nibble(a);
                let value = Instruction::get_value(bytes);

                Ok(SetRegister(register, value))
            }
            (0x7, a, _, _) => {
                let register = Register::from_nibble(a);
                let value = Instruction::get_value(bytes);

                Ok(IncrementRegister(register, value))
            }
            (0x8, a, b, 0x0) => {
                let first_register = Register::from_nibble(a);
                let second_register = Register::from_nibble(b);

                Ok(CopyRegister(first_register, second_register))
            }
            (0x8, a, b, 0x1) => {
                let first_register = Register::from_nibble(a);
                let second_register = Register::from_nibble(b);

                Ok(BitwiseOr(first_register, second_register))
            }
            (0x8, a, b, 0x2) => {
                let first_register = Register::from_nibble(a);
                let second_register = Register::from_nibble(b);

                Ok(BitwiseAnd(first_register, second_register))
            }
            (0x8, a, b, 0x3) => {
                let first_register = Register::from_nibble(a);
                let second_register = Register::from_nibble(b);

                Ok(BitwiseXor(first_register, second_register))
            }
            (0x8, a, b, 0x4) => {
                let first_register = Register::from_nibble(a);
                let second_register = Register::from_nibble(b);

                Ok(IncrementByRegister(first_register, second_register))
            }
            (0x8, a, b, 0x5) => {
                let first_register = Register::from_nibble(a);
                let second_register = Register::from_nibble(b);

                Ok(DecrementByRegister(first_register, second_register))
            }
            (0x8, a, _, 0x6) => {
                let register = Register::from_nibble(a);

                Ok(RightShift(register))
            }
            (0x8, a, _, 0xE) => {
                let register = Register::from_nibble(a);

                Ok(LeftShift(register))
            }
            (0x9, a, b, 0x0) => {
                let first_register = Register::from_nibble(a);
                let second_register = Register::from_nibble(b);

                Ok(JumpIfRegistersNotEq(first_register, second_register))
            }
            (0xA, _, _, _) => {
                let address = Instruction::get_address(bytes);
                Ok(SetIndexRegister(address))
            }
            (0xD, a, b, c) => {
                let x_register = Register::from_nibble(a);
                let y_register = Register::from_nibble(b);
                let height = c;

                Ok(DrawSprite(x_register, y_register, height))
            }
            (0xF, a, 0x1, 0x5) => {
                let register = Register::from_nibble(a);

                Ok(SetDelayTimer(register))
            }
            (0xF, a, 0x5, 0x5) => {
                let register = Register::from_nibble(a);

                Ok(DumpRegisters(register))
            }
            (0xF, a, 0x6, 0x5) => {
                let register = Register::from_nibble(a);

                Ok(LoadRegisters(register))
            }
            _ => Err(format!("Unrecognized instruction: 0x{:04x}", bytes)),
        }
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

    fn break_into_nibbles(bytes: u16) -> InstructionNibble {
        let first = Instruction::get_first_nibble(bytes);
        let second = Instruction::get_second_nibble(bytes);
        let third = Instruction::get_third_nibble(bytes);
        let fourth = Instruction::get_fourth_nibble(bytes);

        (first, second, third, fourth)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

    fn to_nibble(self) -> u8 {
        use Register::*;

        match self {
            V0 => 0x0,
            V1 => 0x1,
            V2 => 0x2,
            V3 => 0x3,
            V4 => 0x4,
            V5 => 0x5,
            V6 => 0x6,
            V7 => 0x7,
            V8 => 0x8,
            V9 => 0x9,
            Va => 0xA,
            Vb => 0xB,
            Vc => 0xC,
            Vd => 0xD,
            Ve => 0xE,
            Vf => 0xF,
        }
    }

    pub fn inclusive_range(start: &Register, end: &Register) -> Result<Vec<Register>, String> {
        let start_nibble = start.to_nibble();
        let end_nibble = end.to_nibble();

        if end_nibble < start_nibble {
            return Err(format!("Invalid register range: {:?} - {:?}", start, end));
        }

        Ok((start_nibble..=end_nibble)
            .map(Register::from_nibble)
            .collect())
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
