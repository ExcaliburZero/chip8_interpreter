pub type InstructionNibbles = (u8, u8, u8, u8);

pub fn get_last_two_nibbles(bytes: u16) -> u8 {
    // Get the last two nibbles
    (bytes & 0x00FF) as u8
}

pub fn get_last_three_nibbles(bytes: u16) -> u16 {
    // Get the last three nibbles
    bytes & 0x0FFF
}

pub fn get_first_nibble(bytes: u16) -> u8 {
    // Get the first nibble
    let three_nibbles_len = 4 * 3;
    ((bytes & 0xF000) >> three_nibbles_len) as u8
}

pub fn get_second_nibble(bytes: u16) -> u8 {
    // Get the second nibble
    let two_nibbles_len = 4 * 2;
    ((bytes & 0x0F00) >> two_nibbles_len) as u8
}

pub fn get_third_nibble(bytes: u16) -> u8 {
    // Get the third nibble
    let one_nibble_len = 4;
    ((bytes & 0x00F0) >> one_nibble_len) as u8
}

pub fn get_fourth_nibble(bytes: u16) -> u8 {
    // Get the last nibble
    (bytes & 0x000F) as u8
}

pub fn break_into_nibbles(bytes: u16) -> InstructionNibbles {
    let first = get_first_nibble(bytes);
    let second = get_second_nibble(bytes);
    let third = get_third_nibble(bytes);
    let fourth = get_fourth_nibble(bytes);

    (first, second, third, fourth)
}
