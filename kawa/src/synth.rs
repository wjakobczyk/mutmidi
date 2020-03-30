use crate::patch::*;
use crate::util::QueueThreadSafe;
use crate::voice::*;
use alloc::sync::Arc;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

pub type VoiceEventsQueue = QueueThreadSafe<VoiceEvent>;

pub type SynthRef = Arc<Mutex<RefCell<Synth>>>;

pub struct Synth {
    pub voice: Voice,
    pub voice_events: VoiceEventsQueue,
    pub patch: RefCell<Patch>,
}

const INIT_EVENTS_CAPACITY: usize = 4;

impl Synth {
    pub fn new() -> Self {
        Synth {
            voice: Voice::new(),
            voice_events: VoiceEventsQueue::new(INIT_EVENTS_CAPACITY),
            patch: RefCell::new(Patch::new()),
        }
    }

    pub fn get_patch(&self) -> Patch {
        self.patch.borrow().clone()
    }

    pub fn set_patch(&mut self, new_patch: &Patch) {
        let mut patch = self.patch.borrow_mut();
        *patch = *new_patch;
    }

    pub fn test(&mut self) {}
}
