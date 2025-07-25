use macros::match_value;

use crate::{constants::*, types::*};

fn add_a_imm8(r8: u8, console: &mut Console) {
    let src_val: u8;
    let src_reg: Value = console.registers[RegSize::Byte(r8)];
    match_value!(src_reg, Value::Byte(r) => { src_val= **r; })
    let a_reg: &mut Value = &mut console.registers[RegSize::Byte(A)];
    
}