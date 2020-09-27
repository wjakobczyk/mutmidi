use crate::patch::*;
use alloc::boxed::Box;
use stm32_flash::Flash;

pub const MAX_PATCHES: usize = 16;

struct Store {
    patches: [Patch; MAX_PATCHES],
}

pub struct Storage {
    flash: Flash,
    store: Box<Store>,
}

impl Storage {
    pub fn new(flash: Flash) -> Self {
        let mut store = Box::new(Store {
            patches: [Patch::new(); MAX_PATCHES],
        });
        flash.read_into(0, &mut *store);

        for patch in &mut store.patches {
            if patch.name[0] == 0xff {
                *patch = Patch::new();
            }
        }

        Storage { flash, store }
    }

    pub fn flush(&self) {
        self.flash.erase().unwrap();
        self.flash.write(0, &*self.store).unwrap();
    }

    pub fn save_patch(&mut self, idx: u8, patch: &Patch) {
        self.store.patches[idx as usize] = *patch;
        self.flush();
    }

    pub fn get_patch(&self, idx: u8) -> &Patch {
        &self.store.patches[idx as usize]
    }
}
