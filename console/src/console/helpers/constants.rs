pub mod cond {
    pub const NZ: u8    = 0;
    pub const Z: u8     = 1;
    pub const NC: u8    = 2;
    pub const C: u8     = 3;

    pub fn get_cond_name(cc: u8) -> String {
        match cc {
            NZ => "NZ".to_string(),
            Z => "Z".to_string(),
            NC => "NC".to_string(),
            C => "C".to_string(),
            _ => panic!("Invalid condition code"),
        }
    }   
}

pub mod intr {
    pub const VBLANK: u8    = 0b00000001;
    pub const LCD: u8       = 0b00000010;
    pub const TIMER: u8     = 0b00000100;
    pub const SERIAL: u8    = 0b00001000;
    pub const JOYPAD: u8    = 0b00010000;

    pub fn intr_to_name(mask: u8) -> String {
        match mask {
            VBLANK  => "VBLANK".to_string(),
            LCD     => "LCD".to_string(),
            TIMER   => "TIMER".to_string(),
            SERIAL  => "SERIAL".to_string(),
            JOYPAD  => "JOYPAD".to_string(),
            _       => panic!("Unrecognized interrupt")
        }
    }

    pub fn get_jump_vector(mask: u8) -> u16 {
        match mask {
            VBLANK  => 0x40,
            LCD     => 0x48,
            TIMER   => 0x50,
            SERIAL  => 0x58,
            JOYPAD  => 0x60,
            _       => panic!("Unrecognized interrupt")
        }
    }
}

pub mod reg8 {
    use core::panic;

    pub const B: u8                 = 0;
    pub const C: u8                 = 1;
    pub const D: u8                 = 2;
    pub const E: u8                 = 3;
    pub const H: u8                 = 4;
    pub const L: u8                 = 5;
    pub const HL_ADDR: u8           = 6;
    pub const A: u8                 = 7;
    pub const EA: u8                = 8; // Explicit A (instead of r8 value)
    pub const MAX_REG8: usize       = 9;

    pub const LIST: [u8; MAX_REG8] = [B, C, D, E, H, L, HL_ADDR, A, EA];

    pub fn reg_to_name(reg: u8) -> String {
        match reg {
            B => "B".to_string(),
            C => "C".to_string(),
            D => "D".to_string(),
            E => "E".to_string(),
            H => "H".to_string(),
            L => "L".to_string(),
            HL_ADDR => "[HL]".to_string(),
            A => "A".to_string(),
            EA => "A".to_string(),
            _ => panic!("Unrecognized BYTE register"),
        }
    }
}

pub mod reg16 {
    pub const BC: u8    = 0;
    pub const DE: u8    = 1;
    pub const HL: u8    = 2;
    pub const SP: u8    = 3;

    pub fn reg_to_name(reg: u8) -> String {
        match reg {
            BC => "BC".to_string(),
            DE => "DE".to_string(),
            HL => "HL".to_string(),
            SP => "SP".to_string(),
            _ => panic!("Unrecognized WORD register"),
        }
    }
}

pub mod reg16stk {
    pub const BC: u8    = 0;
    pub const DE: u8    = 1;
    pub const HL: u8    = 2;
    pub const AF: u8    = 3;

    pub fn reg_to_name(reg: u8) -> String {
        match reg {
            BC => "BC".to_string(),
            DE => "DE".to_string(),
            HL => "HL".to_string(),
            AF => "AF".to_string(),
            _ => panic!("Unrecognized WORD register"),
        }   
    }
}

pub mod reg16mem {
    pub const BC: u8    = 0;
    pub const DE: u8    = 1;
    pub const HLI: u8    = 2;
    pub const HLD: u8    = 3;

    pub fn reg_to_name(reg: u8) -> String {
        match reg {
            BC => "BC".to_string(),
            DE => "DE".to_string(),
            HLI => "HLI".to_string(),
            HLD => "HLD".to_string(),
            _ => panic!("Unrecognized WORD register"),
        }
    }
}

pub mod flag {
    pub const Z: u8                 = 0x80;
    pub const N: u8                 = 0x40;
    pub const H: u8                 = 0x20;
    pub const C: u8                 = 0x10;

    pub const LIST: [u8; 4] = [Z, N, H, C];        

    pub fn flag_to_name(flag: u8) -> String {
        match flag {
            Z => "Z".to_string(),
            N => "N".to_string(),
            H => "H".to_string(),
            C => "C".to_string(),
            _ => panic!("Unrecognized flag"),
        }
    }
}
