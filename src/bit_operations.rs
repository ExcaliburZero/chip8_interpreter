//! Bit-based operations for u8 and u16 values.

pub type InstructionNibbles = (u8, u8, u8, u8);

/// Returns the nth bit of the given byte as a bool.
///
/// Indexed with 0 being the last bit of the byte.
///
/// ```rust
/// # use chip8_interpreter::bit_operations::get_nth_bit;
/// assert_eq!(Ok(true), get_nth_bit(1, 0b00000010));
/// assert_eq!(Ok(false), get_nth_bit(2, 0b00000010));
/// ```
pub fn get_nth_bit(n: u8, byte: u8) -> Result<bool, String> {
    if n >= 8 {
        return Err(format!("Invalid byte bit index: {}", n));
    }

    let mask = 1 << n;
    Ok(((byte & mask) >> n) == 1)
}

/// Returns the last two nibbles of the given u16.
///
/// ```rust
/// # use chip8_interpreter::bit_operations::get_last_two_nibbles;
/// assert_eq!(0xCD, get_last_two_nibbles(0xABCD));
/// ```
pub fn get_last_two_nibbles(bytes: u16) -> u8 {
    (bytes & 0x00FF) as u8
}

/// Returns the last three nibbles of the given u16.
///
/// ```rust
/// # use chip8_interpreter::bit_operations::get_last_three_nibbles;
/// assert_eq!(0x0BCD, get_last_three_nibbles(0xABCD));
/// ```
pub fn get_last_three_nibbles(bytes: u16) -> u16 {
    bytes & 0x0FFF
}

/// Returns the first nibble of the given u16.
///
/// ```rust
/// # use chip8_interpreter::bit_operations::get_first_nibble;
/// assert_eq!(0x0A, get_first_nibble(0xABCD));
/// ```
pub fn get_first_nibble(bytes: u16) -> u8 {
    let three_nibbles_len = 4 * 3;
    ((bytes & 0xF000) >> three_nibbles_len) as u8
}

/// Returns the second nibble of the given u16.
///
/// ```rust
/// # use chip8_interpreter::bit_operations::get_second_nibble;
/// assert_eq!(0x0B, get_second_nibble(0xABCD));
/// ```
pub fn get_second_nibble(bytes: u16) -> u8 {
    let two_nibbles_len = 4 * 2;
    ((bytes & 0x0F00) >> two_nibbles_len) as u8
}

/// Returns the third nibble of the given u16.
///
/// ```rust
/// # use chip8_interpreter::bit_operations::get_third_nibble;
/// assert_eq!(0x0C, get_third_nibble(0xABCD));
/// ```
pub fn get_third_nibble(bytes: u16) -> u8 {
    let one_nibble_len = 4;
    ((bytes & 0x00F0) >> one_nibble_len) as u8
}

/// Returns the fourth nibble of the given u16.
///
/// ```rust
/// # use chip8_interpreter::bit_operations::get_fourth_nibble;
/// assert_eq!(0x0D, get_fourth_nibble(0xABCD));
/// ```
pub fn get_fourth_nibble(bytes: u16) -> u8 {
    (bytes & 0x000F) as u8
}

/// Breaks down the given u16 into a tuple of nibbles.
///
/// ```rust
/// # use chip8_interpreter::bit_operations::break_into_nibbles;
/// assert_eq!((0x0A, 0x0B, 0x0C, 0x0D), break_into_nibbles(0xABCD));
/// ```
pub fn break_into_nibbles(bytes: u16) -> InstructionNibbles {
    let first = get_first_nibble(bytes);
    let second = get_second_nibble(bytes);
    let third = get_third_nibble(bytes);
    let fourth = get_fourth_nibble(bytes);

    (first, second, third, fourth)
}

#[test]
fn get_nth_bit_all() {
    assert_eq!(Ok(false), get_nth_bit(0, 0b00000000));
    assert_eq!(Ok(true), get_nth_bit(0, 0b00000001));
    assert_eq!(Ok(true), get_nth_bit(1, 0b00000010));
    assert_eq!(Ok(true), get_nth_bit(2, 0b00000100));
    assert_eq!(Ok(true), get_nth_bit(3, 0b00001000));
    assert_eq!(Ok(true), get_nth_bit(4, 0b00010000));
    assert_eq!(Ok(true), get_nth_bit(5, 0b00100000));
    assert_eq!(Ok(true), get_nth_bit(6, 0b01000000));
    assert_eq!(Ok(true), get_nth_bit(7, 0b10000000));
}

#[test]
fn get_nth_bit_invalid_bit() {
    assert_eq!(
        Err("Invalid byte bit index: 8".to_string()),
        get_nth_bit(8, 0b00000000)
    );
}
