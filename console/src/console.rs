mod helpers;
pub mod types;

mod block_cb;
mod block_zero;
mod block_one;
mod block_two;
mod block_three;

use std::ops::{Index, IndexMut};

pub use crate::console::helpers::constants::reg8;
use crate::console::{helpers::constants::{cond, flag, reg16, reg16mem, reg16stk, ADDR_BUS_SIZE, IME}, types::{Byte, Register, Word, WordSTK}};

pub struct Console {
    pub addr_bus: [u8; ADDR_BUS_SIZE],
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: Register,
    ip: Register,
    pub pending_ei: bool,
}

impl Console {
    pub fn init(boot_rom: Vec<u8>) -> Result<Console, String> {
        if boot_rom.len() > 0x100 {
            Err(String::from("Boot rom too long"))
        } else {
            let mut tmp_addr_bus = [0; ADDR_BUS_SIZE];
            for i in 0..boot_rom.len() {
                tmp_addr_bus[i] = boot_rom[i];
            }
            Ok(Console {
                addr_bus: tmp_addr_bus,
                af: Register { value: 0 },
                bc: Register { value: 0 },
                de: Register { value: 0 },
                hl: Register { value: 0 },
                sp: Register { halves: [0xFF, 0xFE] },
                ip: Register { value: 0 },
                pending_ei: false,
            })
        }
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let res: u8 = unsafe { self.addr_bus[self.ip.value as usize] };
        unsafe { self.ip.value += 1 };
        res
    }

    pub fn fetch_two_bytes(&mut self) -> u16 {
        let a: u8 = unsafe { self.addr_bus[self.ip.value as usize] };
        let b: u8 = unsafe { self.addr_bus[self.ip.value as usize + 1] };
        unsafe { self.ip.value += 2 };
        ((b as u16) << 8) | a as u16 // Little endian garbage
    }

    pub fn move_ip(&mut self, amount: u8) {
        if (amount as i8) < 0 {
            unsafe { self.ip.value -= (amount as i8).abs() as u16 };
        } else {
            unsafe { self.ip.value += amount as u16 };
        }
    }

    pub fn set_ip(&mut self, val: u16) {
        self.ip.value = val;
    }

    pub fn get_ip(&self) -> u16 {
        unsafe { self.ip.value }
    }

    pub fn stk_push(&mut self, val: u8) {
        let sp_val: &mut u16 = &mut self[Word { idx: reg16::SP }];
        *sp_val -= 1;
        self.addr_bus[*sp_val as usize] = val;
    }

    pub fn stk_pop(&mut self) -> u8 {
        let sp_val: &mut u16 = &mut self[Word { idx: reg16::SP }];
        *sp_val += 1;
        self.addr_bus[*sp_val as usize]
    }

    // Process one instruction. Exposed outside mostly for the debugger
    pub fn step(&mut self) {
        let curr_ip: u16 = self.get_ip();
        let bt = self.fetch_byte();
        if bt == 0xCB {
            let bt_instr: u8 = self.fetch_byte();
            block_cb::dispatch(self, bt_instr, curr_ip);
        }
        else {
            match bt >> 6 {
                0 => block_zero::dispatch(self, bt, curr_ip),
                1 => block_one::dispatch(self, bt, curr_ip),
                2 => block_two::dispatch(self, bt, curr_ip),
                3 => block_three::dispatch(self, bt, curr_ip),
                _ => panic!("Invalid instruction"),
            };
        }

        // TODO: inspect if this is not too soon
        if self.pending_ei {
            self.addr_bus[IME as usize] = 1;
            self.pending_ei = false;
        }
    }

    // Entry point of the console.
    pub fn execute(&mut self) {
        loop { self.step(); }
    }

    fn get_r8(&self, idx: u8) -> &u8 {
        unsafe {
            match idx {
                reg8::B => &self.bc.halves[1],
                reg8::C => &self.bc.halves[0],
                reg8::D => &self.de.halves[1],
                reg8::E => &self.de.halves[0],
                reg8::H => &self.hl.halves[1],
                reg8::L => &self.hl.halves[0],
                reg8::HL_ADDR => &self.addr_bus[self.hl.value as usize],
                reg8::A => &self.af.halves[1],
                _ => panic!("Index out of range"),
            }
        }
    }

    fn get_r8_mut(&mut self, idx: u8) -> &mut u8 {
        unsafe {
            match idx {
                reg8::B => &mut self.bc.halves[1],
                reg8::C => &mut self.bc.halves[0],
                reg8::D => &mut self.de.halves[1],
                reg8::E => &mut self.de.halves[0],
                reg8::H => &mut self.hl.halves[1],
                reg8::L => &mut self.hl.halves[0],
                reg8::HL_ADDR => &mut self.addr_bus[self.hl.value as usize],
                reg8::A => &mut self.af.halves[1],
                _ => panic!("Index out of range"),
            }
        }
    }

    fn get_r16(&self, idx: u8) -> &u16 {
        unsafe {
            match idx {
                reg16::BC => &self.bc.value,
                reg16::DE => &self.de.value,
                reg16::HL => &self.hl.value,
                reg16::SP => &self.sp.value,
                _ => panic!("Index out of range"),
            }
        }
    }

    fn get_r16_mut(&mut self, idx: u8) -> &mut u16 {
        unsafe {
            match idx {
                reg16::BC => &mut self.bc.value,
                reg16::DE => &mut self.de.value,
                reg16::HL => &mut self.hl.value,
                reg16::SP => &mut self.sp.value,
                _ => panic!("Index out of range"),
            }
        }
    }

    fn get_r16stk(&self, idx: u8) -> &u16 {
        unsafe {
            match idx {
                reg16stk::BC => &self.bc.value,
                reg16stk::DE => &self.de.value,
                reg16stk::HL => &self.hl.value,
                reg16stk::AF => &self.af.value,
                _ => panic!("Index out of range")
            }
        }
    }

    fn get_r16stk_mut(&mut self, idx: u8) -> &mut u16 {
        unsafe {
            match idx {
                reg16stk::BC => &mut self.bc.value,
                reg16stk::DE => &mut self.de.value,
                reg16stk::HL => &mut self.hl.value,
                reg16stk::AF => &mut self.af.value,
                _ => panic!("Index out of range")
            }
        }
    }

    pub fn get_r16mem(&mut self, idx: u8) -> u16 {
        unsafe {
            match idx {
                reg16mem::BC => self.bc.value,
                reg16mem::DE => self.de.value,
                reg16mem::HLI => {
                    self.hl.value += 1;
                    self.hl.value - 1
                },
                reg16mem::HLD => {
                    self.hl.value -= 1;
                    self.af.value + 1
                },
                _ => panic!("Index out of range"),
            }
        }
    }

    pub fn is_flag_set(&self, flag: u8) -> bool {
        unsafe { (self.af.halves[1] & (flag as u8))  != 0 }
    }

    pub fn set_flag(&mut self, flag: u8) {
        unsafe { self.af.halves[1] = self.af.halves[1] | (flag as u8); }
    }

    pub fn clear_flag(&mut self, flag: u8) {
        unsafe { self.af.halves[1] = self.af.halves[1] & (!(flag as u8)); }
    }

    pub fn clear_flags(&mut self, flags: &[u8]) {
        for flag in flags.iter() {
            self.clear_flag(*flag);
        }
    }

    pub fn set_flags(&mut self, flags: &[u8]) {
        for flag in flags.iter() {
            self.set_flag(*flag);
        }
    }

    pub fn clear_or_set_flag(&mut self, should_set: bool, flag: u8) {
        if should_set {
            self.set_flag(flag);
        }
        else {
            self.clear_flag(flag);
        }
    }

    pub fn is_condition_met(&self, cc: u8) -> bool {
        match cc {
            cond::NZ => !self.is_flag_set(flag::Z),
            cond::Z => self.is_flag_set(flag::Z),
            cond::NC => !self.is_flag_set(flag::C),
            cond::C => self.is_flag_set(flag::C),
            _ => panic!("Invalid flag encountered"), 
        }
    }
}

impl Index<Byte> for Console {
    type Output = u8;

    fn index(&self, index: Byte) -> &Self::Output {
        self.get_r8(index.idx)
    }
}

impl IndexMut<Byte> for Console {
    fn index_mut(&mut self, index: Byte) -> &mut Self::Output {
        self.get_r8_mut(index.idx)
    }
}

impl Index<Word> for Console {
    type Output = u16;

    fn index(&self, index: Word) -> &Self::Output {
        self.get_r16(index.idx)
    }
}

impl IndexMut<Word> for Console {
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        self.get_r16_mut(index.idx)
    }
}

impl Index<WordSTK> for Console {
    type Output = u16;

    fn index(&self, index: WordSTK) -> &Self::Output {
        self.get_r16stk(index.idx)
    }
}

impl IndexMut<WordSTK> for Console {
    fn index_mut(&mut self, index: WordSTK) -> &mut Self::Output {
        self.get_r16stk_mut(index.idx)
    }
}