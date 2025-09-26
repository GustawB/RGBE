use constants::{cond, flag, reg16, reg16mem, reg8};

use crate::console::{helpers::{bit_ops::{carry, half_carry}, common::{move_ip, rotate_operand}}, types::{BitFlag, CARRY, LEFT, NO_CARRY, RIGHT}, Console};

fn ld_r16_imm16(r16: u8, console: &mut Console, curr_ip: u16) {
    let imm16: u16 = console.fetch_two_bytes();
    console.call_hook(format!("LD {}, 0x{:04X}", reg16::reg_to_name(r16), imm16), curr_ip);

    console.set_r16(r16, imm16);
}

fn ld_r16mem_a(r16: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("LD [{}], A", reg16mem::reg_to_name(r16)), curr_ip);
    
    let r16mem_val: u16 = console.get_r16mem(r16);
    console.set_mem(r16mem_val as usize, console.get_r8(reg8::A));
}

fn ld_a_r16mem(r16: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("LD A, [{}]", reg16mem::reg_to_name(r16)), curr_ip);

    let r16_val: u16 = console.get_r16mem(r16);
    console.set_r8(reg8::A, console.get_mem(r16_val as usize));
}

fn ld_imm16_sp(console: &mut Console, curr_ip: u16) {
    let imm16: u16 = console.fetch_two_bytes();
    console.call_hook(format!("LD 0x{:04X}, SP", imm16), curr_ip);

    let sp_val: u16 = console.get_r16(reg16::SP);
    console.set_mem(imm16 as usize, (sp_val & 0xFF) as u8);
    console.set_mem((imm16.wrapping_add(1)) as usize, (sp_val >> 8) as u8);
}

fn inc_r16(r16: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("INC {}", reg16::reg_to_name(r16)), curr_ip);
    console.set_r16(r16,  console.get_r16(r16).wrapping_add(1));
}

fn dec_r16(r16: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("INC {}", reg16::reg_to_name(r16)), curr_ip);
    console.set_r16(r16,  console.get_r16(r16).wrapping_sub(1));
}

fn add_hl_r16(r16: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("ADD HL, {}", reg16::reg_to_name(r16)), curr_ip);

    let r16_val: u16 = console.get_r16(r16);
    let base: u16 = console.get_r16(reg16::HL);
    console.set_r16(reg16::HL, console.get_r16(reg16::HL).wrapping_add(r16_val));
    console.clear_flag(flag::N);
    console.clear_or_set_flag(half_carry::add_16(base, r16_val), flag::H);
    console.clear_or_set_flag(carry::add_16(base, r16_val), flag::C);
}

fn inc_r8(r8: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("INC {}", reg8::reg_to_name(r8)), curr_ip);

    let base: u8 = console.get_r8(r8);
    console.set_r8(r8, base.wrapping_add(1));
    console.clear_or_set_flag(base.wrapping_add(1) == 0, flag::Z);
    console.clear_flag(flag::N);
    console.clear_or_set_flag(half_carry::add_8(base, 1, 0), flag::H);
}

fn dec_r8(r8: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("DEC {}", reg8::reg_to_name(r8)), curr_ip);

    console.set_r8(r8, console.get_r8(r8).wrapping_sub(1));
    console.clear_or_set_flag(console.get_r8(r8) == 0, flag::Z);
    console.set_flag(flag::N);
    console.clear_or_set_flag(half_carry::sub_8(
                            console.get_r8(r8).wrapping_add(1), 1, 0), flag::H);
}

fn ld_r8_imm8(r8: u8, console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    console.call_hook(format!("LD {}, 0x{:04X}", reg8::reg_to_name(r8), imm8), curr_ip);

    console.set_r8(r8, imm8);
}

fn rotate_a<DIR: BitFlag, C: BitFlag>(console: &mut Console, curr_ip: u16) {
    rotate_operand::<DIR, C>(reg8::EA, console, curr_ip);
}

fn daa(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("DAA"), curr_ip);

    let mut adjustment: u8 = 0;
    let a_val: u8 = console.get_r8(reg8::A);
    let n_flag: bool = console.is_flag_set(flag::N);
    if n_flag {
        if console.is_flag_set(flag::H) {
            adjustment += 0x6;
        }
        if console.is_flag_set(flag::C) {
            adjustment += 0x60;
        }
        console.clear_or_set_flag(a_val.wrapping_sub(adjustment) == 0, flag::Z);
        // Ignoring carry because of // https://www.jnz.dk/z80/daa.html for the condition
        console.set_r8(reg8::A, a_val.wrapping_sub(adjustment));
    }
    else {
        if console.is_flag_set(flag::H) || (a_val & 0xF) > 0x9 {
            adjustment += 0x6;
        }
        if console.is_flag_set(flag::C) || a_val > 0x99 {
            adjustment += 0x60;
        }
        console.clear_or_set_flag(a_val.wrapping_add(adjustment) == 0, flag::Z);
        // https://www.jnz.dk/z80/daa.html for the condition
        console.clear_or_set_flag(adjustment >= 0x60, flag::C);
        console.set_r8(reg8::A, a_val.wrapping_add(adjustment));
    }
    console.clear_flag(flag::H);
}

fn cpl(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("CPL"), curr_ip);

    console.set_flags(&[flag::N, flag::H]);
    console.set_r8(reg8::A, !console.get_r8(reg8::A));
}

fn scf(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("SCF"), curr_ip);

    console.clear_flags(&[flag::N, flag::H]);
    console.set_flag(flag::C);
}

fn ccf(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("CCF"), curr_ip);

    console.clear_flags(&[flag::N, flag::H]);
    console.clear_or_set_flag(!console.is_flag_set(flag::C), flag::C);
}

fn jr_imm8(console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    let ip: u16 = console.get_ip();
    let new_ip: u16 = move_ip(ip, imm8);
    console.call_hook(format!("JR 0x{:04X}", new_ip), curr_ip);
    
    console.set_ip(new_ip);
}

fn jr_cc_imm8(cc: u8, console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    let ip: u16 = console.get_ip();
    let new_ip: u16 = move_ip(ip, imm8);
    console.call_hook(format!("JR {}, 0x{:04X}", cond::get_cond_name(cc), new_ip), curr_ip);
    
    if console.is_condition_met(cc) {
        console.set_ip(new_ip);
    }
}

fn stop(console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("STOP"), curr_ip);

    console.fetch_byte();
    // TODO: implement
}

pub fn dispatch(console: &mut Console, instr: u8, curr_ip: u16) -> () {
    let r8: u8 = (instr << 2) >> 5;
    let r16: u8 = (instr << 2) >> 6;
    let cc: u8 = (instr << 3) >> 6;
    if instr == 0 {
        console.call_hook(format!("NOP"), curr_ip);
    } else if instr & 0x0F == 1 {
        ld_r16_imm16(r16, console, curr_ip);
    } else if instr & 0x0F == 2 {
        ld_r16mem_a(r16, console, curr_ip);
    } else if instr & 0x0F == 10 {
        ld_a_r16mem(r16, console, curr_ip);
    } else if instr == 8 {
        ld_imm16_sp(console, curr_ip);
    } else if instr & 0x0F == 3 {
        inc_r16(r16, console, curr_ip);
    } else if instr & 0x0F == 11 {
        dec_r16(r16, console, curr_ip);
    } else if instr & 0x0F == 9 {
        add_hl_r16(r16, console, curr_ip);        
    } else if instr & 0x07 == 4 {
        inc_r8(r8, console, curr_ip);
    } else if instr & 0x07 == 5 {
        dec_r8(r8, console, curr_ip);
    } else if instr & 0x07 == 6 {
        ld_r8_imm8(r8, console, curr_ip);
    } else if instr == 7 {
        rotate_a::<LEFT, CARRY>(console, curr_ip);
    } else if instr == 15 {
        rotate_a::<RIGHT, CARRY>(console, curr_ip);
    } else if instr == 23 {
        rotate_a::<LEFT, NO_CARRY>(console, curr_ip);
    } else if instr == 31 {
        rotate_a::<RIGHT, NO_CARRY>(console, curr_ip);
    } else if instr == 39 {
        daa(console, curr_ip);
    } else if instr == 47 {
        cpl(console, curr_ip);
    } else if instr == 0x37 {
        scf(console, curr_ip);
    } else if instr == 0x3F {
        ccf(console, curr_ip);
    } else if instr == 24 {
        jr_imm8(console, curr_ip);
    } else if instr & 0x07 == 0 && instr >> 5 == 1 {
        jr_cc_imm8(cc, console, curr_ip);
    } else if instr == 14 {
        stop(console, curr_ip);
    } else {
        panic!("Invalid opcode in block zero.");
    }
    
}