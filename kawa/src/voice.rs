use crate::patch::*;
use crate::util::clamp;
use crate::{
    Elements_GetPatch, Elements_RetriggerGate, Elements_SetGate, Elements_SetNote,
    Elements_SetPitchModulation, Elements_SetStrength,
};
use alloc::vec::Vec;

pub enum VoiceParam {
    ResonatorPosition,
    CCParam(u8),
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

macro_rules! cc_param {
    ($cc:expr, $value:ident) => {
        VoiceEvent::ChangeParam(VoiceParam::CCParam($cc), $value)
    };
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

    pub fn handle_voice_events(&mut self, voice_events: &Vec<VoiceEvent>) {
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
                VoiceEvent::ChangeParam(_, _) => {}
            }
        }
    }

    pub fn handle_patch_events(&self, voice_events: &Vec<VoiceEvent>, patch: &mut Patch) {
        let params = &mut patch.elements_params;

        for event in voice_events {
            match event {
                cc_param!(14, value) => params.exciter_envelope_shape = *value,
                cc_param!(15, value) => params.exciter_bow_level = *value,
                cc_param!(16, value) => params.exciter_bow_timbre = *value,

                cc_param!(18, value) => params.exciter_blow_level = *value,
                cc_param!(19, value) => params.exciter_blow_timbre = *value,
                cc_param!(20, value) => params.exciter_blow_meta = *value,

                cc_param!(22, value) => params.exciter_strike_level = *value,
                cc_param!(23, value) => params.exciter_strike_timbre = *value,
                cc_param!(24, value) => params.exciter_strike_meta = *value,

                cc_param!(26, value) => params.resonator_brightness = *value,
                cc_param!(27, value) => params.resonator_damping = *value,
                cc_param!(28, value) => params.resonator_position = *value,
                cc_param!(29, value) => params.resonator_modulation_frequency = *value,
                cc_param!(30, value) => params.resonator_modulation_offset = *value,

                cc_param!(80, value) => params.reverb_diffusion = *value,
                cc_param!(81, value) => params.reverb_lp = *value,
                cc_param!(82, value) => params.reverb_amount = *value,
                cc_param!(83, value) => params.reverb_time = *value,
                _ => {}
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
