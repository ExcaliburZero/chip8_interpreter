pub const INSTRUCTION_SIZE_BYTES: u16 = 2;

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    ClearDisplay(),
}

impl Instruction {
    pub fn from_u16(bytes: u16) -> Result<Instruction, String> {
        use Instruction::*;

        match bytes {
            0x00E0 => Ok(ClearDisplay()),
            _ => Err(format!("Unrecognized instruction: 0x{:x}", bytes)),
        }
    }
}

#[test]
fn instruction_from_u16() {
    use Instruction::*;

    assert_eq!(Ok(ClearDisplay()), Instruction::from_u16(0x00E0));
}
