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
    console.registers.clear_or_set_flag(((prev_val & 0x0FFF) + (src_val & 0x0FFF)) & 0x1000 != 0, H);
    console.registers.clear_or_set_flag(((prev_val as u32) + (src_val as u32)) > 0xFFFF, C);
}

fn inc_r8(r8: u8, console: &mut Console) {
    let prev_val: u8;
    let reg: &mut Value = &mut console.registers[RegSize::Word(r8)];
    match_value!(reg, Value::Byte(r) => {
        prev_val = **r;
        (**r) += 1;
        console.registers.clear_or_set_flag((prev_val + 1) == 0, Z);
        console.registers.clear_flag(N);
    });
    console.registers.clear_or_set_flag(((prev_val & 0xF) + (1 & 0xF)) & 0x10 != 0, H);
}

fn dec_r8(r8: u8, console: &mut Console) {
    let prev_val: u8;
    let reg: &mut Value = &mut console.registers[RegSize::Word(r8)];
    match_value!(reg, Value::Byte(r) => {
        prev_val = **r;
        (**r) -= 1;
        console.registers.clear_or_set_flag((prev_val - 1) == 0, Z);
        console.registers.set_flag(N);
    });
    console.registers.clear_or_set_flag((prev_val & 0x0F) < (1 & 0x0F), H);
}

fn ld_r8_imm8(r8: u8, console: &mut Console) {
    let imm8: u8 = console.fetch_byte();
    let reg: &mut Value = &mut console.registers[RegSize::Byte(r8)];
    match_value!(reg, Value::Byte(r) => { **r = imm8; })
}

fn rlca(console: &mut Console) {
    console.registers.clear_flags(&[Z, N, H]);
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let mut leftmost_bit: u8 = 0;
    match_value!(a_reg, Value::Byte(r) => {
        leftmost_bit = **r >> 7;
        **r = (**r << 1) | leftmost_bit;
    });
    console.registers.clear_or_set_flag(leftmost_bit == 0, C);
}

fn rrca(console: &mut Console) {
    console.registers.clear_flags(&[Z, N, H]);
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let rightmost_bit: u8;
    match_value!(a_reg, Value::Byte(r) => {
        rightmost_bit = **r << 7;
        **r = (**r >> 1) | (rightmost_bit << 7);
    });
    console.registers.clear_or_set_flag(rightmost_bit == 0, C);
}

fn rla(console: &mut Console) {
    console.registers.clear_flags(&[Z, N, H]);
    let c_bit = if console.registers.is_flag_set(C) {1} else {0}; 
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let leftmost_bit: u8;
    match_value!(a_reg, Value::Byte(r) => {
        leftmost_bit = **r >> 7;
        **r = (**r << 1) | c_bit;
    });
    console.registers.clear_or_set_flag(leftmost_bit == 0, C);
}

fn rra(console: &mut Console) {
    console.registers.clear_flags(&[Z, N, H]);
    let c_bit = if console.registers.is_flag_set(C) {1} else {0}; 
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    let rightmost_bit: u8;
    match_value!(a_reg, Value::Byte(r) => {
        rightmost_bit = **r << 7;
        **r = (**r >> 1) | (c_bit << 7);
    });
    console.registers.clear_or_set_flag(rightmost_bit == 0, C);
}

fn daa(console: &mut Console) {
    let mut adjustment: u8 = 0;
    let h_flag: bool = console.registers.is_flag_set(H);
    let c_flag: bool = console.registers.is_flag_set(C);
    let n_flag: bool = console.registers.is_flag_set(N);
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
    console.registers.set_flags(&[N, H]);
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    match_value!(a_reg, Value::Byte(r) => { **r = !(**r); })
}

fn scf(console: &mut Console) {
    console.registers.clear_flags(&[N, H]);
    console.registers.set_flag(C);
}

fn ccf(console: &mut Console) {
    console.registers.clear_flags(&[N, H]);
    console.registers.clear_or_set_flag(!console.registers.is_flag_set(C), C);
}

fn jr_imm8(console: &mut Console) {
    
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