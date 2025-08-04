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
pub const C: u8                 = 1;
pub const F: u8                 = 8;

pub const IME: u16              = 0xFFFF;

pub mod flag {
    pub const Z: u8                 = 0x80;
    pub const N: u8                 = 0x40;
    pub const H: u8                 = 0x20;
    pub const C: u8                 = 0x10;
}

pub trait BitFlag {
    const VALUE: u8;
}

pub struct ADD;
impl BitFlag for ADD {
    const VALUE: u8 = 0;
}

pub struct SUB;
impl BitFlag for SUB {
    const VALUE: u8 = 1;
}

pub struct AND;
impl BitFlag for AND {
    const VALUE: u8 = 2;
}

pub struct XOR;
impl BitFlag for XOR {
    const VALUE: u8 = 3;
}

pub struct OR;
impl BitFlag for OR {
    const VALUE: u8 = 4;
}

pub struct CARRY;
impl BitFlag for CARRY {
    const VALUE: u8 = 0;
}

#[allow(non_camel_case_types)]
pub struct NO_CARRY;
impl BitFlag for NO_CARRY {
    const VALUE: u8 = 1;
}