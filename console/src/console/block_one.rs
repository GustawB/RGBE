use constants::reg8;

use crate::console::Console;

fn ld_r8_r8(src: u8, dest: u8, console: &mut Console, curr_ip: u16) {
    console.call_hook(format!("LD {}, {}", reg8::reg_to_name(dest), reg8::reg_to_name(src)), curr_ip);

    console.set_r8(dest, console.get_r8(src));
}


fn halt(console: &mut Console, curr_ip: u16) {
    // TODO: implement
    console.call_hook(format!("HALT"), curr_ip);
}

pub fn dispatch(console: &mut Console, instr: u8, curr_ip: u16) -> () {
    let src: u8 = instr & 0x07;
    let dst: u8 = (instr << 2) >> 5;
    if instr == 118 {
        halt(console, curr_ip);
    } else {
        ld_r8_r8(src, dst, console, curr_ip);
    }
}