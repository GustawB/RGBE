mod helpers;
pub mod types;

mod block_cb;
mod block_zero;
mod block_one;
mod block_two;
mod block_three;

use std::{marker::PhantomData, thread};

pub use crate::console::helpers::constants::{reg8, flag};
pub use crate::console::helpers::common::debug_addr;
use crate::{console::{helpers::{constants::{cond, intr, reg16, reg16mem, reg16stk, ADDR_BUS_SIZE, IME}}, types::Register}};
#[cfg(feature = "debugger")]
use crate::types::Hookable;

use std::sync::Arc;

use ppu::Ppu;
use addr_bus::AddrBus;

pub struct Console<'a> {
    pub addr_bus: Arc<AddrBus>,
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: Register,
    ip: Register,
    pub pending_ei: bool,

    phantom: PhantomData<&'a u8>,

    #[cfg(feature = "debugger")]
    hookable: Option<&'a mut dyn Hookable>,
}

const HEADER_SIZE: usize = 52;
// For now hardcoded cartridge header
static HEADER: [u8; HEADER_SIZE] = [
    0x0,            // NOP
    0xC3, 0x50, 0x01,    // jp 0x0150
    // NINTENDO LOGO:
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
    //TO BE FILLED
];

macro_rules! addr_bus {
    ($self:ident, $addr:expr) => {
        $self.addr_bus.lock().unwrap()[$addr]
    }
}

impl<'a> Console<'a> {
    pub fn init(boot_rom: Vec<u8>) -> Result<Console<'a>, String> {
        if boot_rom.len() > 0x100 {
            Err(String::from("Boot rom too long"))
        } else {
            let mut tmp_addr_bus = [0; ADDR_BUS_SIZE];
            for i in 0..boot_rom.len() {
                tmp_addr_bus[i] = boot_rom[i];
            }

            for i in 0..HEADER_SIZE {
                tmp_addr_bus[0x100 + i] = HEADER[i];
            }

            let addr_bus: AddrBus = AddrBus::new(tmp_addr_bus);

            Ok(Console {
                addr_bus: Arc::new(addr_bus),
                af: Register { value: 0 },
                bc: Register { value: 0 },
                de: Register { value: 0 },
                hl: Register { value: 0 },
                sp: Register { halves: [0xFF, 0xFE] },
                ip: Register { value: 0 },
                pending_ei: false,
                phantom: PhantomData,
                #[cfg(feature = "debugger")]
                hookable: None,
            })
        }
    }

    #[cfg(feature = "debugger")]
    pub fn set_hookable<T: Hookable>(&mut self, h: &'a mut T) {
        self.hookable = Some(h);
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let res: u8 = unsafe { addr_bus!(self, self.ip.value as usize) };
        unsafe { self.ip.value += 1 };
        res
    }

    pub fn fetch_two_bytes(&mut self) -> u16 {
        let a: u8 = unsafe { addr_bus!(self, self.ip.value as usize) };
        let b: u8 = unsafe { addr_bus!(self, self.ip.value as usize + 1) };
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

    fn stk_push8(&mut self, val: u8) {
        let sp: u16 = self.get_r16(reg16::SP);
        self.set_r16(reg16::SP, sp - 1 );
        addr_bus!(self, sp as usize - 1) = val;
    }

    fn stk_pop8(&mut self) -> u8 {
        let sp: u16 = self.get_r16(reg16::SP);
        let res: u8 = addr_bus!(self, sp as usize);
        self.set_r16(reg16::SP, sp + 1);
        res
    }

    pub fn stk_push16(&mut self, addr: u16) {
        self.stk_push8((addr >> 8) as u8);
        self.stk_push8((addr & 0x00FF) as u8);
    }

    pub fn stk_pop16(&mut self) -> u16 {
        let low: u16 = self.stk_pop8() as u16;
        let high: u16 = self.stk_pop8() as u16;
        low | (high << 8)
    }

    fn step(&mut self) {
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
            addr_bus!(self, IME as usize) = 1;
            self.pending_ei = false;
        }
    }

    #[cfg(feature = "debugger")]
    pub fn call_hook(&mut  self, log: String, curr_ip: u16) {
        if let Some(h) = self.hookable.take() {
            h.hook(self, log, curr_ip);
            self.hookable = Some(h);
        }
    }

    #[inline(always)]
    #[cfg(not(feature = "debugger"))]
    pub fn call_hook(&mut  self, _log: String, _curr_ip: u16) {}

    fn handle_interrupt(&mut self, mask: u8) {
        self.stk_push16(self.get_ip());
        self.set_ip(intr::get_jump_vector(mask));
        self.call_hook(intr::intr_to_name(mask), self.get_ip());
    }

    // Entry point of the console.
    pub fn execute(&mut self) {
        self.call_hook("".to_owned(), std::u16::MAX);

        let addr_bus: AddrBus = AddrBus::new([0; ADDR_BUS_SIZE]);
        let rc = Arc::new(addr_bus);
        let rc_clone = rc.clone();
        let handle = thread::spawn(move || {
            let mut ppu : Ppu = Ppu::new(rc_clone);
            ppu.execute();
        });
        
        loop {
            let (ime, ie, iflag) = self.addr_bus.get_intr_state();
            if ime == 1 {
                for i in 0..5 {
                    let mask: u8 = 1 << i;
                    if ie & mask == 1 && iflag & mask == 1{
                        self.handle_interrupt(mask);
                        break;
                    }
                }
            }

            self.step();
        }
        handle.join().unwrap();
    }

    fn get_r8(&self, idx: u8) -> u8 {
        unsafe {
            match idx {
                reg8::B => self.bc.halves[1],
                reg8::C => self.bc.halves[0],
                reg8::D => self.de.halves[1],
                reg8::E => self.de.halves[0],
                reg8::H => self.hl.halves[1],
                reg8::L => self.hl.halves[0],
                reg8::HL_ADDR => addr_bus!(self, self.hl.value as usize),
                reg8::A => self.af.halves[1],
                _ => panic!("Index out of range"),
            }
        }
    }

    fn set_r8(&mut self, idx: u8, val: u8) {
        unsafe {
            match idx {
                reg8::B => self.bc.halves[1] = val,
                reg8::C => self.bc.halves[0] = val,
                reg8::D => self.de.halves[1] = val,
                reg8::E => self.de.halves[0] = val,
                reg8::H => self.hl.halves[1] = val,
                reg8::L => self.hl.halves[0] = val,
                reg8::HL_ADDR => addr_bus!(self, self.hl.value as usize)  = val,
                reg8::A => self.af.halves[1] = val,
                _ => panic!("Index out of range"),
            };
        }
    }

    fn get_r16(&self, idx: u8) -> u16 {
        unsafe {
            match idx {
                reg16::BC => self.bc.value,
                reg16::DE => self.de.value,
                reg16::HL => self.hl.value,
                reg16::SP => self.sp.value,
                _ => panic!("Index out of range"),
            }
        }
    }

    fn set_r16(&mut self, idx: u8, val: u16) {
        match idx {
            reg16::BC => self.bc.value = val,
            reg16::DE => self.de.value = val,
            reg16::HL => self.hl.value = val,
            reg16::SP => self.sp.value = val,
            _ => panic!("Index out of range"),
        };
    }

    fn get_r16stk(&self, idx: u8) -> u16 {
        unsafe {
            match idx {
                reg16stk::BC => self.bc.value,
                reg16stk::DE => self.de.value,
                reg16stk::HL => self.hl.value,
                reg16stk::AF => self.af.value,
                _ => panic!("Index out of range")
            }
        }
    }

    fn set_r16stk(&mut self, idx: u8, val: u16) {
        match idx {
            reg16stk::BC => self.bc.value = val,
            reg16stk::DE => self.de.value = val,
            reg16stk::HL => self.hl.value = val,
            reg16stk::AF => self.af.value = val,
            _ => panic!("Index out of range")
        };
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
                    self.hl.value + 1
                },
                _ => panic!("Index out of range"),
            }
        }
    }

    pub fn is_flag_set(&self, flag: u8) -> bool {
        unsafe { (self.af.halves[0] & (flag as u8))  != 0 }
    }

    pub fn set_flag(&mut self, flag: u8) {
        unsafe { self.af.halves[0] = self.af.halves[0] | (flag as u8); }
    }

    pub fn clear_flag(&mut self, flag: u8) {
        unsafe { self.af.halves[0] = self.af.halves[0] & (!(flag as u8)); }
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