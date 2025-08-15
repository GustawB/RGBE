use core::panic;

use crate::{common::{arithm_a_operand, cp_a_operand, logic_a_operand}, constants::*, types::*};

fn arithm_a_r8<OP: BitFlag, C: BitFlag>(r8: u8, console: &mut Console) {
    arithm_a_operand::<OP, C>(console[Byte { idx: r8 }], console);
}

fn logic_a_r8<OP: BitFlag>(r8: u8, console: &mut Console) {
    logic_a_operand::<OP>(console[Byte { idx: r8 }], console);
}

fn cp_a_r8(r8: u8, console: &mut Console) {
    cp_a_operand(console[Byte { idx: r8 }], console);
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