use crate::{bit_ops::{carry, half_carry}, constants::{flag, BitFlag, A, EA}, types::{Byte, Console}};

pub fn arithm_a_operand<OP: BitFlag, C: BitFlag>(operand: u8, console: &mut Console) {
    let mut a_val: u8 = console.physical[Byte { idx: A }];
    match OP::VALUE {
        0 => a_val += operand,
        1 => a_val -= operand,
        _ => panic!("Flag value out of range (possible values are: 0, 1)"),
    }

    console.physical.clear_or_set_flag(OP::VALUE == 0, flag::N);
    match OP::VALUE {
        0 => {
            console.physical.clear_or_set_flag(half_carry::add_8(a_val - operand, operand), flag::H);
            console.physical.clear_or_set_flag(carry::add_8(a_val - operand, operand), flag::C);
        },
        1 => {
            
            console.physical.clear_or_set_flag(half_carry::sub_8(a_val + operand, operand), flag::H);
            console.physical.clear_or_set_flag(carry::sub_8(a_val + operand, operand), flag::C);
        }, 
        _ => panic!("Flag value out of range (possible values are: 0, 1)"),
    }
    console.physical.clear_or_set_flag(a_val == 0, flag::Z);
    *(&mut console.physical[Byte { idx: A }]) = a_val;
}

pub fn logic_a_operand<OP: BitFlag>(operand: u8, console: &mut Console) {
    let mut a_val: u8 = console.physical[Byte { idx: A }];
     match OP::VALUE {
        2 => a_val &= operand,
        3 => a_val ^= operand,
        4 => a_val |= operand,
        _ => panic!("Flag value out of range (possible values are: 2, 3, 4)"),
    }

    console.physical.clear_or_set_flag(a_val == 0, flag::Z);
    console.physical.clear_or_set_flag(OP::VALUE == 2, flag::H);
    console.physical.clear_flags(&[flag::N, flag::C]);
    *(&mut console.physical[Byte { idx: A }]) = a_val;
}

pub fn cp_a_operand(operand: u8, console: &mut Console) {
    let a_val: u8 = console.physical[Byte { idx: A }];
    console.physical.clear_or_set_flag(a_val - operand == 0, flag::Z);
    console.physical.set_flag( flag::N);
    console.physical.clear_or_set_flag(half_carry::sub_8(a_val, operand), flag::H);
    console.physical.clear_or_set_flag(carry::sub_8(a_val, operand), flag::C);
}

pub fn rotate_operand<DIR: BitFlag, C: BitFlag>(r8: u8, console: &mut Console) {
    console.physical.clear_flags(&[flag::N, flag::H]);
    let curr_c: u8 = console.physical.is_flag_set(flag::C) as u8;

    let mut reg: u8;
    if r8 != EA { reg = console.physical[Byte { idx: r8 }]; }
    else { reg = console.physical[Byte { idx: A }]; }

    let c: u8;
    match DIR::VALUE {
        0 => {
            c = reg >> 7;
            match C::VALUE {
                0 => reg = reg << 1 | c,
                1 => reg = reg << 1 | curr_c,
                _ => panic!("Invalid carry"),
            }
        },
        1 => {
            c = reg & 0x1;
            match C::VALUE {
                0 => reg = reg >> 1 | c << 7,
                1 => reg = reg >> 1 | curr_c << 7,
                _ => panic!("Invalid carry"),
            }
        },
        _ => panic!("Invalid direction"),
    };
    console.physical.clear_or_set_flag(reg == 0 && r8 != EA, flag::Z);
    console.physical.clear_or_set_flag(c != 0, flag::C);

    if r8 != EA { *(&mut console.physical[Byte { idx: r8 }]) = reg; }
    else { *(&mut console.physical[Byte { idx: A }]) = reg; }
}