use core::panic;

use macros::{arg_register, match_value};

use crate::{bit_ops::{carry, half_carry}, common::{arithm_a_operand, cp_a_operand, logic_a_operand}, constants::{flag, BitFlag, A, ADD, AND, C, CARRY, HL, IME, NO_CARRY, OR, SP, SUB, XOR}, types::{Console, RegSize, Value}};

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
    ret(console);
    console.addrBus[IME as usize] = 1;
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

#[arg_register(a)]
fn ldh_imm8_a(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    console.addrBus[(0xFF00 + imm8 as u16) as usize] = a_val;
}

#[arg_register(a)]
fn ld_imm16_a(console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    console.addrBus[imm16 as usize] = a_val;
}

fn load_mem_into_a(addr: u16, console: &mut Console) {
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    match_value!(a_reg, Value::Byte(r) => {
        **r = console.addrBus[addr as usize];
    });
}

#[arg_register(c)]
fn ldh_a_c(console: &mut Console) {
    load_mem_into_a(0xFF00 + c_val as u16, console);
}

fn ldh_a_imm8(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    load_mem_into_a(0xFF00 + imm8 as u16, console);
}

fn ld_a_imm16(console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    load_mem_into_a(imm16, console);
}

fn add_sp_imm8(console: &mut Console) -> u16 {
    console.registers.clear_flags(&[flag::Z, flag::N]);
    let imm8: u8 = console.fetch_byte();
    let sp_reg: &mut Value = &mut console.registers[RegSize::Word(SP)];
    let base: u16;
    match_value!(sp_reg, Value::Word(r) => {
        base = **r;
        **r += imm8 as u16;
        console.registers.clear_or_set_flag(half_carry::add_16(base, imm8 as u16), flag::H);
        console.registers.clear_or_set_flag(carry::add_16(base, imm8 as u16), flag::C);
    });
    base + imm8 as u16
}

fn ld_hl_sp_imm8(console: &mut Console) {
    let tmp: u16 = add_sp_imm8(console);
    let hl_reg: &mut Value = &mut console.registers[RegSize::Word(HL)];
    match_value!(hl_reg, Value::Word(r) => { **r = tmp; });
}

#[arg_register(hl)]
fn ld_sp_hl(console: &mut Console) {
    let sp_reg: &mut Value = &mut console.registers[RegSize::Word(SP)];
    match_value!(sp_reg, Value::Word(r) => { **r = hl_val; });
}

fn di(console: &mut Console) {
    console.addrBus[IME as usize] = 0;
}

fn ei(console: &mut Console) {
    console.pending_ei = true;
}

pub fn dispatch(instr: u8, console: &mut Console) -> () {
    let cc: u8 = (instr << 3) >> 5;
    let tgt3: u8 = (instr << 2) >> 5;
    let r16stk: u8 = (instr << 2) >> 6;

    if instr == 198 {
        arithm_a_r8::<ADD, NO_CARRY>(console);
    } else if instr == 206 {
        arithm_a_r8::<ADD, CARRY>(console);
    } else if instr == 214 {
        arithm_a_r8::<SUB, NO_CARRY>(console);
    } else if instr == 222 {
        arithm_a_r8::<SUB, CARRY>(console);
    } else if instr == 230 {
        logic_a_r8::<AND>(console);
    } else if instr == 238 {
        logic_a_r8::<XOR>(console);
    } else if instr == 246 {
        logic_a_r8::<OR>(console);
    } else if instr == 254 {
        cp_a_r8(console);
    } else if instr & 0x18 == 192 {
        ret_cond(cc, console);
    } else if instr == 201 {
        ret(console);
    } else if instr == 217 {
        reti(console);
    } else if instr & 0x18 == 194 {
        jp_cc_imm16(cc, console);
    } else if instr == 195 {
        jp_imm16(console);
    } else if instr == 233 {
        jp_hl(console);
    } else if instr & 0x18 == 196 {
        call_cc_imm16(cc, console);
    } else if instr == 205 {
        call_imm16(console);
    } else if instr & 0x38 == 199 {
        rst_tgt3(tgt3, console);
    } else if instr & 0x00FF == 1 {
        pop_r16stk(r16stk, console);
    } else if instr & 0x00FF == 5 {
        push_r16stk(r16stk, console);
    } else if instr == 226 {
        ldh_c_a(console);
    } else if instr == 224 {
        ldh_imm8_a(console);
    } else if instr == 234 {
        ld_imm16_a(console);
    } else if instr == 242 {
        ldh_a_c(console);
    } else if instr == 240 {
        ldh_a_imm8(console);
    } else if instr == 250 {
        ld_a_imm16(console);
    } else if instr == 232 {
        add_sp_imm8(console);
    } else if instr == 248 {
        ld_hl_sp_imm8(console);
    } else if instr == 249 {
        ld_sp_hl(console);
    } else if instr == 243 {
        di(console);
    } else if instr == 251 {
        ei(console);
    } else {
        panic!("Invalid opcode for block three");
    }
}