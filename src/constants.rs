pub const ADDR_BUS_SIZE: usize  = 65535;

pub mod cond {
    pub const NZ: u8 = 0;
    pub const Z: u8 = 1;
    pub const NC: u8 = 2;
    pub const C: u8 = 3;    
}

const NZ: u8 = 0;
const Z: u8 = 1;
const NC: u8 = 2;
const C: u8 = 3;

pub const SP: u8                = 3;
pub const HL: u8                = 2;
pub const A: u8                 = 7;
pub const F: u8                 = 8;

pub mod flag {
    pub const Z: u8                 = 0x80;
    pub const N: u8                 = 0x40;
    pub const H: u8                 = 0x20;
    pub const C: u8                 = 0x10;
}