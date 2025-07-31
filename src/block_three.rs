use macros::match_value;

use crate::{common::{arithm_a_operand, cp_a_operand, logic_a_operand}, constants::{BitFlag, HL}, types::{Console, RegSize, Value}};

fn arithm_a_r8<OP: BitFlag, C: BitFlag>(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    arithm_a_operand::<OP, C>(imm8, console);
}

fn logic_a_r8<OP: BitFlag>(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    logic_a_operand::<OP>(imm8, console);
}

fn cp_a_r8(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    cp_a_operand(imm8, console);
}

fn ret_cond(cc: u8, console: &mut Console) {
    // TODO: implement
}

fn ret(console: &mut Console) {
    // TODO: implement
}

fn reti(console: &mut Console) {
    // TODO: implement
}

fn jp_cc_imm16(cc: u8, console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    if console.registers.is_condition_met(cc) {
        console.set_pc(imm16);
    }
}

fn jp_imm16(console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    console.set_pc(imm16);
}

fn jp_hl(console: &mut Console) {
    let hl_val: u16;
    let hl_reg: &Value = &console.registers[RegSize::Word(HL)];
    match_value!(hl_reg, Value::Word(hl) => {hl_val = **hl});
    console.set_pc(hl_val);
}

fn call_cc_imm16(cc: u8, console: &mut Console) {
    // TODO: implement
}

fn call_imm16(console: &mut Console) {
    // TODO: implement
}

fn rst_tgt3(console: &mut Console) {
    // TODO: implement
}

fn pop_r16stk(r16stk: u8, console: &mut Console) {
    let reg: &mut Value = &mut console.registers[RegSize::WordSTK(r16stk)];
    match_value!(reg, Value::WordSTK(r) => {
        let new_val: u16 = (console.stk_pop() as u16) | ((console.stk_pop() as u16) << 8);
        **r = new_val;
    });
}

fn push_r16stk(r16stk: u8, console: &mut Console) {
    let reg: &Value = &console.registers[RegSize::WordSTK(r16stk)];
    let val: u16;
    match_value!(reg, Value::WordSTK(r) => { val = **r; });
    console.stk_push((val >> 8) as u8);
    console.stk_push((val & 0x00FF) as u8);
}