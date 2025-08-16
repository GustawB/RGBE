pub const ADDR_BUS_SIZE: usize  = 65535;

pub mod cond {
    pub const NZ: u8 = 0;
    pub const Z: u8 = 1;
    pub const NC: u8 = 2;
    pub const C: u8 = 3;    
}

pub const SP: u8                = 3;
pub const HL: u8                = 2;
pub const A: u8                 = 7;
pub const EA: u8                = 10; // Explicit A (instead of r8 value)
pub const C: u8                 = 1;

pub const IME: u16              = 0xFFFE;

pub mod flag {
    pub const Z: u8                 = 0x80;
    pub const N: u8                 = 0x40;
    pub const H: u8                 = 0x20;
    pub const C: u8                 = 0x10;
}
