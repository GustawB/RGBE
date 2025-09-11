mod helpers;
pub mod types;

mod block_cb;
mod block_zero;
mod block_one;
mod block_two;
mod block_three;

use std::marker::PhantomData;

use crate::console::helpers::constants::IME;
pub use crate::console::helpers::constants::{reg8, flag};
pub use crate::console::helpers::common::debug_addr;
use crate::{console::{helpers::{constants::{cond, intr, reg16, reg16mem, reg16stk}}, types::Register}};
#[cfg(feature = "debugger")]
use crate::types::Hookable;

use std::sync::{Arc, Mutex};

use ppu::Ppu;

const ROM0_BASE: usize = 0x0;
const ROM1_BASE: usize = 0x4000;
const VRAM_BASE: usize = 0x8000;
const ERAM_BASE: usize = 0xA000;
const WRAM_BASE: usize = 0xC000;
const UNUSED_RAM_BASE: usize = 0xD000;
const OAM_BASE: usize = 0xFE00;
const PROHIBITED_BASE: usize = 0xFEA0;
const IO_REGS_BASE: usize = 0xFF00;
const HRAM_BASE: usize = 0xFF80;
const IE: usize = 0xFFFF;

const PALETTES_BASE: usize = 0xFF47;
const PALETTES_END: usize = 0xFF50;

pub struct Console<'a> {
    rom_bank_0: [u8; 0x4000],
    rom_bank_1: [u8; 0x4000],
    vram: Arc<Mutex<[u8; 0x2000]>>,
    eram: [u8; 0x2000],
    wram: [u8; 0x1000],
    oam: Arc<Mutex<[u8; 0x100]>>,
    io_regs: Arc<Mutex<[u8; 0x80]>>,
    hram: [u8; 0x7F],
    ie: Arc<Mutex<u8>>,
    ime: u8,

    palettes_lock: Mutex<()>,

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

impl<'a> Console<'a> {
    pub fn init(boot_rom: Vec<u8>) -> Result<Console<'a>, String> {
        if boot_rom.len() > 0x100 {
            Err(String::from("Boot rom too long"))
        } else {
            let mut tmp_rom_bank_0 = [0; 0x4000];
            for i in 0..boot_rom.len() {
                tmp_rom_bank_0[i] = boot_rom[i];
            }

            for i in 0..HEADER_SIZE {
                tmp_rom_bank_0[0x100 + i] = HEADER[i];
            }

            Ok(Console {
                rom_bank_0: tmp_rom_bank_0,
                rom_bank_1: [0; 0x4000],
                vram: Arc::new(Mutex::new([0; 0x2000])),
                eram: [0; 0x2000],
                wram: [0; 0x1000],
                oam: Arc::new(Mutex::new([0; 0x100])),
                io_regs: Arc::new(Mutex::new([0; 0x80])),
                hram: [0; 0x7F],
                ie: Arc::new(Mutex::new(0)),
                ime: 0,

                palettes_lock: Mutex::new(()),

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
        let res: u8 = unsafe { self.get_mem(self.ip.value as usize) };
        unsafe { self.ip.value += 1 };
        res
    }

    pub fn fetch_two_bytes(&mut self) -> u16 {
        let a: u8 = unsafe { self.get_mem(self.ip.value as usize) };
        let b: u8 = unsafe { self.get_mem(self.ip.value as usize + 1) };
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
        self.set_mem(sp as usize - 1, val);
    }

    fn stk_pop8(&mut self) -> u8 {
        let sp: u16 = self.get_r16(reg16::SP);
        let res: u8 = self.get_mem(sp as usize);
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
            self.set_mem(IME as usize, 1);
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

    pub fn set_ime(&mut self, val: u8) {
        match val {
            0 => self.ime = 0,
            _ => self.ime = 1,
        }
    }

    // Entry point of the console.
    pub fn execute(&mut self) {
        self.call_hook("".to_owned(), std::u16::MAX);

        /*let rc = Arc::new(addr_bus);
        let rc_clone = rc.clone();
        let handle = thread::spawn(move || {
            let mut ppu : Ppu = Ppu::new(rc_clone);
            ppu.execute();
        });*/
        
        loop {
            /*let (ime, ie, iflag) = self.addr_bus.get_intr_state();
            if ime == 1 {
                for i in 0..5 {
                    let mask: u8 = 1 << i;
                    if ie & mask == 1 && iflag & mask == 1{
                        self.handle_interrupt(mask);
                        break;
                    }
                }
            }*/

            self.step();
        }
        //handle.join().unwrap();
    }

    pub fn get_r8(&self, idx: u8) -> u8 {
        unsafe {
            match idx {
                reg8::B => self.bc.halves[1],
                reg8::C => self.bc.halves[0],
                reg8::D => self.de.halves[1],
                reg8::E => self.de.halves[0],
                reg8::H => self.hl.halves[1],
                reg8::L => self.hl.halves[0],
                reg8::HL_ADDR => self.get_mem(self.hl.value as usize),
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
                reg8::HL_ADDR => self.set_mem(self.hl.value as usize, val),
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

    pub fn get_mem(&self, addr: usize) -> u8 {
        match addr {
            (ROM0_BASE..ROM1_BASE) => self.rom_bank_0[addr],
            (ROM1_BASE..VRAM_BASE) => self.rom_bank_1[addr - ROM1_BASE],
            (VRAM_BASE..ERAM_BASE) => {
                match self.vram.try_lock() {
                    Ok(guard) => guard[addr - VRAM_BASE],
                    Err(_) => 0xFF, // garbage value
                }
            },
            (ERAM_BASE..WRAM_BASE) => self.eram[addr - ERAM_BASE],
            (WRAM_BASE..UNUSED_RAM_BASE) => self.wram[addr - WRAM_BASE], 
            (OAM_BASE..PROHIBITED_BASE) => {
                match self.oam.try_lock() {
                    Ok(guard) => guard[addr - OAM_BASE],
                    Err(_) => 0xFF // garbage value,
                }
            },
            (IO_REGS_BASE..HRAM_BASE) => {
                if (PALETTES_BASE..PALETTES_END).contains(&addr) {
                    match self.palettes_lock.try_lock() {
                        Ok(_) => self.io_regs.lock().unwrap()[addr - IO_REGS_BASE],
                        Err(_) => 0xFF, // garbage value
                    }
                } else {
                    self.io_regs.lock().unwrap()[addr - IO_REGS_BASE]
                }
            },
            (HRAM_BASE..IE) => self.hram[addr - HRAM_BASE],
            IE => *self.ie.lock().unwrap(),
            _ => panic!("Invalid address")
        }
    }

    pub fn set_mem(&mut self, addr: usize, val: u8) {
        match addr {
            (ROM0_BASE..ROM1_BASE) => {
                self.rom_bank_0[addr] = val;
            }
            (ROM1_BASE..VRAM_BASE) => {
                self.rom_bank_1[addr - ROM1_BASE] = val;
            },
            (VRAM_BASE..ERAM_BASE) => {
                match self.vram.try_lock() {
                    Ok(mut guard) => guard[addr - VRAM_BASE] = val,
                    Err(_) => (),
                };
            },
            (ERAM_BASE..WRAM_BASE) => {
                self.eram[addr - ERAM_BASE] = val;
            },
            (WRAM_BASE..UNUSED_RAM_BASE) => {
                self.wram[addr - WRAM_BASE] = val;
            },
            (OAM_BASE..PROHIBITED_BASE) => {
                match self.oam.try_lock() {
                    Ok(mut guard) => guard[addr - OAM_BASE] = val,
                    Err(_) => (),
                }
            },
            (IO_REGS_BASE..HRAM_BASE) => {
                if (PALETTES_BASE..PALETTES_END).contains(&addr) {
                    match self.palettes_lock.try_lock() {
                        Ok(_) => self.io_regs.lock().unwrap()[addr - IO_REGS_BASE] = val,
                        Err(_) => (),
                    }
                } else {
                    self.io_regs.lock().unwrap()[addr - IO_REGS_BASE] = val;
                }
            },
            (HRAM_BASE..IE) => self.hram[addr - HRAM_BASE] = val,
            IE => *self.ie.lock().unwrap() = val,
            _ => panic!("Invalid address")
        };
    }
}
