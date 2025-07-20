use macros::match_value;
use crate::{constants::*, types::*};

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
    {
        let src_reg: &Value = &console.registers[RegSize::Word(r16)];
        match_value!(src_reg, Value::Word(r) => { src_val = **r; });
    }
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
    let prev_val: u16;
    let src_val: u16;
    {
        let src_reg: &Value = &console.registers[RegSize::Word(r16)];
        match_value!(src_reg, Value::Word(r) => { src_val = **r; });
    }
    let hl_reg: &mut Value = &mut console.registers[RegSize::Word(HL)];
    match_value!(hl_reg, Value::Word(hl) => {
        prev_val = **hl;
        (**hl) += src_val;
        console.registers.clear_flag(N);
    });
    if ((prev_val & 0x0FFF) + (src_val & 0x0FFF)) & 0x1000 != 0 {
        console.registers.set_flag(H);
    }
    if ((prev_val as u32) + (src_val as u32)) > 0xFFFF {
        console.registers.set_flag(C);
    }
}

fn inc_r8(r8: u8, console: &mut Console) {
    let prev_val: u8;
    let reg: &mut Value = &mut console.registers[RegSize::Word(r8)];
    match_value!(reg, Value::Byte(r) => {
        prev_val = **r;
        (**r) += 1;
        if (**r) == 0 {
            console.registers.set_flag(Z);
        }
        console.registers.clear_flag(N);
    });
    if ((prev_val & 0xF) + (1 & 0xF)) & 0x10 != 0 {
        console.registers.set_flag(H);
    }
}

fn dec_r8(r8: u8, console: &mut Console) {
    let prev_val: u8;
    let reg: &mut Value = &mut console.registers[RegSize::Word(r8)];
    match_value!(reg, Value::Byte(r) => {
        prev_val = **r;
        (**r) -= 1;
        if (**r) == 0 {
            console.registers.set_flag(Z);
        }
        console.registers.set_flag(N);
    });
    if (prev_val & 0x0F) < (1 & 0x0F) {
        console.registers.set_flag(H);
    }
}

fn ld_r8_imm8(r8: u8, console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    let reg: &mut Value = &mut console.registers[RegSize::Byte(r8)];
    match_value!(reg, Value::Byte(r) => { **r = imm8; })
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