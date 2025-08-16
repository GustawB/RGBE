use crate::{console::types::types::Byte, Console};

fn ld_r8_r8(src: u8, dest: u8, console: &mut Console) {
    let src_val: u8 = console[Byte { idx: src }];
    let dest_val: &mut u8 = &mut console[Byte { idx: dest }];
    *dest_val = src_val;
}


fn halt(_console: &mut Console) {
    // TODO: implement
}

pub fn dispatch(instr: u8, console: &mut Console) -> () {
    if instr == 118 {
        halt(console);
    } else {
        ld_r8_r8(instr << 5, (instr << 2) >> 4, console);
    }
}