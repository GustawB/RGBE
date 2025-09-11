use constants::{flag, reg8};
use log::debug;

use crate::console::{helpers::{bit_ops::{carry, half_carry}}, types::{BitFlag, ADD_VAL, AND_VAL, CARRY_VAL, LEFT_VAL, NO_CARRY_VAL, OR_VAL, RIGHT_VAL, SUB_VAL, XOR_VAL}, Console};

fn log_arithm_a<OP: BitFlag, C: BitFlag>(console: &mut Console, operand: u8, arg_type: usize, curr_ip: u16) {
    let arg: String = match arg_type {
        reg8::MAX_REG8 => format!("{operand}"),
        _ => reg8::reg_to_name(arg_type as u8),
    };

    match OP::VALUE {
        ADD_VAL => {
            match C::VALUE {
                CARRY_VAL => console.call_hook(format!("ADC A, {arg}"), curr_ip),
                NO_CARRY_VAL => console.call_hook(format!("ADD A, {arg}"), curr_ip),
                _ => panic!("Flag value out of range (possible values are: CARRY_VAL, NO_CARRY_VAL)"),
            }
        },
        SUB_VAL => {
            match C::VALUE {
                CARRY_VAL => console.call_hook(format!("SBC A, {arg}"), curr_ip),
                NO_CARRY_VAL => console.call_hook(format!("ADD A, {arg}"), curr_ip),
                _ => panic!("Flag value out of range (possible values are: CARRY_VAL, NO_CARRY_VAL)"),
            }
        },
        _ => panic!("Flag value out of range (possible values are: ADD_VAL, SUB_VAL)"),
    };
}

#[inline(always)]
pub fn debug_addr(addr: u16, expr: String) {
    debug!("0x{:04X}: {expr}", addr);
}

pub fn arithm_a_operand<OP: BitFlag, C: BitFlag>(mut operand: u8, console: &mut Console, arg_type: u8, curr_ip: u16) {
    log_arithm_a::<OP, C>(console, operand, arg_type as usize, curr_ip);
    if C::VALUE == CARRY_VAL && console.is_flag_set(flag::C) {
        // If op with carry, like ADC, and Carry is set, increment the operand.
        operand += 1;
    }
    let mut a_val: u8 = console.get_r8(reg8::A);
    match OP::VALUE {
        ADD_VAL => a_val += operand,
        SUB_VAL => a_val -= operand,
        _ => panic!("Flag value out of range (possible values are: ADD_VAL, SUB_VAL)"),
    }

    console.clear_or_set_flag(OP::VALUE == 0, flag::N);
    match OP::VALUE {
        ADD_VAL => {
            console.clear_or_set_flag(half_carry::add_8(a_val - operand, operand), flag::H);
            console.clear_or_set_flag(carry::add_8(a_val - operand, operand), flag::C);
        },
        SUB_VAL => {
            console.clear_or_set_flag(half_carry::sub_8(a_val + operand, operand), flag::H);
            console.clear_or_set_flag(carry::sub_8(a_val + operand, operand), flag::C);
        }, 
        _ => panic!("Flag value out of range (possible values are: ADD_VAL, SUB_VAL)"),
    }
    console.clear_or_set_flag(a_val == 0, flag::Z);
    console.set_r8(reg8::A, a_val);
}

pub fn logic_a_operand<OP: BitFlag>(operand: u8, console: &mut Console) {
    let mut a_val: u8 = console.get_r8(reg8::A);
     match OP::VALUE {
        AND_VAL => a_val &= operand,
        XOR_VAL => a_val ^= operand,
        OR_VAL => a_val |= operand,
        _ => panic!("Flag value out of range (possible values are: AND_VAL, XIOR_VAL, OR_VAL)"),
    }

    console.clear_or_set_flag(a_val == 0, flag::Z);
    console.clear_or_set_flag(OP::VALUE == 2, flag::H);
    console.clear_flags(&[flag::N, flag::C]);
    console.set_r8(reg8::A, a_val);
}

pub fn cp_a_operand(operand: u8, console: &mut Console) {
    let a_val: u8 = console.get_r8(reg8::A);
    console.clear_or_set_flag(a_val == operand, flag::Z);
    console.set_flag( flag::N);
    console.clear_or_set_flag(half_carry::sub_8(a_val, operand), flag::H);
    console.clear_or_set_flag(carry::sub_8(a_val, operand), flag::C);
}

pub fn rotate_operand<DIR: BitFlag, C: BitFlag>(r8: u8, console: &mut Console, curr_ip: u16) {
    console.clear_flags(&[flag::N, flag::H]);
    let curr_c: u8 = console.is_flag_set(flag::C) as u8;

    let mut reg: u8;
    if r8 != reg8::EA { reg = console.get_r8(r8); }
    else { reg = console.get_r8(reg8::A); }

    let c: u8;
    match DIR::VALUE {
        LEFT_VAL => {
            c = reg >> 7;
            match C::VALUE {
                CARRY_VAL => {
                    if r8 == reg8::EA { console.call_hook(format!("RLCA"), curr_ip); }
                    else { console.call_hook(format!("RLC {}", reg8::reg_to_name(r8)), curr_ip); }

                    reg = reg << 1 | c;
                },
                NO_CARRY_VAL => {
                    if r8 == reg8::EA { console.call_hook(format!("RLA"), curr_ip); }
                    else { console.call_hook(format!("RL {}", reg8::reg_to_name(r8)), curr_ip); }

                    reg = reg << 1 | curr_c;

                },
                _ => panic!("Invalid carry"),
            }
        },
        RIGHT_VAL => {
            c = reg & 0x1;
            match C::VALUE {
                CARRY_VAL => {
                    if r8 == reg8::EA { console.call_hook(format!("RRCA"), curr_ip); }
                    else { console.call_hook(format!("RRC {}", reg8::reg_to_name(r8)), curr_ip); }

                    reg = reg >> 1 | c << 7;
                },
                NO_CARRY_VAL => {
                    if r8 == reg8::EA { console.call_hook(format!("RRA"), curr_ip); }
                    else { console.call_hook(format!("RR {}", reg8::reg_to_name(r8)), curr_ip); }

                    reg = reg >> 1 | curr_c << 7;
                },
                _ => panic!("Invalid carry"),
            }
        },
        _ => panic!("Invalid direction"),
    };
    console.clear_or_set_flag(reg == 0 && r8 != reg8::EA, flag::Z);
    console.clear_or_set_flag(c != 0, flag::C);

    if r8 != reg8::EA { console.set_r8(r8, reg); }
    else { console.set_r8(reg8::A, reg); }
}