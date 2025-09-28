use paste::paste;

use crate::Console;

pub union Register {
    pub value: u16,
    pub halves: [u8; 2]
}

pub trait BitFlag {
    const VALUE: u8;

    fn to_string() -> String;
}


macro_rules! def_bitflag_type {
    ($name:ident, $val:literal) => {
        paste! {
            pub const [<$name _VAL>]: u8 = $val;
            #[allow(non_camel_case_types)]
            pub struct $name;
            impl BitFlag for $name {
                const VALUE: u8 = [<$name _VAL>];
                fn to_string() -> String {
                    stringify!($name).to_string()
                }
            }
        }
    };
}

def_bitflag_type!(ADD, 0);
def_bitflag_type!(SUB, 1);
def_bitflag_type!(AND, 2);
def_bitflag_type!(XOR, 3);
def_bitflag_type!(OR, 4);
def_bitflag_type!(CARRY, 0);
def_bitflag_type!(NO_CARRY, 1);
def_bitflag_type!(LEFT, 0);
def_bitflag_type!(RIGHT, 1);

pub trait Hookable {
    fn hook(&mut self, console: &Console, log: String, addr: u16);
}