use macros::match_value;
use crate::{bit_ops::{carry, half_carry}, constants::*, types::*};

fn ld_r16_imm16(r16: u8, console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    let reg: &mut Value = &mut console.registers[RegSize::Word(r16)];
    match_value!(reg, Value::Word(r) => { **r = imm16; })
}

fn ld_r16mem_a(r16: u8, console: &mut Console) {
    let dest_reg: &Value = &console.registers[RegSize::Word(r16)];
    let a_reg: &Value = &console.registers[RegSize::Byte(A)];
    match_value!(dest_reg, Value::Word(r) => {
        match_value!(a_reg, Value::Byte(a) =>  {
            console.addrBus[(**r) as usize] = **a;
        })
    })
}

fn ld_a_r16mm(r16: u8, console: &mut Console) {
    let src_val: u16;
    let src_reg: &Value = &console.registers[RegSize::Word(r16)];
    match_value!(src_reg, Value::Word(r) => { src_val = **r; });
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    match_value!(a_reg, Value::Byte(a) => { **a = console.addrBus[src_val as usize]; })
}

fn ld_imm16_sp(console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    let sp_reg: &mut Value = &mut console.registers[RegSize::Word(SP)];
    match_value!(sp_reg, Value::Word(r) => {
        console.addrBus[imm16 as usize] = (**r & 0xFF) as u8;
        console.addrBus[(imm16 + 1) as usize] = (**r >> 8) as u8;
    })
}

fn inc_r16(r16: u8, console: &mut Console) {
    let reg: &mut Value = &mut console.registers[RegSize::Word(r16)];
    match_value!(reg, Value::Word(r) => { (**r) += 1; });
}

fn dec_r16(r16: u8, console: &mut Console) {
    let reg: &mut Value = &mut console.registers[RegSize::Word(r16)];
    match_value!(reg, Value::Word(r) => { (**r) -= 1; });
}

fn add_hl_r16(r16: u8, console: &mut Console) {
    let base: u16;
    let addend: u16;
    let src_reg: &Value = &console.registers[RegSize::Word(r16)];
    match_value!(src_reg, Value::Word(r) => { addend = **r; });
    let hl_reg: &mut Value = &mut console.registers[RegSize::Word(HL)];
    match_value!(hl_reg, Value::Word(hl) => {
        base = **hl;
        (**hl) += addend;
        console.registers.clear_flag(flag::N);
    });
    console.registers.clear_or_set_flag(half_carry::add_16(base, addend), flag::H);
    console.registers.clear_or_set_flag(carry::add_16(base, addend), flag::C);
}

fn inc_r8(r8: u8, console: &mut Console) {
    let base: u8;
    let reg: &mut Value = &mut console.registers[RegSize::Word(r8)];
    match_value!(reg, Value::Byte(r) => {
        base = **r;
        (**r) += 1;
        console.registers.clear_or_set_flag((base + 1) == 0, flag::Z);
        console.registers.clear_flag(flag::N);
    });
    console.registers.clear_or_set_flag(half_carry::add_8(base, 1), flag::H);
}

fn dec_r8(r8: u8, console: &mut Console) {
    let base: u8;
    let reg: &mut Value = &mut console.registers[RegSize::Word(r8)];
    match_value!(reg, Value::Byte(r) => {
        base = **r;
        (**r) -= 1;
        console.registers.clear_or_set_flag((base - 1) == 0, flag::Z);
        console.registers.set_flag(flag::N);
    });
    console.registers.clear_or_set_flag(half_carry::sub_8(base, 1), flag::H);
}

fn ld_r8_imm8(r8: u8, console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    let reg: &mut Value = &mut console.registers[RegSize::Byte(r8)];
    match_value!(reg, Value::Byte(r) => { **r = imm8; })
}

fn rlca(console: &mut Console) {
    console.registers.clear_flags(&[flag::Z, flag::N, flag::H]);
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let mut leftmost_bit: u8 = 0;
    match_value!(a_reg, Value::Byte(r) => {
        leftmost_bit = **r >> 7;
        **r = (**r << 1) | leftmost_bit;
    });
    console.registers.clear_or_set_flag(leftmost_bit == 0, flag::C);
}

fn rrca(console: &mut Console) {
    console.registers.clear_flags(&[flag::Z, flag::N, flag::H]);
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let rightmost_bit: u8;
    match_value!(a_reg, Value::Byte(r) => {
        rightmost_bit = **r << 7;
        **r = (**r >> 1) | (rightmost_bit << 7);
    });
    console.registers.clear_or_set_flag(rightmost_bit == 0, flag::C);
}

fn rla(console: &mut Console) {
    console.registers.clear_flags(&[flag::Z, flag::N, flag::H]);
    let c_bit = if console.registers.is_flag_set(flag::C) {1} else {0}; 
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let leftmost_bit: u8;
    match_value!(a_reg, Value::Byte(r) => {
        leftmost_bit = **r >> 7;
        **r = (**r << 1) | c_bit;
    });
    console.registers.clear_or_set_flag(leftmost_bit == 0, flag::C);
}

fn rra(console: &mut Console) {
    console.registers.clear_flags(&[flag::Z, flag::N, flag::H]);
    let c_bit = if console.registers.is_flag_set(flag::C) {1} else {0}; 
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let rightmost_bit: u8;
    match_value!(a_reg, Value::Byte(r) => {
        rightmost_bit = **r << 7;
        **r = (**r >> 1) | (c_bit << 7);
    });
    console.registers.clear_or_set_flag(rightmost_bit == 0, flag::C);
}

fn daa(console: &mut Console) {
    let mut adjustment: u8 = 0;
    let h_flag: bool = console.registers.is_flag_set(flag::H);
    let c_flag: bool = console.registers.is_flag_set(flag::C);
    let n_flag: bool = console.registers.is_flag_set(flag::N);
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    if n_flag {
        if h_flag {
            adjustment += 0x6;
        }
        if c_flag {
            adjustment += 0x60;
        }
        match_value!(a_reg, Value::Byte(r) => { **r -= adjustment; });
    }
    else {
        match_value!(a_reg, Value::Byte(r) => {
            if h_flag || (**r & 0xF) > 0x9 {
                adjustment += 0x6;
            }
            if c_flag || **r > 0x99 {
                adjustment += 0x60;
            }
            **r += adjustment;
        });
    }
    // TODO add setting of carry flag depending on the result.
}

fn cpl(console: &mut Console) {
    console.registers.set_flags(&[flag::N, flag::H]);
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    match_value!(a_reg, Value::Byte(r) => { **r = !(**r); })
}

fn scf(console: &mut Console) {
    console.registers.clear_flags(&[flag::N, flag::H]);
    console.registers.set_flag(flag::C);
}

fn ccf(console: &mut Console) {
    console.registers.clear_flags(&[flag::N, flag::H]);
    console.registers.clear_or_set_flag(!console.registers.is_flag_set(flag::C), flag::C);
}

fn jr_imm8(console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    console.move_pc(imm8 as i16);
}

fn jr_cc_imm8(cc: u8, console: &mut Console) {
    if console.registers.is_condition_met(cc) {
        jr_imm8(console);
    }
}

fn stop(console: &mut Console) {
    console.fetch_byte();
    // TODO: implement
}

pub fn dispatch(instr: u8, console: &mut Console) -> () {
    let op: u8 = (instr << 2) >> 4;
    if instr << 4 == 1 {
        ld_r16_imm16(op, console);
    } else if instr << 4 == 2 {
        ld_r16mem_a(op, console);
    } else if instr << 4 == 10 {
        ld_a_r16mm(op, console);
    } else if instr == 8 {
        ld_imm16_sp(console);
    } else if instr << 4 == 3 {
        inc_r16(op, console);
    } else if instr << 4 == 11 {
        dec_r16(op, console);
    } else if instr << 4 == 9 {
        add_hl_r16(op, console);        
    } else if instr << 3 == 4 {
        inc_r8(op, console);
    } else if instr << 3 == 5 {
        dec_r16(op, console);
    } else if instr << 3 == 6 {
        ld_r8_imm8(op, console);
    } else if instr == 7 {
        rlca(console);
    } else if instr == 15 {
        rrca(console);
    } else if instr == 23 {
        rla(console);
    } else if instr == 31 {
        rra(console);
    } else if instr == 39 {
        daa(console);
    } else if instr == 47 {
        cpl(console);
    } else if instr == 55 {
        scf(console);
    } else if instr == 61 {
        ccf(console);
    } else if instr == 24 {
        jr_imm8(console);
    } else if instr << 5 == 0 && instr >> 5 == 1 {
        jr_cc_imm8(op, console);
    } else if instr == 14 {
        stop(console);
    } else {
        panic!("Unrecognized OPCode in block zero.");
    }
    
}