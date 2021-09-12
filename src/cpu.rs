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

const FONT_ADDRESS: Address = 0x050;

type Address = u16;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CPU {
    registers: Registers,
    ram: RAM,
}

impl CPU {
    pub fn load_default_font(&mut self) {
        self.ram.write_bytes(FONT_ADDRESS, &DEFAULT_FONT)
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

#[derive(Debug, Eq, PartialEq)]
struct RAM {
    memory: [u8; 4096],
}

impl Default for RAM {
    fn default() -> Self {
        RAM { memory: [0; 4096] }
    }
}

impl RAM {
    fn write_bytes(&mut self, address: Address, bytes: &[u8]) {
        for (i, byte) in bytes.iter().enumerate() {
            let cur_address = address + (i as u16);
            self.write_byte(cur_address, *byte)
        }
    }

    fn write_byte(&mut self, address: Address, byte: u8) {
        self.memory[address as usize] = byte;
    }

    fn read_byte(&self, address: Address) -> Result<u8, String> {
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
    assert_eq!(Ok(0x00), ram.read_byte(FONT_ADDRESS));
}

#[test]
fn ram_write_and_read_byte() {
    let mut ram = RAM::default();

    assert_eq!(Ok(0x00), ram.read_byte(0x0000));

    ram.write_byte(0x00, 0x42);

    assert_eq!(Ok(0x42), ram.read_byte(0x0000));
}

#[test]
fn cpu_load_default_font() {
    let mut cpu = CPU::default();

    assert_eq!(Ok(0x00), cpu.ram.read_byte(FONT_ADDRESS));

    cpu.load_default_font();

    for i in 0..80 {
        assert_eq!(
            Ok(DEFAULT_FONT[i]),
            cpu.ram.read_byte(FONT_ADDRESS + (i as u16))
        );
    }
}

#[test]
fn ram_read_byte_invalid_memory_address() {
    let ram = RAM::default();

    let expected = Err("Read at invalid memory address: 0x1000".to_string());
    assert_eq!(expected, ram.read_byte(0x1000));
}
