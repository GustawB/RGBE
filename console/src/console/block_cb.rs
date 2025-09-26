use core::panic;

use constants::{flag, reg8};

use crate::console::{helpers::{common::rotate_operand}, types::{BitFlag, CARRY, LEFT, NO_CARRY, RIGHT}, Console};

fn rotate<DIR: BitFlag, C: BitFlag>(r8: u8, console: &mut Console, curr_ip: u16) {
    rotate_operand::<DIR, C>(r8, console, curr_ip);
}

fn shift<DIR: BitFlag>(r8: u8, console: &mut Console, curr_ip: u16) {
    let mut r8_val: u8 = console.get_r8(r8);
    let c: u8;
    match DIR::VALUE {
        0 => {
            console.call_hook(format!("SLA {}", reg8::reg_to_name(r8)), curr_ip);
            c = r8_val >> 7;
            r8_val <<= 1;
        },
        1 => {
            console.call_hook(format!("SRA {}", reg8::reg_to_name(r8)), curr_ip);
            c = r8_val & 0x1;
            r8_val = (r8_val >> 1) | (r8_val & 0x80);
        }
        _ => panic!("Invalid direction"),
    }

    console.clear_flags(&[flag::N, flag::H]);
    console.clear_or_set_flag(r8_val == 0, flag::Z);
    console.clear_or_set_flag(c != 0, flag::C);
    console.set_r8(r8, r8_val);
}

fn swap_r8(r8: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("SWAP {}", reg8::reg_to_name(r8)), curr_ip);

    console.clear_flags(&[flag::N, flag::H, flag::C]);
    let mut r8_val: u8 = console.get_r8(r8);
    r8_val = r8_val << 4 | r8_val >> 4;
    console.clear_or_set_flag(r8_val == 0, flag::Z);
    console.set_r8(r8, r8_val);
}

fn srl_r8(r8: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("SRL {}", reg8::reg_to_name(r8)), curr_ip);

    console.clear_flags(&[flag::N, flag::H, flag::C]);
    let mut r8_val: u8 = console.get_r8(r8);
    let c: u8 = r8_val & 0x1;
    r8_val >>= 1;
    console.clear_or_set_flag(r8_val == 0, flag::Z);
    console.clear_or_set_flag(c != 0, flag::C);
    console.set_r8(r8, r8_val);
}

fn bit_b3_r8(b3: u8, r8: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("BIT {b3}, {}", reg8::reg_to_name(r8)), curr_ip);

    let r8_val: u8 = console.get_r8(r8);
    console.clear_flag(flag::N);
    console.set_flag(flag::H);
    console.clear_or_set_flag(r8_val & 0x1 << b3 == 0, flag::Z);
}

fn res_b3_r8(b3: u8, r8: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("RES {b3}, {}", reg8::reg_to_name(r8)), curr_ip);
    console.set_r8(r8, console.get_r8(r8) & !(0x1 << b3));
}

fn set_b3_r8(b3: u8, r8: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("SET {b3}, {}", reg8::reg_to_name(r8)), curr_ip);
    console.set_r8(r8, console.get_r8(r8) | 0x1 << b3);
}

pub fn dispatch(console: &mut Console, instr: u8, curr_ip: u16) -> () {
    let r8: u8 = (instr << 5) >> 5;
    let b3: u8 = (instr << 2) >> 5;
    
    if instr >> 3 == 0 {
        rotate::<LEFT, CARRY>(r8, console, curr_ip);
    } else if instr >> 3 == 1 {
        rotate::<RIGHT, CARRY>(r8, console, curr_ip);
    } else if instr >> 3 == 2 {
        rotate::<LEFT, NO_CARRY>(r8, console, curr_ip);
    } else if instr >> 3 == 3 {
        rotate::<RIGHT, NO_CARRY>(r8, console, curr_ip);
    } else if instr >> 3 == 4 {
        shift::<LEFT>(r8, console, curr_ip);
    } else if instr >> 3 == 5 {
        shift::<RIGHT>(r8, console, curr_ip);
    } else if instr >> 3 == 6 {
        swap_r8(r8, console, curr_ip);
    } else if instr >> 3 == 7 {
        srl_r8(r8, console, curr_ip);
    } else if instr >> 6 == 1 {
        bit_b3_r8(b3, r8, console, curr_ip);
    } else if instr >> 6 == 2 {
        res_b3_r8(b3, r8, console, curr_ip);
    } else if instr >> 6 == 3 {
        set_b3_r8(b3, r8, console, curr_ip);
    } else {
        panic!("Invalid opcode in block cb");
    }
}