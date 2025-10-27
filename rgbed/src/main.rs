use core::panic;
use std::{env, fs, i64};
use std::fs::{read, File, OpenOptions};
use std::collections::HashMap;
use console::debug_addr;
use console::types::Hookable;
use constants::reg16::{self, SP};
use constants::{flag, reg8};
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
    pub const VERBOSE: char         = 'v';
}

struct Debugger {
    break_count: u32,
    started: bool,
    stepping: bool,
    verbose: bool,
    breakpoints: HashMap<u16, String>,
}

impl Debugger {
    pub fn init() -> Debugger {
        Debugger {
            break_count: 0,
            started: false,
            stepping: false,
            verbose: false,
            breakpoints: HashMap::new(),
        }
    }

    pub fn run(&mut self, console: &mut Console, addr: u16) {
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
                    if self.started {
                        self.stepping = true;
                        break;
                    } else {
                        println!("No ongoing debugging session. Enter \'{}\' to start debugging", actions::RUN);
                    }
                },
                actions::DUMP_REGS => Debugger::dump_regs(console),
                actions::VERBOSE => self.verbose = !self.verbose,
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

    fn dump_regs(console: &mut Console) {
        println!("REG8 DUMP:");
        for reg in reg8::LIST {
            if reg != reg8::HL_ADDR && reg != reg8::EA {
                println!("Register: {}; Value: 0x{:02X}", reg8::reg_to_name(reg), console.get_r8(reg));
            }
        }

        print!("Flags: ");
        for f in flag::LIST {
            print!("{}: {}; ", flag::flag_to_name(f), console.is_flag_set(f) as u8);
        }
        println!();
    }
}

impl Hookable for Debugger {
    fn hook(&mut self, console: &mut Console, log: String, addr: u16) {
        let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("logs.txt")
                .unwrap();
            
            
        if addr != std::u16::MAX {
            let msg = format!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n",
                            console.get_r8(reg8::A), console.get_flags(), console.get_r8(reg8::B), console.get_r8(reg8::C),
                            console.get_r8(reg8::D), console.get_r8(reg8::E), console.get_r8(reg8::H), console.get_r8(reg8::L),
                            console.get_r16(reg16::SP), addr, console.get_mem(addr.into()), console.get_mem((addr + 1).into()),
                            console.get_mem((addr + 2).into()), console.get_mem((addr + 3).into()));
            file.write_all(msg.as_bytes()).unwrap();
            debug_addr(addr, log);
            if self.verbose {
                Debugger::dump_regs(console);
            }
        }

        self.run(console, addr);
    }
}

// Dissembly ROM:
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