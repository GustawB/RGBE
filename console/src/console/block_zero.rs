use crate::console::{helpers::{bit_ops::{carry, half_carry}, common::{debug_addr, rotate_operand}, constants::{cond, flag, reg16, reg16mem, reg8}}, types::{BitFlag, Byte, Word, CARRY, LEFT, NO_CARRY, RIGHT}, Console};

fn ld_r16_imm16(r16: u8, console: &mut Console, curr_ip: u16) {
    let imm16: u16 = console.fetch_two_bytes();
    let reg: &mut u16 = &mut console[Word { idx: r16 }];
    *reg = imm16;

    debug_addr(curr_ip, format!("LD {}, 0x{:04X}", reg16::reg_to_name(r16), imm16));
}

fn ld_r16mem_a(r16: u8, console: &mut Console, curr_ip: u16) {
    let r16mem_val: u16 = console.get_r16mem(r16);
    let a_val: &u8 = &console[Byte { idx: reg8::A }];
    console.addr_bus[r16mem_val as usize] = *a_val;

    debug_addr(curr_ip, format!("LD [{}], A", reg16mem::reg_to_name(r16)));
}

fn ld_a_r16mem(r16: u8, console: &mut Console, curr_ip: u16) {
    let r16_val: u16 = console.get_r16mem(r16);
    let addr_bus_val: u8 = console.addr_bus[r16_val as usize];
    let a_val: &mut u8 = &mut console[Byte { idx: reg8::A }];
    *a_val = addr_bus_val;

    debug_addr(curr_ip, format!("LD A, [{}]", reg16mem::reg_to_name(r16)));
}

fn ld_imm16_sp(console: &mut Console, curr_ip: u16) {
    let imm16: u16 = console.fetch_two_bytes();
    let sp_val: u16 = console[Word { idx: reg16::SP }];
    console.addr_bus[imm16 as usize] = (sp_val & 0xFF) as u8;
    console.addr_bus[(imm16 + 1) as usize] = (sp_val >> 8) as u8;

    debug_addr(curr_ip, format!("LD 0x{:04X}, SP", imm16));
}

fn inc_r16(r16: u8, console: &mut Console, curr_ip: u16) {
    let reg: &mut u16 = &mut console[Word { idx: r16 }];
    *reg += 1;

    debug_addr(curr_ip, format!("INC {}", reg16::reg_to_name(r16)));
}

fn dec_r16(r16: u8, console: &mut Console, curr_ip: u16) {
    let reg: &mut u16 = &mut console[Word { idx: r16 }];
    *reg -= 1;

    debug_addr(curr_ip, format!("INC {}", reg16::reg_to_name(r16)));
}

fn add_hl_r16(r16: u8, console: &mut Console, curr_ip: u16) {
    let r16_val: u16 = console[Word { idx: r16 }];
    let hl_val: &mut u16 = &mut console[Word { idx: reg16::HL }];
    let base: u16 = *hl_val;
    *hl_val += r16_val;
    console.clear_flag(flag::N);
    console.clear_or_set_flag(half_carry::add_16(base, r16_val), flag::H);
    console.clear_or_set_flag(carry::add_16(base, r16_val), flag::C);

    debug_addr(curr_ip, format!("ADD HL, {}", reg16::reg_to_name(r16)));
}

fn inc_r8(r8: u8, console: &mut Console, curr_ip: u16) {
    let reg: &mut u8 = &mut console[Byte { idx: r8 }];
    let base: u8 = *reg;
    *reg += 1;
    console.clear_or_set_flag((base + 1) == 0, flag::Z);
    console.clear_flag(flag::N);
    console.clear_or_set_flag(half_carry::add_8(base, 1), flag::H);

    debug_addr(curr_ip, format!("INC {}", reg8::reg_to_name(r8)));
}

fn dec_r8(r8: u8, console: &mut Console, curr_ip: u16) {
    let reg: &mut u8 = &mut console[Byte { idx: r8 }];
    let base: u8 = *reg;
    *reg -= 1;
    console.clear_or_set_flag((base - 1) == 0, flag::Z);
    console.set_flag(flag::N);
    console.clear_or_set_flag(half_carry::sub_8(base, 1), flag::H);

    debug_addr(curr_ip, format!("DEC {}", reg8::reg_to_name(r8)));
}

fn ld_r8_imm8(r8: u8, console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    let reg: &mut u8 = &mut console[Byte { idx: r8 }];
    *reg = imm8;

    debug_addr(curr_ip, format!("LD {}, 0x{:04X}", reg8::reg_to_name(r8), imm8));
}

fn rotate_a<DIR: BitFlag, C: BitFlag>(console: &mut Console, curr_ip: u16) {
    rotate_operand::<DIR, C>(reg8::EA, console, curr_ip);
}

fn daa(console: &mut Console, curr_ip: u16) {
    let mut adjustment: u8 = 0;
    let a_val: u8 = console[Byte { idx: reg8::A }];
    let n_flag: bool = console.is_flag_set(flag::N);
    if n_flag {
        if console.is_flag_set(flag::H) {
            adjustment += 0x6;
        }
        if console.is_flag_set(flag::C) {
            adjustment += 0x60;
        }
        console.clear_or_set_flag(a_val - adjustment == 0, flag::Z);
        console.clear_or_set_flag(carry::sub_8(a_val, adjustment), flag::C);
    }
    else {
        if console.is_flag_set(flag::H) || (a_val & 0xF) > 0x9 {
            adjustment += 0x6;
        }
        if console.is_flag_set(flag::C) || a_val > 0x99 {
            adjustment += 0x60;
        }
        console.clear_or_set_flag(a_val == 0, flag::Z);
        console.clear_or_set_flag(carry::add_8(a_val - adjustment, adjustment), flag::C);
    }
    console.clear_flag(flag::H);

    let a_val_mut: &mut u8 = &mut console[Byte { idx: reg8::A }];
    if n_flag { *a_val_mut -= adjustment; } else { *a_val_mut += adjustment; }

    debug_addr(curr_ip, format!("DAA"));
}

fn cpl(console: &mut Console, curr_ip: u16) {
    console.set_flags(&[flag::N, flag::H]);
    let a_val: &mut u8 = &mut console[Byte { idx: reg8::A }];
    *a_val = !(*a_val);

    debug_addr(curr_ip, format!("CPL"));
}

fn scf(console: &mut Console, curr_ip: u16) {
    console.clear_flags(&[flag::N, flag::H]);
    console.set_flag(flag::C);

    debug_addr(curr_ip, format!("SCF"));
}

fn ccf(console: &mut Console, curr_ip: u16) {
    console.clear_flags(&[flag::N, flag::H]);
    console.clear_or_set_flag(!console.is_flag_set(flag::C), flag::C);

    debug_addr(curr_ip, format!("CCF"));
}

fn jr_imm8(console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    console.move_ip(imm8);

    debug_addr(curr_ip, format!("JR 0x{:04X}", console.get_ip()));
}

fn jr_cc_imm8(cc: u8, console: &mut Console, curr_ip: u16) {
    let imm8: u8 = console.fetch_byte();
    if console.is_condition_met(cc) {
        console.move_ip(imm8);
    }

    debug_addr(curr_ip, format!("JR {}, 0x{:04X}",
                cond::get_cond_name(cc), console.get_ip()));
}

fn stop(console: &mut Console, curr_ip: u16) {
    console.fetch_byte();
    // TODO: implement

    debug_addr(curr_ip, format!("STOP"));
}

pub fn dispatch(console: &mut Console, instr: u8, curr_ip: u16) -> () {
    let r8: u8 = (instr << 2) >> 5;
    let r16: u8 = (instr << 2) >> 6;
    let cc: u8 = (instr << 3) >> 5;
    if instr & 0x0F == 1 {
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
    } else if instr == 55 {
        scf(console, curr_ip);
    } else if instr == 61 {
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