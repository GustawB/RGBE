use log::debug;

use crate::console::{helpers::{bit_ops::{carry, half_carry}, constants::{flag, reg8}}, types::types::{BitFlag, Byte, ADD_VAL, AND_VAL, CARRY_VAL, LEFT_VAL, NO_CARRY_VAL, OR_VAL, RIGHT_VAL, SUB_VAL, XOR_VAL}, Console};

fn log_arithm_a<OP: BitFlag, C: BitFlag>(operand: u8, arg_type: u8, curr_ip: u16) {
    let arg: String = match arg_type {
        reg8::MAX_REG8 => format!("{operand}"),
        _ => reg8::reg_to_name(arg_type),
    };

    match OP::VALUE {
        ADD_VAL => {
            match C::VALUE {
                CARRY_VAL =>debug_addr(curr_ip, format!("ADC A, {arg}")),
                NO_CARRY_VAL => debug_addr(curr_ip, format!("ADD A, {arg}")),
                _ => panic!("Flag value out of range (possible values are: CARRY_VAL, NO_CARRY_VAL)"),
            }
        },
        SUB_VAL => {
            match C::VALUE {
                CARRY_VAL => debug_addr(curr_ip, format!("SBC A, {arg}")),
                NO_CARRY_VAL => debug_addr(curr_ip, format!("ADD A, {arg}")),
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
    log_arithm_a::<OP, C>(operand, arg_type, curr_ip);
    if C::VALUE == CARRY_VAL && console.is_flag_set(flag::C) {
        // If op with carry, like ADC, and Carry is set, increment the operand.
        operand += 1;
    }
    let mut a_val: u8 = console[Byte { idx: reg8::A }];
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
    *(&mut console[Byte { idx: reg8::A }]) = a_val;
}

pub fn logic_a_operand<OP: BitFlag>(operand: u8, console: &mut Console) {
    let mut a_val: u8 = console[Byte { idx: reg8::A }];
     match OP::VALUE {
        AND_VAL => a_val &= operand,
        XOR_VAL => a_val ^= operand,
        OR_VAL => a_val |= operand,
        _ => panic!("Flag value out of range (possible values are: AND_VAL, XIOR_VAL, OR_VAL)"),
    }

    console.clear_or_set_flag(a_val == 0, flag::Z);
    console.clear_or_set_flag(OP::VALUE == 2, flag::H);
    console.clear_flags(&[flag::N, flag::C]);
    *(&mut console[Byte { idx: reg8::A }]) = a_val;
}

pub fn cp_a_operand(operand: u8, console: &mut Console) {
    let a_val: u8 = console[Byte { idx: reg8::A }];
    console.clear_or_set_flag(a_val - operand == 0, flag::Z);
    console.set_flag( flag::N);
    console.clear_or_set_flag(half_carry::sub_8(a_val, operand), flag::H);
    console.clear_or_set_flag(carry::sub_8(a_val, operand), flag::C);
}

pub fn rotate_operand<DIR: BitFlag, C: BitFlag>(r8: u8, console: &mut Console, curr_ip: u16) {
    console.clear_flags(&[flag::N, flag::H]);
    let curr_c: u8 = console.is_flag_set(flag::C) as u8;

    let mut reg: u8;
    if r8 != reg8::EA { reg = console[Byte { idx: r8 }]; }
    else { reg = console[Byte { idx: reg8::A }]; }

    let c: u8;
    match DIR::VALUE {
        LEFT_VAL => {
            c = reg >> 7;
            match C::VALUE {
                CARRY_VAL => {
                    reg = reg << 1 | c;

                    if r8 == reg8::EA { debug_addr(curr_ip, format!("RLCA")); }
                    else { debug_addr(curr_ip, format!("RLC {}", reg8::reg_to_name(r8))); }
                },
                NO_CARRY_VAL => {
                    reg = reg << 1 | curr_c;

                    if r8 == reg8::EA { debug_addr(curr_ip, format!("RLA")); }
                    else { debug_addr(curr_ip, format!("RL {}", reg8::reg_to_name(r8))); }
                },
                _ => panic!("Invalid carry"),
            }
        },
        RIGHT_VAL => {
            c = reg & 0x1;
            match C::VALUE {
                CARRY_VAL => {
                    reg = reg >> 1 | c << 7;

                    if r8 == reg8::EA { debug_addr(curr_ip, format!("RRCA")); }
                    else { debug_addr(curr_ip, format!("RRC {}", reg8::reg_to_name(r8))); }
                },
                NO_CARRY_VAL => {
                    reg = reg >> 1 | curr_c << 7;

                    if r8 == reg8::EA { debug_addr(curr_ip, format!("RRA")); }
                    else { debug_addr(curr_ip, format!("RR {}", reg8::reg_to_name(r8))); }
                },
                _ => panic!("Invalid carry"),
            }
        },
        _ => panic!("Invalid direction"),
    };
    console.clear_or_set_flag(reg == 0 && r8 != reg8::EA, flag::Z);
    console.clear_or_set_flag(c != 0, flag::C);

    if r8 != reg8::EA { *(&mut console[Byte { idx: r8 }]) = reg; }
    else { *(&mut console[Byte { idx: reg8::A }]) = reg; }
}