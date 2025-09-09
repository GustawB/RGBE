use crate::Console;

pub union Register {
    pub value: u16,
    pub halves: [u8; 2]
}

pub trait BitFlag {
    const VALUE: u8;

    fn to_string() -> String;
}

pub const ADD_VAL: u8 = 0;
pub struct ADD;
impl BitFlag for ADD {
    const VALUE: u8 = ADD_VAL;

    fn to_string() -> String {
        "ADD".to_string()
    }
}

pub const SUB_VAL: u8 = 1;
pub struct SUB;
impl BitFlag for SUB {
    const VALUE: u8 = SUB_VAL;

    fn to_string() -> String {
        "SUB".to_string()
    }
}

pub const AND_VAL: u8 = 2;
pub struct AND;
impl BitFlag for AND {
    const VALUE: u8 = AND_VAL;

    fn to_string() -> String {
        "AND".to_string()
    }
}

pub const XOR_VAL: u8 = 3;
pub struct XOR;
impl BitFlag for XOR {
    const VALUE: u8 = XOR_VAL;

    fn to_string() -> String {
        "XOR".to_string()
    }
}

pub const OR_VAL: u8 = 4;
pub struct OR;
impl BitFlag for OR {
    const VALUE: u8 = OR_VAL;

    fn to_string() -> String {
        "OR".to_string()
    }
}

pub const CARRY_VAL: u8 = 0;
pub struct CARRY;
impl BitFlag for CARRY {
    const VALUE: u8 = CARRY_VAL;

    fn to_string() -> String {
        "CARRY".to_string()
    }
}

pub const NO_CARRY_VAL: u8 = 1;
#[allow(non_camel_case_types)]
pub struct NO_CARRY;
impl BitFlag for NO_CARRY {
    const VALUE: u8 = NO_CARRY_VAL;

    fn to_string() -> String {
        "NO_CARRY".to_string()
    }
}

pub const LEFT_VAL: u8 = 0;
pub struct LEFT;
impl BitFlag for LEFT {
    const VALUE: u8 = LEFT_VAL;

    fn to_string() -> String {
        "LEFT".to_string()
    }
}

pub const RIGHT_VAL: u8 = 1;
pub struct RIGHT;
impl BitFlag for RIGHT {
    const VALUE: u8 = RIGHT_VAL;

    fn to_string() -> String {
        "RIGHT".to_string()
    }
}

pub trait Hookable {
    fn hook(&mut self, console: &Console, log: String, addr: u16);
}