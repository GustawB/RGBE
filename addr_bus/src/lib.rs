use std::sync::Mutex;

pub const ADDR_BUS_SIZE: usize  = 65535;

#[derive(Clone, Copy)]
pub struct IntrState {
    pub ime: u8,
    pub ie: u8,
    pub iflag: u8,
}

pub struct AddrBus {
    addr_bus: [u8; ADDR_BUS_SIZE],
    intr_state: Mutex<IntrState>,
}

impl AddrBus {
    pub fn new (addr_bus: [u8; ADDR_BUS_SIZE]) -> AddrBus {
        AddrBus {
            addr_bus: addr_bus,
            intr_state: Mutex::new(IntrState { ime: 0, ie: 0, iflag: 0 })
        }
    }

    pub fn get_intr_state(&self) -> IntrState {
        *self.intr_state.lock().unwrap()
    }

    pub fn set_intr_state(&mut self, new_state: IntrState) {
        *self.intr_state.lock().unwrap() = new_state;
    }

    pub fn get(&self, addr: u16) -> u8 {
        0
    }
}
