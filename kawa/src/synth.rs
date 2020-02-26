use crate::patch::*;
use crate::util::QueueThreadSafe;
use crate::voice::*;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use stm32_flash::Flash;

pub type VoiceEventsQueue = QueueThreadSafe<VoiceEvent>;

///3 threads: main/ui, audio irq, midi uart irq
pub struct SynthSharedState {
    ///called in audio irq thread only
    pub voice: Mutex<Voice>,

    ///called in main and midi threads (enque) and audio irq (deque)
    pub voice_events: VoiceEventsQueue,

    //updated in main/UI and retrieved in audio irq
    pub patch: Mutex<RefCell<Patch>>,
}

pub struct Synth {
    pub shared_state: SynthSharedState,
}

const INIT_EVENTS_CAPACITY: usize = 4;

impl Synth {
    pub fn new() -> Self {
        Synth {
            shared_state: SynthSharedState {
                voice: Mutex::new(Voice::new()),
                voice_events: VoiceEventsQueue::new(INIT_EVENTS_CAPACITY),
                patch: Mutex::new(RefCell::new(Patch::new())),
            },
        }
    }

    pub fn save_patch(&self, flash: &mut Flash, offset: usize) {
        let mut patch_copy = None;

        cortex_m::interrupt::free(|cs| {
            patch_copy = Some(self.shared_state.patch.borrow(cs).borrow().clone());
        });

        flash.write(offset, &patch_copy.unwrap()).unwrap();
    }

    pub fn load_patch(&mut self, flash: &Flash, offset: usize) {
        cortex_m::interrupt::free(|cs| {
            let mut patch = self.shared_state.patch.borrow(cs).borrow_mut();
            *patch = flash.read(offset);
        });
    }
}
