use core::panic;

use log::debug;

use crate::console::{helpers::{bit_ops::{carry, half_carry}, common::{arithm_a_operand, cp_a_operand, logic_a_operand}, constants::{flag, reg16, reg16stk, reg8, IME}}, types::types::{BitFlag, Byte, Word, WordSTK, ADD, AND, CARRY, NO_CARRY, OR, SUB, XOR}, Console};

fn pop_low_high(console: &mut Console) -> (u16, u16) {
    let low: u16 = console.stk_pop() as u16;
    let high: u16 = console.stk_pop() as u16;
    (low, high)
}

fn arithm_a_imm8<OP: BitFlag, C: BitFlag>(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    arithm_a_operand::<OP, C>(imm8, console, reg8::MAX_REG8);
}

fn logic_a_imm8<OP: BitFlag>(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    logic_a_operand::<OP>(imm8, console);

    debug!("{} A, {}", OP::to_string(), imm8);
}

fn cp_a_imm8(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    cp_a_operand(imm8, console);

    debug!("CP A, {}", imm8);
}

fn ret(console: &mut Console) {
    let (low, high) = pop_low_high(console);
    console.set_ip(low | (high << 8));

    debug!("RET");
}

fn ret_cond(cc: u8, console: &mut Console) {
    if console.is_condition_met(cc) {
        ret(console);
    }

    debug!("RET cc");
}

fn reti(console: &mut Console) {
    ret(console);
    console.addr_bus[IME as usize] = 1;

    debug!("RETI");
}

fn jp_cc_imm16(cc: u8, console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    if console.is_condition_met(cc) {
        console.set_ip(imm16);
    }

    debug!("JP CC, {imm16}");
}

fn jp_imm16(console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    console.set_ip(imm16);

    debug!("JP {imm16}");
}

fn jp_hl(console: &mut Console) {
    let hl_val: u16 = console[Word { idx: reg16::HL }];
    console.set_ip(hl_val);

    debug!("JP HL");
}

fn setup_call(console: &mut Console) {
    let next_instr_addr: u16 = console.get_ip();
    console.stk_push((next_instr_addr & 0x00FF) as u8);
    console.stk_push((next_instr_addr >> 8) as u8);
}

fn call_imm16(console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    setup_call(console);
    console.set_ip(imm16);

    debug!("CALL {imm16}");
}

fn call_cc_imm16(cc: u8, console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    if console.is_condition_met(cc) {
        setup_call(console);
        console.set_ip(imm16);
    }

    debug!("CALL cc, {imm16}");
}

fn rst_tgt3(tgt3: u8, console: &mut Console) {
    setup_call(console);
    console.set_ip(tgt3 as u16);

    debug!("RST {tgt3}");
}

fn pop_r16stk(r16stk: u8, console: &mut Console) {
    let (low, high) = pop_low_high(console);
    let reg: &mut u16 = &mut console[WordSTK { idx: r16stk }];
    *reg = low | (high << 8);

    debug!("POP {}", reg16stk::reg_to_name(r16stk));
}

fn push_r16stk(r16stk: u8, console: &mut Console) {
    let val: u16 = console[WordSTK { idx: r16stk }];
    console.stk_push((val >> 8) as u8);
    console.stk_push((val & 0x00FF) as u8);

    debug!("PUSH {}", reg16stk::reg_to_name(r16stk));
}

fn ldh_c_a(console: &mut Console) {
    let a_val: u8 = console[Byte { idx: reg8::A }];
    let c_val: u8 = console[Byte { idx: reg8::C }];
    console.addr_bus[(0xFF00 + c_val as u16) as usize] = a_val;

    debug!("LDH C, A");
}

fn ldh_imm8_a(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    let a_val: u8 = console[Byte { idx: reg8::A }];
    console.addr_bus[(0xFF00 + imm8 as u16) as usize] = a_val;

    debug!("LDH [{imm8}], A");
}

fn ld_imm16_a(console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    let a_val: u8 = console[Byte { idx: reg8::A }];
    console.addr_bus[imm16 as usize] = a_val;

    debug!("LD [{imm16}], A");
}

fn load_mem_into_a(addr: u16, console: &mut Console) {
    let addr_val: u8 = console.addr_bus[addr as usize];
    let a_val: &mut u8 = &mut console[Byte { idx: reg8::A }];
    *a_val = addr_val;
}

fn ldh_a_c(console: &mut Console) {
    let c_val: u8 = console[Byte { idx: reg8::C }];
    load_mem_into_a(0xFF00 + c_val as u16, console);

    debug!("LDH A, C");
}

fn ldh_a_imm8(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    load_mem_into_a(0xFF00 + imm8 as u16, console);

    debug!("LDH A, {imm8}");
}

fn ld_a_imm16(console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    load_mem_into_a(imm16, console);

    debug!("LD A, {imm16}");
}

fn add_sp_imm8_logless(console: &mut Console, imm8: u8) -> u16 {
    console.clear_flags(&[flag::Z, flag::N]);
    let sp_val: u16 = console[Word { idx: reg16::SP }];
    console.clear_or_set_flag(half_carry::add_16(sp_val, imm8 as u16), flag::H);
    console.clear_or_set_flag(carry::add_16(sp_val, imm8 as u16), flag::C);
    *(&mut console[Word { idx: reg16::SP }]) = sp_val + imm8 as u16;
    sp_val + imm8 as u16
}

fn add_sp_imm8(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    add_sp_imm8_logless(console, imm8);

    debug!("ADD SP, {imm8}");
}

fn ld_hl_sp_imm8(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    let tmp: u16 = add_sp_imm8_logless(console, imm8);
    let hl_val: &mut u16 = &mut console[Word { idx: reg16::HL }];
    *hl_val = tmp;

    debug!("LD HL, SP+{imm8}")
}

fn ld_sp_hl(console: &mut Console) {
    let hl_val: u16 = console[Word { idx: reg16::HL }];
    *(&mut console[Word { idx: reg16::SP }]) = hl_val;

    debug!("LD SP, HL");
}

fn di(console: &mut Console) {
    console.addr_bus[IME as usize] = 0;

    debug!("DI");
}

fn ei(console: &mut Console) {
    console.pending_ei = true;

    debug!("EI");
}

pub fn dispatch(instr: u8, console: &mut Console) -> () {
    let cc: u8 = (instr << 3) >> 5;
    let tgt3: u8 = (instr << 2) >> 5;
    let r16stk: u8 = (instr << 2) >> 6;

    if instr == 198 {
        arithm_a_imm8::<ADD, NO_CARRY>(console);
    } else if instr == 206 {
        arithm_a_imm8::<ADD, CARRY>(console);
    } else if instr == 214 {
        arithm_a_imm8::<SUB, NO_CARRY>(console);
    } else if instr == 222 {
        arithm_a_imm8::<SUB, CARRY>(console);
    } else if instr == 230 {
        logic_a_imm8::<AND>(console);
    } else if instr == 238 {
        logic_a_imm8::<XOR>(console);
    } else if instr == 246 {
        logic_a_imm8::<OR>(console);
    } else if instr == 254 {
        cp_a_imm8(console);
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
        panic!("Invalid oipode for block three");
    }
}