use crate::console::{helpers::{common::debug_addr, constants::reg8}, types::types::Byte, Console};

fn ld_r8_r8(src: u8, dest: u8, console: &mut Console, curr_ip: u16) {
    let src_val: u8 = console[Byte { idx: src }];
    let dest_val: &mut u8 = &mut console[Byte { idx: dest }];
    *dest_val = src_val;

    debug_addr(curr_ip, format!("LD {}, {}", reg8::reg_to_name(dest), reg8::reg_to_name(src)));
}


fn halt(_console: &mut Console, curr_ip: u16) {
    // TODO: implement

    debug_addr(curr_ip, format!("HALT"));
}

pub fn dispatch(console: &mut Console, instr: u8, curr_ip: u16) -> () {
    if instr == 118 {
        halt(console, curr_ip);
    } else {
        ld_r8_r8(instr << 5, (instr << 2) >> 4, console, curr_ip);
    }
}