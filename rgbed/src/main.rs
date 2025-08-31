use core::panic;
use std::{env, i64};
use std::fs::read;
use std::collections::HashMap;
use console::debug_addr;
use console::types::Hookable;
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
}

struct Debugger {
    break_count: u32,
    started: bool,
    stepping: bool,
    breakpoints: HashMap<u16, String>,
}

impl Debugger {
    pub fn init() -> Debugger {
        Debugger {
            break_count: 0,
            started: false,
            stepping: false,
            breakpoints: HashMap::new(),
        }
    }

    pub fn run(&mut self, console: &Console, addr: u16) {
        if self.breakpoints.contains_key(&addr) {
            println!("Breakpoint {} at address 0x{:04X} reached",
                    self.breakpoints.get(&addr).unwrap(), addr);
        } else if !self.stepping && self.started {
            return;
        }
        self.stepping = false;

        loop {
            let cmd: char = read!();
            match cmd {
                actions::RUN => {
                    self.started = true;
                    return;
                },
                actions::SET_BREAK => self.set_break(),
                actions::REMOVE_BREAK => self.remove_break(),
                actions::STEP => {
                    self.step();
                    return;
                }
                actions::DUMP_REGS => Debugger::dump_regs(console),
                _ => println!("Unknown debug command"),
            };
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
            self.stepping = true;
        } else {
            println!("No ongoing debugging session. Enter \'{}\' to start debugging", actions::RUN);
        }
    }

    fn dump_regs(console: &Console) {
        println!("REG8 DUMP:");
        for reg in reg8::LIST {
            if reg != reg8::HL_ADDR && reg != reg8::EA {
                println!("Register: {}; Value: 0x{:02X}", reg8::reg_to_name(reg), console[Byte { idx: reg }]);
            }
        }
    }
}

impl Hookable for Debugger {
    fn hook(&mut self, console: &Console, log: String, addr: u16) {
        if addr != std::u16::MAX {
            debug_addr(addr, log);
        }

        self.run(console, addr);
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

    let mut debugger = Debugger::init();
    console.set_hookable(&mut debugger);

    console.execute();
}