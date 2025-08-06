use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::{block_cb, block_one, block_three, block_two, block_zero, constants::*};
use std::ops::{Index, IndexMut};
use std::marker::{self, PhantomData};

pub struct Console<'c> {
    pub addr_bus: [u8; ADDR_BUS_SIZE],
    executable: File,
    pub registers: Registers<'c>,
    pub pending_ei: bool,
}

impl<'c> Console<'c> {
    pub fn init(executable: File) -> Console<'c> {
        Console { 
            addr_bus: [0; ADDR_BUS_SIZE],
            executable: executable,
            registers: Registers::init(),
            pending_ei: false,
        }
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let mut buf: [u8; 1] = [0];
        self.executable.read_exact(&mut buf).unwrap();
        buf[0]
    }

    pub fn fetch_two_bytes(&mut self) -> u16 {
        let mut buf: [u8; 2] = [0; 2];
        self.executable.read_exact(&mut buf).unwrap();
        ((buf[1] as u16) << 8) | buf[0] as u16 // Little endian garbage
    }

    pub fn move_pc(&mut self, amount: u16) {
        match self.executable.seek_relative(amount.into()) {
            Ok(()) => (),
            Err(..) => panic!("Failed to move program counter."),
        }
    }

    pub fn set_pc(&mut self, val: u16) {
        match self.executable.seek(SeekFrom::Start(val.into())) {
            Ok(res_pos) => if res_pos != val.into() {panic!("Program counter set to invalid value.")},
            Err(..) => panic!("Failed to set program counter."),
        }
    }

    pub fn get_pc(&mut self) -> u16 {
        self.executable.stream_position().unwrap() as u16
    }

    pub fn stk_push(&mut self, val: u8) {
        let sp_val: &mut u16 = &mut self.registers[Word { idx: SP }];
        *sp_val -= 1;
        self.addr_bus[*sp_val as usize] = val;
    }

    pub fn stk_pop(&mut self) -> u8 {
        let sp_val: &mut u16 = &mut self.registers[Word { idx: SP }];
        *sp_val += 1;
        self.addr_bus[*sp_val as usize]
    }

    fn step(&mut self) {
        let bt = self.fetch_byte();
        if bt == 0xCD {
            block_cb::dispatch(bt, self);
        }
        else {
            match bt >> 6 {
                0 => block_zero::dispatch(bt, self),
                1 => block_one::dispatch(bt, self),
                2 => block_two::dispatch(bt, self),
                3=>  block_three::dispatch(bt, self),
                _ => panic!("Invalid instruction"),
            };
        }
    }

    pub fn execute(&mut self) {
        loop {
            self.step();
            if self.pending_ei {
                self.addr_bus[IME as usize] = 1;
                self.pending_ei = false;
            }
        }
    }
}

union Register {
    value: u16,
    halves: [u8; 2]
}

pub struct Registers<'r> {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: Register,
    pc: Register,
    _marker: marker::PhantomData<&'r u8>,
}

impl<'r> Registers<'r> {
    pub fn init() -> Registers<'r> {
        Registers {
            af: Register { value: 0 },
            bc: Register { value: 0 },
            de: Register { value: 0 },
            hl: Register { value: 0 },
            sp: Register { halves: [0xFF, 0xFE] },
            pc: Register { value: 0 },
            _marker: PhantomData
        }
    }

    pub fn get_r8(&self, idx: u8) -> &u8 {
        unsafe {
            match idx {
                0 => &self.bc.halves[1],
                1 => &self.bc.halves[0],
                2 => &self.de.halves[1],
                3 => &self.de.halves[0],
                4 => &self.hl.halves[1],
                5 => &self.hl.halves[0],
                6 => panic!("Unimplemented"),
                7 => &self.af.halves[1],
                8 => &self.af.halves[0],
                _ => panic!("Index out of range"),
            }
        }
    }

    pub fn get_r8_mut(&mut self, idx: u8) -> &mut u8 {
        unsafe {
            match idx {
                0 => &mut self.bc.halves[1],
                1 => &mut self.bc.halves[0],
                2 => &mut self.de.halves[1],
                3 => &mut self.de.halves[0],
                4 => &mut self.hl.halves[1],
                5 => &mut self.hl.halves[0],
                6 => panic!("Unimplemented"),
                7 => &mut self.af.halves[1],
                8 => &mut self.af.halves[0],
                _ => panic!("Index out of range"),
            }
        }
    }

    pub fn get_r16(&self, idx: u8) -> &u16 {
        unsafe {
            match idx {
                0 => &self.bc.value,
                1 => &self.de.value,
                2 => &self.hl.value,
                3 => &self.sp.value,
                _ => panic!("Index out of range"),
            }
        }
    }

    pub fn get_r16_mut(&mut self, idx: u8) -> &mut u16 {
        unsafe {
            match idx {
                0 => &mut self.bc.value,
                1 => &mut self.de.value,
                2 => &mut self.hl.value,
                3 => &mut self.sp.value,
                _ => panic!("Index out of range"),
            }
        }
    }

    fn get_r16stk(&self, idx: u8) -> &u16 {
        unsafe {
            match idx {
                0 => &self.bc.value,
                1 => &self.de.value,
                2 => &self.hl.value,
                3 => &self.af.value,
                _ => panic!("Index out of range")
            }
        }
    }

    fn get_r16stk_mut(&mut self, idx: u8) -> &mut u16 {
        unsafe {
            match idx {
                0 => &mut self.bc.value,
                1 => &mut self.de.value,
                2 => &mut self.hl.value,
                3 => &mut self.af.value,
                _ => panic!("Index out of range")
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

pub struct Byte {
    pub idx: u8,
}
pub struct Word {
    pub idx: u8,
}
pub struct WordSTK {
    pub idx: u8,
}

impl<'r> Index<Byte> for Registers<'r> {
    type Output = u8;

    fn index(&self, index: Byte) -> &Self::Output {
        self.get_r8(index.idx)
    }
}

impl<'a> IndexMut<Byte> for Registers<'a> {
    fn index_mut(&mut self, index: Byte) -> &mut Self::Output {
        self.get_r8_mut(index.idx)
    }
}

impl<'r> Index<Word> for Registers<'r> {
    type Output = u16;

    fn index(&self, index: Word) -> &Self::Output {
        self.get_r16(index.idx)
    }
}

impl<'a> IndexMut<Word> for Registers<'a> {
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        self.get_r16_mut(index.idx)
    }
}

impl<'r> Index<WordSTK> for Registers<'r> {
    type Output = u16;

    fn index(&self, index: WordSTK) -> &Self::Output {
        self.get_r16stk(index.idx)
    }
}

impl<'a> IndexMut<WordSTK> for Registers<'a> {
    fn index_mut(&mut self, index: WordSTK) -> &mut Self::Output {
        self.get_r16stk_mut(index.idx)
    }
}