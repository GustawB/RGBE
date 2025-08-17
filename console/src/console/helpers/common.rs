use crate::console::{helpers::{bit_ops::{carry, half_carry}, constants::{flag, reg8}}, types::types::{BitFlag, Byte, ADD_VAL, AND_VAL, CARRY_VAL, LEFT_VAL, NO_CARRY_VAL, OR_VAL, RIGHT_VAL, SUB_VAL, XOR_VAL}, Console};


// TODO: use carry bruh
pub fn arithm_a_operand<OP: BitFlag, C: BitFlag>(operand: u8, console: &mut Console) {
    let mut a_val: u8 = console[Byte { idx: reg8::A }];
    match OP::VALUE {
        ADD_VAL => a_val += operand,
        SUB_VAL => a_val -= operand,
        _ => panic!("Flag value out of range (possible values are: 0, 1)"),
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
        _ => panic!("Flag value out of range (possible values are: 0, 1)"),
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
        _ => panic!("Flag value out of range (possible values are: 2, 3, 4)"),
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

pub fn rotate_operand<DIR: BitFlag, C: BitFlag>(r8: u8, console: &mut Console) {
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
                CARRY_VAL => reg = reg << 1 | c,
                NO_CARRY_VAL => reg = reg << 1 | curr_c,
                _ => panic!("Invalid carry"),
            }
        },
        RIGHT_VAL => {
            c = reg & 0x1;
            match C::VALUE {
                CARRY_VAL => reg = reg >> 1 | c << 7,
                NO_CARRY_VAL => reg = reg >> 1 | curr_c << 7,
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