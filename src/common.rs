use macros::match_value;

use crate::{bit_ops::{carry, half_carry}, constants::{flag, BitFlag, A}, types::{Console, RegSize, Value}};

pub fn arithm_a_operand<OP: BitFlag, C: BitFlag>(operand: u8, console: &mut Console) {
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let base: u8;
    match_value!(a_reg, Value::Byte(r) => {
        base = **r;
        match OP::VALUE {
            0 => **r += operand,
            1 => **r -= operand,
            _ => panic!("Flag value out of range (possible values are: 0, 1)"),
        }
    });

    console.registers.clear_or_set_flag(OP::VALUE == 0, flag::N);
    match OP::VALUE {
        0 => {
            console.registers.clear_or_set_flag(base + operand == 0, flag::Z);
            console.registers.clear_or_set_flag(half_carry::add_8(base, operand), flag::H);
            console.registers.clear_or_set_flag(carry::add_8(base, operand), flag::C);
        },
        1 => {
            console.registers.clear_or_set_flag(base - operand == 0, flag::Z);
            console.registers.clear_or_set_flag(half_carry::sub_8(base, operand), flag::H);
            console.registers.clear_or_set_flag(carry::sub_8(base, operand), flag::C);
        }, 
        _ => panic!("Flag value out of range (possible values are: 0, 1)"),
    }
}

pub fn logic_a_operand<OP: BitFlag>(operand: u8, console: &mut Console) {
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let res: u8;
    match_value!(a_reg, Value::Byte(r) => {
        match OP::VALUE {
            2 => **r = (**r) & operand,
            3 => **r = (**r) ^ operand,
            4 => **r = (**r) | operand,
            _ => panic!("Flag value out of range (possible values are: 2, 3, 4)"),
        }
        res = **r;
    });

    console.registers.clear_or_set_flag(res == 0, flag::Z);
    console.registers.clear_or_set_flag(OP::VALUE == 2, flag::H);
    console.registers.clear_flags(&[flag::N, flag::C]);
}

pub fn cp_a_operand(operand: u8, console: &mut Console) {
    let a_reg: &Value = &console.registers[RegSize::Byte(A)];
    let base: u8;
    match_value!(a_reg, Value::Byte(r) => { base = **r; });

    console.registers.clear_or_set_flag(base - operand == 0, flag::Z);
    console.registers.set_flag( flag::N);
    console.registers.clear_or_set_flag(half_carry::sub_8(base, operand), flag::H);
    console.registers.clear_or_set_flag(carry::sub_8(base, operand), flag::C);
}