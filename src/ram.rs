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
    pub fn write_bytes(&mut self, address: Address, bytes: &[u8]) {
        for (i, byte) in bytes.iter().enumerate() {
            let cur_address = address + (i as u16);
            self.write_byte(cur_address, *byte)
        }
    }

    pub fn write_byte(&mut self, address: Address, byte: u8) {
        self.memory[address as usize] = byte;
    }

    pub fn read_byte(&self, address: Address) -> Result<u8, String> {
        if address >= 4096 {
            return Err(format!("Read at invalid memory address: 0x{:x}", address));
        }

        Ok(self.memory[address as usize])
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

    ram.write_byte(0x00, 0x42);

    assert_eq!(Ok(0x42), ram.read_byte(0x0000));
}

#[test]
fn ram_read_byte_invalid_memory_address() {
    let ram = RAM::default();

    let expected = Err("Read at invalid memory address: 0x1000".to_string());
    assert_eq!(expected, ram.read_byte(0x1000));
}
