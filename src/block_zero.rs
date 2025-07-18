use macros::match_value;
use crate::{constants::*, types::*};

fn ld_r16_imm16(r16: u8, console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    let reg: &mut Value = &mut console.registers[RegSize::Word(r16)];
    match_value!(reg, Value::Word(r) => { **r = imm16; });
}

fn ld_r16mem_a(r16: u8, console: &mut Console) {
    let dest_reg: &Value = &console.registers[RegSize::Word(r16)];
    let a_reg: &Value = &console.registers[RegSize::Byte(7)];
    match dest_reg {
        Value::Word(r) => 
            match a_reg {
                Value::Byte(a) => console.addrBus[(**r) as usize] = **a,
                _ => panic!("Invalid register size returned"),
            }
        _ => panic!("Invalid register size returned"),
    }
}

fn ld_a_r16mm(r16: u8, console: &mut Console) {
    let src_val: u16;
    {
        let src_reg: &Value = &console.registers[RegSize::Word(r16)];
        match src_reg {
            Value::Word(r) => src_val = **r,
            _ => panic!("Invalid register size returned"),
        }
    }
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(7)];
    match a_reg {
        Value::Byte(a) => **a = console.addrBus[src_val as usize],
        _ => panic!("Invalid register size returned"),
    }
}

fn ld_imm16_sp(r16: u8, console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    let sp_reg: &mut Value = &mut console.registers[RegSize::Word(SP)];
    match sp_reg {
        Value::Word(r) => **r = imm16,
        _ => panic!("Invalid register size returned"),
    }
}

fn inc_r16(r16: u8, console: &mut Console) {
    let reg: &mut Value = &mut console.registers[RegSize::Word(r16)];
    match reg {
        Value::Word(r) => (**r) += 1,
        _ => panic!("Invalid register size returned"),
    }
}

fn dec_r16(r16: u8, console: &mut Console) {
    let reg: &mut Value = &mut console.registers[RegSize::Word(r16)];
    match reg {
        Value::Word(r) => (**r) -= 1,
        _ => panic!("Invalid register size returned"),
    }
}

fn add_hl_r16(r16: u8, console: &mut Console) {
    let src_val: u16;
    {
        let src_reg: &Value = &console.registers[RegSize::Word(r16)];
        match src_reg {
            Value::Word(r) => src_val = **r,
            _ => panic!("Invalid register size returned"),
        }
    }
    let hl_reg: &mut Value = &mut console.registers[RegSize::Word(HL)];
    match hl_reg {
        Value::Word(hl) => (**hl) += src_val,
    }
}

fn inc_r8(r8: u8, console: &mut Console) {
    let reg: &mut Value = &mut console.registers[RegSize::Word(r8)];
    match reg {
        Value::Byte(r) => (**r) += 1,
        _ => panic!("Invalid register size returned"),
    }
}

fn dec_r8(r8: u8, console: &mut Console) {
    let reg: &mut Value = &mut console.registers[RegSize::Word(r8)];
    match reg {
        Value::Byte(r) => (**r) += 1,
        _ => panic!("Invalid register size returned"),
    }
}

fn ld_r8_imm8(r8: u8, console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    let reg: &mut Value = &mut console.registers[RegSize::Byte(r8)];
    match reg {
        Value::Byte(r) => **r = imm8,
        _ => panic!("Invalid register size returned"),
    }
}

fn rlca() {
    
}

pub fn dispatch(instr: u8, console: &mut Console) -> () {
    if instr << 4 == 1 {

    } else if instr << 4 == 2 {

    } else if instr << 4 == 3 {
        
    } else if instr << 4 == 8 {
        
    } else if instr << 4 == 9 {
        
    } else if instr << 4 == 10 {
        
    } else if instr << 4 == 11 {
        
    }
    
}