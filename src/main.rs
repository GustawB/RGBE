use std::env;
use std::fs::File;

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
    
    let file = File::open(filename).unwrap();
    let mut console: Console = Console::init(file);

    console.execute();
}
