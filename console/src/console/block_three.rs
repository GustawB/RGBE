use core::panic;

use constants::{cond, flag, reg16, reg16stk, reg8};

use crate::console::{helpers::{bit_ops::{carry, half_carry}, common::{arithm_a_operand, cp_a_operand, logic_a_operand}}, types::{BitFlag, ADD, AND, CARRY, NO_CARRY, OR, SUB, XOR}, Console};

fn arithm_a_imm8<OP: BitFlag, C: BitFlag>(console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    arithm_a_operand::<OP, C>(imm8, console, reg8::MAX_REG8 as u8, curr_ip);
}

fn logic_a_imm8<OP: BitFlag>(console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    console.call_hook(format!("{} A, 0x{:04X}", OP::to_string(), imm8), curr_ip);

    logic_a_operand::<OP>(imm8, console);
}

fn cp_a_imm8(console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    console.call_hook(format!("CP A, 0x{:04X}", imm8), curr_ip);

    cp_a_operand(imm8, console);
}

fn ret(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("RET"), curr_ip);
    let ip: u16 = console.stk_pop16();
    console.set_ip(ip);
}

fn ret_cond(cc: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("RET cc"), curr_ip);

    if console.is_condition_met(cc) {
        ret(console, curr_ip);
    }
}

fn reti(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("RETI"), curr_ip);

    ret(console, curr_ip);
    console.set_ime(1);
}

fn jp_cc_imm16(cc: u8, console: &mut Console, curr_ip: u16) {
    let imm16: u16 = console.fetch_two_bytes();
    console.call_hook(format!("JP {}, 0x{:04X}", cond::get_cond_name(cc), imm16), curr_ip);

    if console.is_condition_met(cc) {
        console.set_ip(imm16);
    }
}

fn jp_imm16(console: &mut Console, curr_ip: u16) {
    let imm16: u16 = console.fetch_two_bytes();
    console.call_hook(format!("JP 0x{:04X}", imm16), curr_ip);
    console.set_ip(imm16);
}

fn jp_hl(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("JP HL"), curr_ip);

    let hl_val: u16 = console.get_r16(reg16::HL);
    console.set_ip(hl_val);
}

fn setup_call(console: &mut Console) {
    let next_instr_addr: u16 = console.get_ip();
    console.stk_push16(next_instr_addr);
}

fn call_imm16(console: &mut Console, curr_ip: u16) {
    let imm16: u16 = console.fetch_two_bytes();
    console.call_hook(format!("CALL 0x{:04X}", imm16), curr_ip);

    setup_call(console);
    console.set_ip(imm16);
}

fn call_cc_imm16(cc: u8, console: &mut Console, curr_ip: u16) {
    let imm16: u16 = console.fetch_two_bytes();
    console.call_hook(format!("CALL {}, 0x{:04X}", cond::get_cond_name(cc), imm16), curr_ip);

    if console.is_condition_met(cc) {
        setup_call(console);
        console.set_ip(imm16);
    }
}

fn rst_tgt3(tgt3: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("RST {tgt3}"), curr_ip);
    setup_call(console);
    console.set_ip(tgt3 as u16);
}

fn pop_r16stk(r16stk: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("POP {}", reg16stk::reg_to_name(r16stk)), curr_ip);

    let popped: u16 = console.stk_pop16();
    console.set_r16stk(r16stk, popped);
}

fn push_r16stk(r16stk: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("PUSH {}", reg16stk::reg_to_name(r16stk)), curr_ip);

    let val: u16 = console.get_r16stk(r16stk);
    console.stk_push16(val);
}

fn ldh_c_a(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("LDH [C], A"), curr_ip);

    let a_val: u8 = console.get_r8(reg8::A);
    let c_val: u8 = console.get_r8(reg8::C);
    console.set_mem((0xFF00 + c_val as u16) as usize, a_val);
}

fn ldh_imm8_a(console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    console.call_hook(format!("LDH [0x{:04X}], A", imm8), curr_ip);

    let a_val: u8 = console.get_r8(reg8::A);
    console.set_mem((0xFF00 + imm8 as u16) as usize, a_val);
}

fn ld_imm16_a(console: &mut Console, curr_ip: u16) {
    let imm16: u16 = console.fetch_two_bytes();
    console.call_hook(format!("LD [0x{:04X}], A", imm16), curr_ip);

    let a_val: u8 = console.get_r8(reg8::A);
    console.set_mem(imm16 as usize, a_val);
}

fn load_mem_into_a(addr: u16, console: &mut Console) {
    let addr_val: u8 = console.get_mem(addr as usize);
    console.set_r8(reg8::A, addr_val);
}

fn ldh_a_c(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("LDH A, [C]"), curr_ip);

    let c_val: u8 = console.get_r8(reg8::C);
    load_mem_into_a(0xFF00 + c_val as u16, console);
}

fn ldh_a_imm8(console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    console.call_hook(format!("LDH A, [0x{:04X}]", imm8), curr_ip);

    load_mem_into_a(0xFF00 + imm8 as u16, console);
}

fn ld_a_imm16(console: &mut Console, curr_ip: u16) {
    let imm16: u16 = console.fetch_two_bytes();
    console.call_hook(format!("LD A, [0x{:04X}]", imm16), curr_ip);

    load_mem_into_a(imm16, console);
}

fn add_sp_imm8_logless(console: &mut Console, imm8: u8) -> u16 {
    console.clear_flags(&[flag::Z, flag::N]);
    let sp_val: u16 = console.get_r16(reg16::SP);
    console.clear_or_set_flag(half_carry::add_16(sp_val, imm8 as u16), flag::H);
    console.clear_or_set_flag(carry::add_16(sp_val, imm8 as u16), flag::C);
    console.set_r16(reg16::SP, sp_val + imm8 as u16);
    sp_val + imm8 as u16
}

fn add_sp_imm8(console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    console.call_hook(format!("ADD SP, 0x{:04X}", imm8), curr_ip);

    add_sp_imm8_logless(console, imm8);
}

fn ld_hl_sp_imm8(console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    console.call_hook(format!("LD HL, SP+0x{:04X}", imm8), curr_ip);

    let tmp: u16 = add_sp_imm8_logless(console, imm8);
    console.set_r16(reg16::HL, tmp);
}

fn ld_sp_hl(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("LD SP, HL"), curr_ip);

    let hl_val: u16 = console.get_r16(reg16::HL);
    console.set_r16(reg16::SP, hl_val);
}

fn di(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("DI"), curr_ip);
    console.set_ime(0);
}

fn ei(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("EI"), curr_ip);
    console.pending_ei = true;
}

pub fn dispatch(console: &mut Console, instr: u8, curr_ip: u16) -> () {
    let cc: u8 = (instr << 3) >> 5;
    let tgt3: u8 = (instr << 2) >> 5;
    let r16stk: u8 = (instr << 2) >> 6;

    if instr == 198 {
        arithm_a_imm8::<ADD, NO_CARRY>(console, curr_ip);
    } else if instr == 206 {
        arithm_a_imm8::<ADD, CARRY>(console, curr_ip);
    } else if instr == 214 {
        arithm_a_imm8::<SUB, NO_CARRY>(console, curr_ip);
    } else if instr == 222 {
        arithm_a_imm8::<SUB, CARRY>(console, curr_ip);
    } else if instr == 230 {
        logic_a_imm8::<AND>(console, curr_ip);
    } else if instr == 238 {
        logic_a_imm8::<XOR>(console, curr_ip);
    } else if instr == 246 {
        logic_a_imm8::<OR>(console, curr_ip);
    } else if instr == 254 {
        cp_a_imm8(console, curr_ip);
    } else if instr & 0x18 == 192 {
        ret_cond(cc, console, curr_ip);
    } else if instr == 201 {
        ret(console, curr_ip);
    } else if instr == 217 {
        reti(console, curr_ip);
    } else if instr & 0x18 == 194 {
        jp_cc_imm16(cc, console, curr_ip);
    } else if instr == 195 {
        jp_imm16(console, curr_ip);
    } else if instr == 233 {
        jp_hl(console, curr_ip);
    } else if instr & 0x18 == 196 {
        call_cc_imm16(cc, console, curr_ip);
    } else if instr == 205 {
        call_imm16(console, curr_ip);
    } else if instr & 0x38 == 199 {
        rst_tgt3(tgt3, console, curr_ip);
    } else if instr & 0x0F == 1 {
        pop_r16stk(r16stk, console, curr_ip);
    } else if instr & 0x0F == 5 {
        push_r16stk(r16stk, console, curr_ip);
    } else if instr == 226 {
        ldh_c_a(console, curr_ip);
    } else if instr == 224 {
        ldh_imm8_a(console, curr_ip);
    } else if instr == 234 {
        ld_imm16_a(console, curr_ip);
    } else if instr == 242 {
        ldh_a_c(console, curr_ip);
    } else if instr == 240 {
        ldh_a_imm8(console, curr_ip);
    } else if instr == 250 {
        ld_a_imm16(console, curr_ip);
    } else if instr == 232 {
        add_sp_imm8(console, curr_ip);
    } else if instr == 248 {
        ld_hl_sp_imm8(console, curr_ip);
    } else if instr == 249 {
        ld_sp_hl(console, curr_ip);
    } else if instr == 243 {
        di(console, curr_ip);
    } else if instr == 251 {
        ei(console, curr_ip);
    } else {
        let _sex = instr & 0x00FF; 
        panic!("Invalid opcode in block three");
    }
}