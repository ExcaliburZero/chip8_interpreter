use crate::bit_operations;
use crate::ram::Address;

pub const INSTRUCTION_SIZE_BYTES: u16 = 2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    ClearDisplay(),                             // 0x00E0
    Return(),                                   // 0x00EE
    Jump(Address),                              // 0x1NNN
    Call(Address),                              // 0x2NNN
    JumpIfEqValue(Register, u8),                // 0x3XNN
    JumpIfNotEqValue(Register, u8),             // 0x4XNN
    JumpIfRegistersEq(Register, Register),      // 0x5XY0
    SetRegister(Register, u8),                  // 0x6XNN
    IncrementRegister(Register, u8),            // 0x7XNN
    CopyRegister(Register, Register),           // 0x8XY0
    BitwiseOr(Register, Register),              // 0x8XY1
    BitwiseAnd(Register, Register),             // 0x8XY2
    BitwiseXor(Register, Register),             // 0x8XY3
    IncrementByRegister(Register, Register),    // 0x8XY4
    DecrementByRegister(Register, Register),    // 0x8XY5
    RightShift(Register),                       // 0x8XY6
    DecrementByRegisterRev(Register, Register), // 0x8XY7
    LeftShift(Register),                        // 0x8XYE
    JumpIfRegistersNotEq(Register, Register),   // 0x9XY0
    SetIndexRegister(Address),                  // 0xANNN
    SetRandomAnd(Register, u8),                 // 0xCXNN
    DrawSprite(Register, Register, u8),         // 0xDXYN
    SkipIfNotPressed(Register),                 // 0xEXA1
    GetDelayTimer(Register),                    // 0xFX07
    SetDelayTimer(Register),                    // 0xFX15
    SetSoundTimer(Register),                    // 0xFX18
    IncrementIndexByRegister(Register),         // 0xFX1E
    GetFontCharacter(Register),                 // 0xFX29
    StoreBinCodedDec(Register),                 // 0xFX33
    DumpRegisters(Register),                    // 0xFX55
    LoadRegisters(Register),                    // 0xFX65
}

impl Instruction {
    pub fn from_u16(bytes: u16) -> Result<Instruction, String> {
        use Instruction::*;

        match bit_operations::break_into_nibbles(bytes) {
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
            (0x8, a, b, 0x7) => {
                let first_register = Register::from_nibble(a);
                let second_register = Register::from_nibble(b);

                Ok(DecrementByRegisterRev(first_register, second_register))
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
            (0xC, a, _, _) => {
                let register = Register::from_nibble(a);
                let value = Instruction::get_value(bytes);

                Ok(SetRandomAnd(register, value))
            }
            (0xD, a, b, c) => {
                let x_register = Register::from_nibble(a);
                let y_register = Register::from_nibble(b);
                let height = c;

                Ok(DrawSprite(x_register, y_register, height))
            }
            (0xE, a, 0xA, 0x1) => {
                let register = Register::from_nibble(a);

                Ok(SkipIfNotPressed(register))
            }
            (0xF, a, 0x0, 0x7) => {
                let register = Register::from_nibble(a);

                Ok(GetDelayTimer(register))
            }
            (0xF, a, 0x1, 0x5) => {
                let register = Register::from_nibble(a);

                Ok(SetDelayTimer(register))
            }
            (0xF, a, 0x1, 0x8) => {
                let register = Register::from_nibble(a);

                Ok(SetSoundTimer(register))
            }
            (0xF, a, 0x1, 0xE) => {
                let register = Register::from_nibble(a);

                Ok(IncrementIndexByRegister(register))
            }
            (0xF, a, 0x2, 0x9) => {
                let register = Register::from_nibble(a);

                Ok(GetFontCharacter(register))
            }
            (0xF, a, 0x3, 0x3) => {
                let register = Register::from_nibble(a);

                Ok(StoreBinCodedDec(register))
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
        bit_operations::get_last_three_nibbles(bytes)
    }

    fn get_value(bytes: u16) -> u8 {
        bit_operations::get_last_two_nibbles(bytes)
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
    assert_eq!(Ok(Return()), Instruction::from_u16(0x00EE));
    assert_eq!(Ok(Jump(0x0123)), Instruction::from_u16(0x1123));
    assert_eq!(Ok(Call(0x0123)), Instruction::from_u16(0x2123));
    assert_eq!(
        Ok(JumpIfEqValue(Register::V2, 0x12)),
        Instruction::from_u16(0x3212)
    );
    assert_eq!(
        Ok(JumpIfNotEqValue(Register::V2, 0x12)),
        Instruction::from_u16(0x4212)
    );
    assert_eq!(
        Ok(JumpIfRegistersEq(Register::V2, Register::V3)),
        Instruction::from_u16(0x5230)
    );
    assert_eq!(Ok(SetRegister(V1, 0x23)), Instruction::from_u16(0x6123));
    assert_eq!(
        Ok(IncrementRegister(Register::V4, 0x12)),
        Instruction::from_u16(0x7412)
    );
    assert_eq!(
        Ok(CopyRegister(Register::V5, Register::V6)),
        Instruction::from_u16(0x8560)
    );
    assert_eq!(
        Ok(BitwiseOr(Register::V7, Register::V8)),
        Instruction::from_u16(0x8781)
    );
    assert_eq!(
        Ok(BitwiseAnd(Register::V9, Register::Va)),
        Instruction::from_u16(0x89A2)
    );
    assert_eq!(
        Ok(BitwiseXor(Register::Vb, Register::Vc)),
        Instruction::from_u16(0x8BC3)
    );
    assert_eq!(
        Ok(IncrementByRegister(Register::Vd, Register::Ve)),
        Instruction::from_u16(0x8DE4)
    );
    assert_eq!(
        Ok(DecrementByRegister(Register::Vf, Register::Ve)),
        Instruction::from_u16(0x8FE5)
    );
    assert_eq!(Ok(RightShift(Register::V1)), Instruction::from_u16(0x8106));
    assert_eq!(
        Ok(DecrementByRegisterRev(Register::V1, Register::V2)),
        Instruction::from_u16(0x8127)
    );
    assert_eq!(Ok(LeftShift(Register::V1)), Instruction::from_u16(0x810E));
    assert_eq!(
        Ok(JumpIfRegistersNotEq(Register::V1, Register::V2)),
        Instruction::from_u16(0x9120)
    );
    assert_eq!(Ok(SetIndexRegister(0x22A)), Instruction::from_u16(0xA22A));
    assert_eq!(
        Ok(SetRandomAnd(Register::V1, 0x23)),
        Instruction::from_u16(0xC123)
    );
    assert_eq!(
        Ok(DrawSprite(Register::V1, Register::V2, 0x03)),
        Instruction::from_u16(0xD123)
    );
    assert_eq!(
        Ok(SkipIfNotPressed(Register::V1)),
        Instruction::from_u16(0xE1A1)
    );
    assert_eq!(
        Ok(GetDelayTimer(Register::V1)),
        Instruction::from_u16(0xF107)
    );
    assert_eq!(
        Ok(SetDelayTimer(Register::V1)),
        Instruction::from_u16(0xF115)
    );
    assert_eq!(
        Ok(SetSoundTimer(Register::V1)),
        Instruction::from_u16(0xF118)
    );
    assert_eq!(
        Ok(IncrementIndexByRegister(Register::V1)),
        Instruction::from_u16(0xF11E)
    );
    assert_eq!(
        Ok(GetFontCharacter(Register::V1)),
        Instruction::from_u16(0xF129)
    );
    assert_eq!(
        Ok(StoreBinCodedDec(Register::V1)),
        Instruction::from_u16(0xF133)
    );
    assert_eq!(
        Ok(DumpRegisters(Register::V1)),
        Instruction::from_u16(0xF155)
    );
    assert_eq!(
        Ok(LoadRegisters(Register::V1)),
        Instruction::from_u16(0xF165)
    );
}
