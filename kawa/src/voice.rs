use crate::patch::*;
use crate::{
    Elements_GetPatch, Elements_RetriggerGate, Elements_SetGate, Elements_SetModulation,
    Elements_SetNote, Elements_SetStrength,
};
use alloc::vec::Vec;

pub enum VoiceEvent {
    NoteOn {
        retrigger: bool,
        note: f32,
        strength: f32,
    },
    NoteOff,
    ChangeModulation(f32),
}

pub struct Voice {}

impl Voice {
    pub fn new() -> Self {
        Voice {}
    }

    fn handle_events(&self, voice_events: &Vec<VoiceEvent>) {
        for event in voice_events {
            match event {
                VoiceEvent::NoteOn {
                    retrigger,
                    note,
                    strength,
                } => unsafe {
                    Elements_SetGate(true);
                    if *retrigger {
                        Elements_RetriggerGate();
                    }
                    Elements_SetNote(*note);
                    Elements_SetStrength(*strength);
                },
                VoiceEvent::NoteOff => unsafe {
                    Elements_SetGate(false);
                },
                VoiceEvent::ChangeModulation(modulation) => unsafe {
                    Elements_SetModulation(*modulation);
                },
            }
        }
    }

    fn update_patch(&self, patch: &Patch) {
        unsafe {
            let elements_params = &mut *Elements_GetPatch();

            //will add modulation here
            *elements_params = patch.elements_params;
        }
    }

    pub fn update(&self, voice_events: &Vec<VoiceEvent>, patch: &Patch) {
        self.handle_events(voice_events);
        self.update_patch(patch);
    }
}
