use macros::{arg_register, match_value};
use crate::{bit_ops::{carry, half_carry}, common::rotate_operand, constants::*, types::*};

fn ld_r16_imm16(r16: u8, console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    let reg: &mut Value = &mut console.registers[RegSize::Word(r16)];
    match_value!(reg, Value::Word(r) => { **r = imm16; })
}

#[arg_register(r16)]
fn ld_r16mem_a(r16: u8, console: &mut Console) {
    let a_reg: &Value = &console.registers[RegSize::Byte(A)];
    match_value!(a_reg, Value::Byte(a) =>  {
        console.addr_bus[r16_val as usize] = **a;
    })
}

#[arg_register(r16)]
fn ld_a_r16mm(r16: u8, console: &mut Console) {
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    match_value!(a_reg, Value::Byte(a) => { **a = console.addr_bus[r16_val as usize]; })
}

fn ld_imm16_sp(console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    let sp_reg: &mut Value = &mut console.registers[RegSize::Word(SP)];
    match_value!(sp_reg, Value::Word(r) => {
        console.addr_bus[imm16 as usize] = (**r & 0xFF) as u8;
        console.addr_bus[(imm16 + 1) as usize] = (**r >> 8) as u8;
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

#[arg_register(r16)]
fn add_hl_r16(r16: u8, console: &mut Console) {
    let base: u16;
    let hl_reg: &mut Value = &mut console.registers[RegSize::Word(HL)];
    match_value!(hl_reg, Value::Word(hl) => {
        base = **hl;
        (**hl) += r16_val;
        console.registers.clear_flag(flag::N);
    });
    console.registers.clear_or_set_flag(half_carry::add_16(base, r16_val), flag::H);
    console.registers.clear_or_set_flag(carry::add_16(base, r16_val), flag::C);
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
    match_value!(reg, Value::Byte(r) => { **r = imm8; });
}

fn rotate_a<DIR: BitFlag, C: BitFlag>(console: &mut Console) {
    rotate_operand::<DIR, C>(EA, console);
}

fn daa(console: &mut Console) {
    let mut adjustment: u8 = 0;
    let h_flag: bool = console.registers.is_flag_set(flag::H);
    let c_flag: bool = console.registers.is_flag_set(flag::C);
    let n_flag: bool = console.registers.is_flag_set(flag::N);
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let base: u8;
    if n_flag {
        if h_flag {
            adjustment += 0x6;
        }
        if c_flag {
            adjustment += 0x60;
        }
        match_value!(a_reg, Value::Byte(r) => {
            base = **r;
            **r -= adjustment;
        });
        console.registers.clear_or_set_flag(base - adjustment == 0, flag::Z);
        console.registers.clear_or_set_flag(carry::sub_8(base, adjustment), flag::C);
    }
    else {
        match_value!(a_reg, Value::Byte(r) => {
            if h_flag || (**r & 0xF) > 0x9 {
                adjustment += 0x6;
            }
            if c_flag || **r > 0x99 {
                adjustment += 0x60;
            }
            base = **r;
            **r += adjustment;
        });
        console.registers.clear_or_set_flag(base + adjustment == 0, flag::Z);
        console.registers.clear_or_set_flag(carry::add_8(base, adjustment), flag::C);
    }
    console.registers.clear_flag(flag::H);
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
    console.move_pc(imm8 as u16);
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
    let op: u8 = (instr << 2) >> 6;
    if instr & 0x0F == 1 {
        ld_r16_imm16(op, console);
    } else if instr & 0x0F == 2 {
        ld_r16mem_a(op, console);
    } else if instr & 0x0F == 10 {
        ld_a_r16mm(op, console);
    } else if instr == 8 {
        ld_imm16_sp(console);
    } else if instr & 0x0F == 3 {
        inc_r16(op, console);
    } else if instr & 0x0F == 11 {
        dec_r16(op, console);
    } else if instr & 0x0F == 9 {
        add_hl_r16(op, console);        
    } else if instr & 0x07 == 4 {
        inc_r8(op, console);
    } else if instr & 0x07 == 5 {
        dec_r16(op, console);
    } else if instr & 0x07 == 6 {
        ld_r8_imm8(op, console);
    } else if instr == 7 {
        rotate_a::<LEFT, CARRY>(console);
    } else if instr == 15 {
        rotate_a::<RIGHT, CARRY>(console);
    } else if instr == 23 {
        rotate_a::<LEFT, NO_CARRY>(console);
    } else if instr == 31 {
        rotate_a::<RIGHT, NO_CARRY>(console);
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
    } else if instr & 0x07 == 0 && instr >> 5 == 1 {
        jr_cc_imm8(op, console);
    } else if instr == 14 {
        stop(console);
    } else {
        panic!("Unrecognized OPCode in block zero.");
    }
    
}