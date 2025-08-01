use macros::{arg_register, match_value};

use crate::{common::{arithm_a_operand, cp_a_operand, logic_a_operand}, constants::{BitFlag, A, C, HL}, types::{Console, RegSize, Value}};

fn pop_low_high(console: &mut Console) -> (u16, u16) {
    let low: u16 = console.stk_pop() as u16;
    let high: u16 = console.stk_pop() as u16;
    (low, high)
}

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

fn ret(console: &mut Console) {
    let (low, high) = pop_low_high(console);
    console.set_pc(low | (high << 8));
}

fn ret_cond(cc: u8, console: &mut Console) {
    if console.registers.is_condition_met(cc) {
        ret(console);
    }
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

fn setup_call(console: &mut Console) {
    let next_instr_addr: u16 = console.get_pc();
    console.stk_push((next_instr_addr & 0x00FF) as u8);
    console.stk_push((next_instr_addr >> 8) as u8);
}

fn call_imm16(console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    setup_call(console);
    console.set_pc(imm16);
}

fn call_cc_imm16(cc: u8, console: &mut Console) {
    if console.registers.is_condition_met(cc) {
        call_imm16(console);
    }
}

fn rst_tgt3(tgt3: u8, console: &mut Console) {
    setup_call(console);
    console.set_pc(tgt3 as u16);
}

fn pop_r16stk(r16stk: u8, console: &mut Console) {
    let (low, high) = pop_low_high(console);
    let reg: &mut Value = &mut console.registers[RegSize::WordSTK(r16stk)];
    match_value!(reg, Value::WordSTK(r) => { **r = low | (high << 8); });
}

fn push_r16stk(r16stk: u8, console: &mut Console) {
    let reg: &Value = &console.registers[RegSize::WordSTK(r16stk)];
    let val: u16;
    match_value!(reg, Value::WordSTK(r) => { val = **r; });
    console.stk_push((val >> 8) as u8);
    console.stk_push((val & 0x00FF) as u8);
}

#[arg_register(a)] #[arg_register(c)]
fn ldh_c_a(console: &mut Console) {
    console.addrBus[(0xFF00 + c_val as u16) as usize] = a_val;
}