use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};
use crate::{block_zero, constants::*};
use std::ops::{Index, IndexMut};
use std::marker::{self, PhantomData};

pub struct Console<'c> {
    pub addrBus: [u8; ADDR_BUS_SIZE],
    executable: File,
    pub registers: Registers<'c>,
}

impl<'c> Console<'c> {
    pub fn init(executable: File) -> Console<'c> {
        Console { 
            addrBus: [0; ADDR_BUS_SIZE],
            executable: executable,
            registers: Registers::init(),
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

    pub fn move_pc(&mut self, amount: i16) {
        match self.executable.seek_relative(amount as i64) {
            Ok(()) => (),
            Err(..) => panic!("Falied to move program counter."),
        }
    }

    fn step(&mut self) -> Result<()> {
        let bt = self.fetch_byte();
        if bt == 0xCD {
            Ok(())
        }
        else {
            Ok (match bt >> 6 {
                0 => block_zero::dispatch(bt, self),
                1=> unimplemented!(),
                2=> unimplemented!(),
                3=> unimplemented!(),
                _ => panic!("Invalid instruction"),
            })
        }
    }

    pub fn execute(&mut self) -> Result<()> {
        loop {
            
        }
    }
}

pub struct Registers<'r> {
    AF: [i8; 2],
    BC: [i8; 2],
    DE: [i8; 2],
    HL: [i8; 2],
    SP: [i8; 2],
    PC: [i8; 2],
    _marker: marker::PhantomData<&'r u8>,
}

pub enum Value<'v> {
    Byte(&'v mut u8),
    Word(&'v mut u16),
}

pub enum RegSize {
    Byte(u8),
    Word(u8),
}

impl<'r> Registers<'r> {
    pub fn init() -> Registers<'r> {
        Registers {
            AF: [0, 0],
            BC: [0, 0],
            DE: [0, 0],
            HL: [0, 0],
            SP: [0, 0],
            PC: [0, 0],
            _marker: PhantomData
        }
    }

    pub fn get_r8(&mut self, idx: u8) -> &mut i8 {
        let res: &mut i8;
        unsafe {
            let ptr = match idx {
                0 => self.BC.as_mut_ptr(),
                1 => self.BC.as_mut_ptr().add(1),
                2 => self.DE.as_mut_ptr(),
                3 => self.DE.as_mut_ptr().add(1),
                4 => self.HL.as_mut_ptr(),
                5 => self.HL.as_mut_ptr().add(1),
                6 => panic!("Unimplemented"),
                7 => self.AF.as_mut_ptr(),
                8 => self.AF.as_mut_ptr().add(1),
                _ => panic!("Index out of range")
            };
            res = &mut *ptr;
        };
        res
    }

    pub fn get_r16(&mut self, idx: u8) -> &mut i16 {
        let res: &mut i16;
        unsafe {
            let ptr: *mut i16 = match idx {
                0 => self.BC.as_mut_ptr() as *mut i16,
                1 => self.DE.as_mut_ptr() as *mut i16,
                2 => self.HL.as_mut_ptr() as *mut i16,
                3 => self.SP.as_mut_ptr() as *mut i16,
                _ => panic!("Index out of range")
            };
            res = &mut *ptr;
        }
        res
    }

    fn get_r16stk(&mut self, idx: u8) -> &mut i16 {
        let res: &mut i16;
        unsafe {
            let ptr: *mut i16 = match idx {
                0 => self.BC.as_mut_ptr() as *mut i16,
                1 => self.DE.as_mut_ptr() as *mut i16,
                2 => self.HL.as_mut_ptr() as *mut i16,
                3 => self.AF.as_mut_ptr() as *mut i16,
                _ => panic!("Index out of range")
            };
            res = &mut *ptr;
        }
        res
    }

    pub fn is_flag_set(&self, flag: u8) -> bool {
        (self.AF[1] & (flag as i8)) != 0
    }

    pub fn set_flag(&mut self, flag: u8) {
        self.AF[1] = self.AF[1] | (flag as i8);
    }

    pub fn clear_flag(&mut self, flag: u8) {
        self.AF[1] = self.AF[1] & (!(flag as i8));
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

impl<'a> Index<RegSize> for Registers<'a> {
    type Output = Value<'a>;

    fn index(&self, index: RegSize) -> &Self::Output {
        match index {
            RegSize::Byte(i) => unimplemented!(),
            RegSize::Word(i) => unimplemented!(),
        }
    }
}

impl<'a> IndexMut<RegSize> for Registers<'a> {
    fn index_mut(&mut self, index: RegSize) -> &mut Self::Output {
        match index {
            RegSize::Byte(i) => unimplemented!(),
            RegSize::Word(i) => unimplemented!(),
        }
    }
}