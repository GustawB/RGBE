pub mod half_carry {
    pub fn add_8(base: u8, addend: u8) -> bool {
        ((base & 0xF) + (addend & 0xF)) & 0x10 != 0
    }

    pub fn sub_8(base: u8, subtrahend: u8) -> bool {
        (base & 0x0F) < (subtrahend & 0x0F)
    }

    pub fn add_16(base: u16, addend: u16) -> bool {
        ((base & 0x0FFF) + (addend & 0x0FFF)) & 0x1000 != 0
    }

    pub fn sub_16(base: u16, subtrahend: u16) -> bool {
        (base & 0x0FFF) < (subtrahend & 0x0FFF)
    }
}

pub mod carry {
    pub fn add_8(base: u8, addend: u8) -> bool {
        ((base as u16) + (addend as u16)) > 0xFF
    }

    pub fn sub_8(base: u8, subtrahend: u8) -> bool {
        subtrahend > base
    }

    pub fn add_16(base: u16, addend: u16) -> bool {
        ((base as u32) + (addend as u32)) > 0xFFFF
    }

    pub fn sub_16(base: u16, subtrahend: u16) -> bool {
        subtrahend > base
    }
}