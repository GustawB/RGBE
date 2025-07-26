use core::panic;

use macros::match_value;

use crate::{bit_ops::*, constants::*, types::*};

fn arithm_a_imm8<OP: BitFlag, C: BitFlag>(r8: u8, console: &mut Console) {
    let mut addend: u8 = (console.registers.is_flag_set(flag::C) && C::VALUE != 0) as u8;
    let src_reg: &Value = &console.registers[RegSize::Byte(r8)];
    match_value!(src_reg, Value::Byte(r) => { addend += **r; });

    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let base: u8;
    match_value!(a_reg, Value::Byte(r) => {
        base = **r;
        match OP::VALUE {
            0 => **r += addend,
            1 => **r -= addend,
            _ => panic!("Flag value out of range (possible values are: 0, 1)"),
        }
    });

    console.registers.clear_or_set_flag(OP::VALUE == 0, flag::N);
    match OP::VALUE {
        0 => {
            console.registers.clear_or_set_flag(base + addend == 0, flag::Z);
            console.registers.clear_or_set_flag(half_carry::add_8(base, addend), flag::H);
            console.registers.clear_or_set_flag(carry::add_8(base, addend), flag::C);
        },
        1 => {
            console.registers.clear_or_set_flag(base - addend == 0, flag::Z);
            console.registers.clear_or_set_flag(half_carry::sub_8(base, addend), flag::H);
            console.registers.clear_or_set_flag(carry::sub_8(base, addend), flag::C);
        }, 
        _ => panic!("Flag value out of range (possible values are: 0, 1)"),
    }
}

fn logic_a_r8<OP: BitFlag>(r8: u8, console: &mut Console) {
    let src_val: u8;
    let src_reg: &Value = &console.registers[RegSize::Byte(r8)];
    match_value!(src_reg, Value::Byte(r) => { src_val = **r; });

    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let res: u8;
    match_value!(a_reg, Value::Byte(r) => {
        match OP::VALUE {
            2 => **r = (**r) & src_val,
            3 => **r = (**r) ^ src_val,
            4 => **r = (**r) | src_val,
            _ => panic!("Flag value out of range (possible values are: 2, 3, 4)"),
        }
        res = **r;
    });

    console.registers.clear_or_set_flag(res == 0, flag::Z);
    console.registers.clear_or_set_flag(OP::VALUE == 2, flag::H);
    console.registers.clear_flags(&[flag::N, flag::C]);
}

fn cp_a_r8(r8: u8, console: &mut Console) {
    let subtrahend: u8;
    let src_reg: &Value = &console.registers[RegSize::Byte(r8)];
    match_value!(src_reg, Value::Byte(r) => { subtrahend = **r; });

    let a_reg: &Value = &console.registers[RegSize::Byte(A)];
    let base: u8;
    match_value!(a_reg, Value::Byte(r) => { base = **r; });

    console.registers.clear_or_set_flag(base - subtrahend == 0, flag::Z);
    console.registers.set_flag( flag::N);
    console.registers.clear_or_set_flag(half_carry::sub_8(base, subtrahend), flag::H);
    console.registers.clear_or_set_flag(carry::sub_8(base, subtrahend), flag::C);
}

pub fn dispatch(instr: u8, console: &mut Console) -> () {
    arithm_a_imm8::<ADD, CARRY>(instr, console);
}