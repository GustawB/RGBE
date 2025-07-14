use crate::types::*;



fn ld_r16_imm16(r16: u8, console: &mut Console) {
    let imm16: u16 = console.fetch_two_bytes();
    let reg: &mut Value = &console.registers[RegSize::Word(r16)];
    match reg {
        Value::Word(r) => *r = imm16,
        _ => panic!("Invalid register size returned"),
    }
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