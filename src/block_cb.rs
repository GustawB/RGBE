use core::panic;

use macros::{arg_register, match_value};

use crate::{common::rotate_operand, constants::{flag, BitFlag, CARRY, LEFT, NO_CARRY, RIGHT}, types::{Console, RegSize, Value}};

fn rotate<DIR: BitFlag, C: BitFlag>(r8: u8, console: &mut Console) {
    rotate_operand::<DIR, C>(r8, console);
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
                **r <<= 1;
            },
            1 => {
                c = **r & 0x1;
                **r >>= 1;
            }
            _ => panic!("Invalid direction"),
        }
        res = **r;
    });
    console.registers.clear_or_set_flag(res == 0, flag::Z);
    console.registers.clear_or_set_flag(c != 0, flag::C);
}

fn swap_r8(r8: u8, console: &mut Console) {
    console.registers.clear_flags(&[flag::N, flag::H, flag::C]);
    let r8_reg: &mut Value = &mut console.registers[RegSize::Byte(r8)];
    let res: u8;
    match_value!(r8_reg, Value::Byte(r) => {
        **r = **r << 4 | **r >> 4;
        res = **r;
    });
    console.registers.clear_or_set_flag(res == 0, flag::Z);
}

fn srl_r8(r8: u8, console: &mut Console) {
    console.registers.clear_flags(&[flag::N, flag::H, flag::C]);
    let r8_reg: &mut Value = &mut console.registers[RegSize::Byte(r8)];
    let res: u8;
    let c: u8;
    match_value!(r8_reg, Value::Byte(r) => {
        c = **r & 0x1;
        **r = **r >> 1;
        res = **r;
    });
    console.registers.clear_or_set_flag(res == 0, flag::Z);
    console.registers.clear_or_set_flag(c != 0, flag::Z);
}

#[arg_register(r8)]
fn bit_b3_r8(b3: u8, r8: u8, console: &mut Console) {
    console.registers.clear_flag(flag::N);
    console.registers.set_flag(flag::H);
    console.registers.clear_or_set_flag(r8_val & 0x1 << b3 == 0, flag::Z);
}

fn res_b3_r8(b3: u8, r8: u8, console: &mut Console) {
    let r8_reg: &mut Value = &mut console.registers[RegSize::Byte(r8)];
    match_value!(r8_reg, Value::Byte(r) => { **r &= !(0x1 << b3); });
}

#[arg_register(r8)]
fn set_b3_r8(b3: u8, r8: u8, console: &mut Console) {
    let r8_reg: &mut Value = &mut console.registers[RegSize::Byte(r8)];
    match_value!(r8_reg, Value::Byte(r) => { **r |= 0x1 << b3; });
}

pub fn dispatch(instr: u8, console: &mut Console) -> () {
    let r8: u8 = (instr << 5) >> 5;
    let b3: u8 = (instr << 2) >> 5;
    
    if instr >> 3 == 0 {
        rotate::<LEFT, CARRY>(r8, console);
    } else if instr >> 3 == 1 {
        rotate::<RIGHT, CARRY>(r8, console);
    } else if instr >> 3 == 2 {
        rotate::<LEFT, NO_CARRY>(r8, console);
    } else if instr >> 3 == 3 {
        rotate::<RIGHT, NO_CARRY>(r8, console);
    } else if instr >> 3 == 4 {
        shift::<LEFT>(r8, console);
    } else if instr >> 3 == 5 {
        shift::<RIGHT>(r8, console);
    } else if instr >> 3 == 6 {
        swap_r8(r8, console);
    } else if instr >> 3 == 7 {
        srl_r8(r8, console);
    } else if instr >> 6 == 1 {
        bit_b3_r8(b3, r8, console);
    } else if instr >> 6 == 2 {
        res_b3_r8(b3, r8, console);
    } else if instr >> 6 == 3 {
        set_b3_r8(b3, r8, console);
    } else {
        panic!("Invalid opcode for block three");
    }
}