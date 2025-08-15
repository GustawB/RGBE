use core::panic;
use std::env;
use std::fs::read;

use crate::types::Console;

mod bit_ops;
mod constants;
mod types;
mod common;
mod block_zero;
mod block_one;
mod block_two;
mod block_three;
mod block_cb;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename: &String = &args[1];
    
    let boot_rom: Vec<u8> = read(filename).expect("Failed to read the boot rom");
    let mut console: Console = match Console::init(boot_rom) {
        Ok(c) => c,
        Err(msg) => panic!("Fainel to create Console: {msg}")
    };

    console.execute();
}
