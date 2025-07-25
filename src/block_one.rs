use macros::match_value;

use crate::types::*;

fn ld_r8_r8(src: u8, dest: u8, console: &mut Console) {
    let src_val: u8;
    {
        let src_reg: &Value = &console.registers[RegSize::Byte(src)];
        match_value!(src_reg, Value::Byte(r) => { src_val = **r; })
    }
    let dest_reg: &mut Value = &mut console.registers[RegSize::Byte(dest)];
    match_value!(dest_reg, Value::Byte(r) => { **r = src_val; })
}

fn halt(console: &mut Console) {
    // TODO: implement
}

pub fn dispatch(instr: u8, console: &mut Console) -> () {
    if instr == 118 {
        halt(console);
    } else {
        ld_r8_r8(instr << 5, (instr << 2) >> 4, console);
    }
}