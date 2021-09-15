pub type Address = u16;

#[derive(Debug, Eq, PartialEq)]
pub struct RAM {
    memory: [u8; 4096],
}

impl Default for RAM {
    fn default() -> Self {
        RAM { memory: [0; 4096] }
    }
}

impl RAM {
    pub fn write_bytes(&mut self, address: Address, bytes: &[u8]) -> Result<(), String> {
        for (i, byte) in bytes.iter().enumerate() {
            let cur_address = address + (i as u16);
            self.write_byte(cur_address, *byte)?
        }

        Ok(())
    }

    pub fn write_byte(&mut self, address: Address, byte: u8) -> Result<(), String> {
        if address >= 4096 {
            return Err(format!("Write at invalid memory address: 0x{:x}", address));
        }

        self.memory[address as usize] = byte;
        Ok(())
    }

    pub fn read_byte(&self, address: Address) -> Result<u8, String> {
        if address >= 4096 {
            return Err(format!("Read at invalid memory address: 0x{:x}", address));
        }

        Ok(self.memory[address as usize])
    }

    pub fn read_u16(&self, address: Address) -> Result<u16, String> {
        let first_byte = self.read_byte(address)? as u16;
        let second_byte = self.read_byte(address + 1)? as u16;

        Ok((first_byte << 8) | second_byte)
    }

    pub fn read_sprite(&self, address: Address, height: u8) -> Result<Vec<u8>, String> {
        // Note: Sprite width is always 8 pixels and data is encoded as each byte is a row of the
        // sprite with 0=transparent and 1=filled.
        let mut sprite_bytes = vec![];
        for i in 0..height {
            sprite_bytes.push(self.read_byte(address + i as u16)?);
        }

        Ok(sprite_bytes)
    }
}

#[test]
fn ram_starts_empty() {
    let ram = RAM::default();

    assert_eq!(Ok(0x00), ram.read_byte(0x0000));
    assert_eq!(Ok(0x00), ram.read_byte(0x050));
}

#[test]
fn ram_write_and_read_byte() {
    let mut ram = RAM::default();

    assert_eq!(Ok(0x00), ram.read_byte(0x0000));

    assert_eq!(Ok(()), ram.write_byte(0x00, 0x42));

    assert_eq!(Ok(0x42), ram.read_byte(0x0000));
}

#[test]
fn ram_read_byte_invalid_memory_address() {
    let ram = RAM::default();

    let expected = Err("Read at invalid memory address: 0x1000".to_string());
    assert_eq!(expected, ram.read_byte(0x1000));
}

#[test]
fn ram_write_byte_invalid_memory_address() {
    let mut ram = RAM::default();

    let expected = Err("Write at invalid memory address: 0x1000".to_string());
    assert_eq!(expected, ram.write_byte(0x1000, 0xFF));
}

#[test]
fn ram_read_u16() {
    let mut ram = RAM::default();

    assert_eq!(Ok(()), ram.write_byte(0x0000, 0x12));
    assert_eq!(Ok(()), ram.write_byte(0x0001, 0x34));

    assert_eq!(Ok(0x1234), ram.read_u16(0x0000));
}
