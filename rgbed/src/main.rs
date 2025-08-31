use core::panic;
use std::{env, i64};
use std::fs::read;
use std::collections::HashMap;
use console::types::{Hookable, Stepable};
use console::{reg8, types::{Byte}};
use env_logger::Env;
use text_io::read;
use std::io::Write;

use console::Console;

mod actions {
    pub const RUN: char             = 'r';
    pub const SET_BREAK: char       = 'b';
    pub const REMOVE_BREAK: char    = 'x';
    pub const STEP: char            = 's';
    pub const DUMP_REGS: char       = 'd';
    pub const EXIT: char            = 'e';
}

struct Debugger {
    break_count: u32,
    started: bool,
    breakpoints: HashMap<u16, String>,
}

impl Debugger {
    pub fn init() -> Debugger {
        Debugger {
            break_count: 0,
            started: false,
            breakpoints: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let cmd: char = read!();
            match cmd {
                actions::RUN => self.run_debugger(),
                actions::SET_BREAK => self.set_break(),
                actions::REMOVE_BREAK => self.remove_break(),
                actions::STEP => self.step(),
                actions::DUMP_REGS => self.dump_regs(),
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
                    println!("{b} at address 0x{:04X} reached", ip);
                    break;
                }
                None => self.console.step(),
            }
        }
    }

    fn set_break(&mut self) {
        let addr_str: String = read!();
        let addr: u16 = i64::from_str_radix(addr_str.trim_start_matches("0x"), 16).unwrap().try_into().unwrap();
        match self.breakpoints.get(&addr) {
            Some(b) => {
                println!("Breakpoint {b} at address 0x{:04X} already set", addr)
            }
            None => {
                self.break_count += 1;
                let name: String = format!("break_{}", self.break_count);
                println!("{name} set at address 0x{:04X}", addr);
                self.breakpoints.insert(addr, name);
            }
        };
    }

    // Holding two separate hashmaps may be faster,
    // but for now, simplicity > speed (I have at most 2-3 breakpoints anyway)
    fn remove_break(&mut self) {
        let name: String = read!();
        // It is guaranteed that names are unique
        let mut addr: u16 = std::u16::MAX;
        for (key, val) in self.breakpoints.iter() {
            if *val == name {
                addr = *key;
            }
        };

        if addr == std::u16::MAX {
            println!("Breakpoint {name} does not exist");
        } else {
           self.breakpoints.remove(&addr).unwrap();
           println!("Removed breakpoint {name} at address 0x{:04X}", addr);
        }
    }

    fn step(&mut self) {
        if self.started {
            self.console.step();
        } else {
            println!("No ongoing debugging session. Enter \'{}\' to start debugging", actions::RUN);
        }
    }

    fn dump_regs(&mut self) {
        println!("REG8 DUMP:");
        for reg in reg8::LIST {
            if reg != reg8::HL_ADDR && reg != reg8::EA {
                println!("Register: {}; Value: 0x{:02X}", reg8::reg_to_name(reg), self.console[Byte { idx: reg }]);
            }
        }
    }
}

impl Hookable for Debugger {
    fn hook(&mut self, console: &Console) {
        
    }
}

// Deassembly ROM:
// https://www.neviksti.com/DMG/DMG_ROM.asm
fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename: &String = &args[1];
    
    let boot_rom: Vec<u8> = read(filename).expect("Failed to read the boot rom");
    let mut console: Console = match Console::init(boot_rom) {
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

    let debugger = Debugger::init();
    console.set_hookable(&debugger);

    console.execute();
}