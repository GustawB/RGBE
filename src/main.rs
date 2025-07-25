use std::env;
use std::io::Read;
use std::fs::File;
use std::io::BufReader;

mod bit_ops;
mod constants;
mod types;
mod block_zero;
mod block_one;
mod block_two;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() ==  3);
    let filename: &String = &args[2];
    
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);

    let byte_stream = buf_reader.bytes();

    for b in byte_stream {
        println!("{}", b.unwrap());
    }
}
