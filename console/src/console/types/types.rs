pub union Register {
    pub value: u16,
    pub halves: [u8; 2]
}

pub struct Byte {
    pub idx: u8,
}
pub struct Word {
    pub idx: u8,
}
pub struct WordSTK {
    pub idx: u8,
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

pub struct LEFT;
impl BitFlag for LEFT {
    const VALUE: u8 = 0;
}

pub struct RIGHT;
impl BitFlag for RIGHT {
    const VALUE: u8 = 1;
}