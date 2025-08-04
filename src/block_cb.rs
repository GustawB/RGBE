use core::panic;

use macros::match_value;

use crate::{constants::{flag, BitFlag}, types::{Console, RegSize, Value}};

fn rotate<DIR: BitFlag, C: BitFlag>(r8: u8, console: &mut Console) {
    console.registers.clear_flags(&[flag::N, flag::H]);
    let curr_c: u8 = console.registers.is_flag_set(flag::C) as u8;
    let r8_reg: &mut Value = &mut console.registers[RegSize::Byte(r8)];
    let c: u8;
    let res: u8;
    match_value!(r8_reg, Value::Byte(r) => {
        match DIR::VALUE {
            0 => {
                c = **r >> 7;
                match C::VALUE {
                    0 => **r = **r << 1 | c,
                    1 => **r = **r << 1 | curr_c,
                    _ => panic!("Invalid carry"),
                }
            },
            1 => {
                c = **r & 0x1;
                match C::VALUE {
                    0 => **r = **r >> 1 | c << 7,
                    1 => **r = **r >> 1 | curr_c << 7,
                    _ => panic!("Invalid carry"),
                }
            },
            _ => panic!("Invalid direction"),
        };
        res = **r;
    });
    console.registers.clear_or_set_flag(res == 0, flag::Z);
    console.registers.clear_or_set_flag(c != 0, flag::C);
}

fn shift<DIR: BitFlag>(r8: u8, console: &mut Console) {
    console.registers.clear_flags(&[flag::N, flag::H]);
    let r8_reg: &mut Value = &mut console.registers[RegSize::Byte(r8)];
    let c: u8;
    let res: u8;
    match_value!(r8_reg, Value::Byte(r) => {
        match DIR::VALUE {
            0 => {
                c = **r >> 7;
                **r << 1;
            },
            1 => {
                c = **r & 0x1;
                **r >> 1;
            }
            _ => panic!("Invalid direction"),
        }
        res = **r;
    });
    console.registers.clear_or_set_flag(res == 0, flag::Z);
    console.registers.clear_or_set_flag(c != 0, flag::C);
}