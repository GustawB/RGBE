use core::panic;

use macros::{arg, match_value};

use crate::{bit_ops::*, constants::*, types::*};

#[arg(r8)]
fn arithm_a_r8<OP: BitFlag, C: BitFlag>(r8: u8, console: &mut Console) {
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let base: u8;
    match_value!(a_reg, Value::Byte(r) => {
        base = **r;
        match OP::VALUE {
            0 => **r += r8_val,
            1 => **r -= r8_val,
            _ => panic!("Flag value out of range (possible values are: 0, 1)"),
        }
    });

    console.registers.clear_or_set_flag(OP::VALUE == 0, flag::N);
    match OP::VALUE {
        0 => {
            console.registers.clear_or_set_flag(base + r8_val == 0, flag::Z);
            console.registers.clear_or_set_flag(half_carry::add_8(base, r8_val), flag::H);
            console.registers.clear_or_set_flag(carry::add_8(base, r8_val), flag::C);
        },
        1 => {
            console.registers.clear_or_set_flag(base - r8_val == 0, flag::Z);
            console.registers.clear_or_set_flag(half_carry::sub_8(base, r8_val), flag::H);
            console.registers.clear_or_set_flag(carry::sub_8(base, r8_val), flag::C);
        }, 
        _ => panic!("Flag value out of range (possible values are: 0, 1)"),
    }
}

#[arg(r8)]
fn logic_a_r8<OP: BitFlag>(r8: u8, console: &mut Console) {
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let res: u8;
    match_value!(a_reg, Value::Byte(r) => {
        match OP::VALUE {
            2 => **r = (**r) & r8_val,
            3 => **r = (**r) ^ r8_val,
            4 => **r = (**r) | r8_val,
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
    let r8: u8 = instr & 0x07;
    let op: u8 = (instr << 2) >> 5;
    match op {
        0 => arithm_a_r8::<ADD, NO_CARRY>(r8, console),
        1 => arithm_a_r8::<ADD, CARRY>(r8, console),
        2 => arithm_a_r8::<SUB, NO_CARRY>(r8, console),
        3 => arithm_a_r8::<SUB, CARRY>(r8, console),
        4 => logic_a_r8::<AND>(r8, console),
        5 => logic_a_r8::<XOR>(r8, console),
        6 => logic_a_r8::<OR>(r8, console),
        7 => cp_a_r8(r8, console),
        _ => panic!("Invalid opcode in block two"),
    }
}