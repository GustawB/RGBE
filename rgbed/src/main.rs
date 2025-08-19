use core::panic;
use std::env;
use std::fs::read;
use std::collections::HashMap;
use env_logger::Env;
use text_io::read;
use std::io::Write;

use console::Console;

mod actions {
    pub const RUN: char = 'r';
    pub const SET_BREAK: char = 'b';
    pub const STEP: char = 's';
    pub const EXIT: char = 'e';
}

struct Debugger {
    break_count: u32,
    started: bool,
    breakpoints: HashMap<u16, String>,
    console: Console,
}

impl Debugger {
    pub fn init(console: Console) -> Debugger {
        Debugger {
            break_count: 0,
            started: false,
            breakpoints: HashMap::new(),
            console: console,
        }
    }

    pub fn run(&mut self) {
        loop {
            let cmd: char = read!();
            match cmd {
                actions::RUN => self.run_debugger(),
                actions::SET_BREAK => self.set_break(),
                actions::STEP => self.step(),
                actions::EXIT => break,
                _ => println!("Unknown debug command"),
            };
        }
    }

    fn run_debugger(&mut self) {
        self.started = true;
        loop {
            let ip: u16 = self.console.get_ip(); 
            match self.breakpoints.get(&ip) {
                Some(b) => {
                    println!("{b} at address {ip} reached");
                    break;
                }
                None => self.console.step(),
            }
        }
    }

    fn set_break(&mut self) {
        let addr: u16 = read!();
        match self.breakpoints.get(&addr) {
            Some(b) => {
                println!("Breakpoint {b} at address {addr} already set")
            }
            None => {
                self.break_count += 1;
                let name: String = format!("break{}", self.break_count);
                println!("{name} set at address {addr}");
                self.breakpoints.insert(addr, name);
            }
        };
    }

    fn step(&mut self) {
        if self.started {
            self.console.step();
        } else {
            println!("No ongoing debugging session. Enter \'{}\' to start debugging", actions::RUN);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename: &String = &args[1];
    
    let boot_rom: Vec<u8> = read(filename).expect("Failed to read the boot rom");
    let console: Console = match Console::init(boot_rom) {
        Ok(c) => c,
        Err(msg) => panic!("Failed to create Console: {msg}")
    };


    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "debug");
    env_logger::Builder::from_env(env).format(|buf, record| {
            writeln!(
                buf,
                "[{}] {}",
                record.level(),
                record.args()
            )
        }).init();

    let mut debugger: Debugger = Debugger::init(console);
    debugger.run();
}