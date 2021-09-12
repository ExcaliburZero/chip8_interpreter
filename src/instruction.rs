use crate::ram::Address;

pub const INSTRUCTION_SIZE_BYTES: u16 = 2;

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    ClearDisplay(),
    SetIndexRegister(Address),
}

impl Instruction {
    pub fn from_u16(bytes: u16) -> Result<Instruction, String> {
        use Instruction::*;

        match bytes {
            0x00E0 => Ok(ClearDisplay()),
            _ => {
                let opcode = Instruction::get_opcode(bytes);
                match opcode {
                    0xA => {
                        let address = Instruction::get_address(bytes);
                        Ok(SetIndexRegister(address))
                    }
                    _ => Err(format!("Unrecognized instruction: 0x{:x}", bytes)),
                }
            }
        }
    }

    fn get_opcode(bytes: u16) -> u8 {
        // Get the first nibble
        let three_nibbles_len = 4 * 3;
        ((bytes & 0xF000) >> three_nibbles_len) as u8
    }

    fn get_address(bytes: u16) -> Address {
        // Get the last three nibbles
        bytes & 0x0FFF
    }
}

#[test]
fn instruction_from_u16() {
    use Instruction::*;

    assert_eq!(Ok(ClearDisplay()), Instruction::from_u16(0x00E0));
    assert_eq!(Ok(SetIndexRegister(0x22A)), Instruction::from_u16(0xA22A));
}
