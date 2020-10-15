use crate::patch::*;
use crate::util::clamp;
use crate::{
    Elements_GetPatch, Elements_RetriggerGate, Elements_SetGate, Elements_SetNote,
    Elements_SetPitchModulation, Elements_SetStrength,
};
use alloc::vec::Vec;

pub enum VoiceParam {
    ResonatorPosition,
}

pub enum VoiceEvent {
    NoteOn {
        retrigger: bool,
        note: f32,
        strength: f32,
    },
    NoteOff,
    ChangePitchModulation(f32),
    ChangeStrength(f32),
    ChangeParam(VoiceParam, f32),
}

pub struct Voice {
    resonator_modulation: f32,
}

impl Voice {
    pub fn new() -> Self {
        Voice {
            resonator_modulation: 0f32,
        }
    }

    pub fn handle_events(&mut self, voice_events: &Vec<VoiceEvent>) {
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
                VoiceEvent::ChangePitchModulation(modulation) => unsafe {
                    Elements_SetPitchModulation(*modulation);
                },
                VoiceEvent::ChangeStrength(value) => unsafe {
                    Elements_SetStrength(*value);
                },
                VoiceEvent::ChangeParam(VoiceParam::ResonatorPosition, value) => {
                    self.resonator_modulation = *value
                }
            }
        }
    }

    pub fn update_patch(&self, patch: &Patch) {
        unsafe {
            let elements_params = &mut *Elements_GetPatch();

            *elements_params = patch.elements_params;

            elements_params.resonator_position = clamp(
                elements_params.resonator_position + self.resonator_modulation,
                0.0,
                1.0,
            );
        }
    }
}
