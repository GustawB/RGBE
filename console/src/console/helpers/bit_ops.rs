pub mod half_carry {
    pub fn add_8(base: u8, addend: u8, carry: u8) -> bool {
        ((base & 0xF).wrapping_add(addend & 0xF).wrapping_add(carry & 0xF)) & 0x10 == 0x10
    }

    pub fn sub_8(base: u8, subtrahend: u8, carry: u8) -> bool {
        ((base & 0xF).wrapping_sub(subtrahend & 0xF).wrapping_sub(carry & 0xF)) & 0x10 == 0x10
    }

    pub fn add_16(base: u16, addend: u16) -> bool {
        ((base & 0x0FFF).wrapping_add(addend & 0x0FFF)) & 0x1000 != 0
    }

    #[allow(dead_code)]
    pub fn sub_16(base: u16, subtrahend: u16) -> bool {
        ((base & 0x0FFF).wrapping_sub(subtrahend & 0x0FFF)) & 0x1000 != 0
    }
}

pub mod carry {
    pub fn add_8(base: u8, addend: u8, carry: u8) -> bool {
        ((base as u16).wrapping_add(addend as u16).wrapping_add(carry as u16)) & 0x100 == 0x100
    }

    pub fn sub_8(base: u8, subtrahend: u8, carry: u8) -> bool {
        ((base as u16).wrapping_sub(subtrahend as u16).wrapping_sub(carry as u16)) & 0x100 == 0x100
    }

    pub fn add_16(base: u16, addend: u16) -> bool {
        ((base as u32).wrapping_add(addend as u32)) & 0x10000 == 0x10000
    }

    #[allow(dead_code)]
    pub fn sub_16(base: u16, subtrahend: u16) -> bool {
        ((base as u32).wrapping_sub(subtrahend as u32)) & 0x10000 == 0x10000
    }
}