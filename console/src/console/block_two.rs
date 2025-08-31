use core::panic;

use crate::console::{helpers::{common::{arithm_a_operand, cp_a_operand, logic_a_operand}, constants::reg8}, types::{BitFlag, Byte, ADD, AND, CARRY, NO_CARRY, OR, SUB, XOR}, Console};

fn arithm_a_r8<OP: BitFlag, C: BitFlag>(r8: u8, console: &mut Console, curr_ip: u16) {
    arithm_a_operand::<OP, C>(console[Byte { idx: r8 }], console, r8, curr_ip);
}

fn logic_a_r8<OP: BitFlag>(r8: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("{} A, {}", OP::to_string(), reg8::reg_to_name(r8)), curr_ip);
    
    logic_a_operand::<OP>(console[Byte { idx: r8 }], console);
}

fn cp_a_r8(r8: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("CP A, {}", reg8::reg_to_name(r8)), curr_ip);

    cp_a_operand(console[Byte { idx: r8 }], console);
}

pub fn dispatch(console: &mut Console, instr: u8, curr_ip: u16) -> () {
    let r8: u8 = instr & 0x07;
    let op: u8 = (instr << 2) >> 5;
    match op {
        0 => arithm_a_r8::<ADD, NO_CARRY>(r8, console, curr_ip),
        1 => arithm_a_r8::<ADD, CARRY>(r8, console, curr_ip),
        2 => arithm_a_r8::<SUB, NO_CARRY>(r8, console, curr_ip),
        3 => arithm_a_r8::<SUB, CARRY>(r8, console, curr_ip),
        4 => logic_a_r8::<AND>(r8, console, curr_ip),
        5 => logic_a_r8::<XOR>(r8, console, curr_ip),
        6 => logic_a_r8::<OR>(r8, console, curr_ip),
        7 => cp_a_r8(r8, console, curr_ip),
        _ => panic!("Invalid opcode in block two"),
    }
}