use crate::ElementsParams;

pub const PATCH_NAME_SIZE: usize = 8;

#[derive(Debug, Default, Copy, Clone)]
pub struct Patch {
    pub name: [u8; PATCH_NAME_SIZE],
    pub elements_params: ElementsParams,
}

impl Patch {
    pub fn new() -> Self {
        Patch {
            name: [
                'e' as u8, 'm' as u8, 'p' as u8, 't' as u8, 'y' as u8, ' ' as u8, ' ' as u8,
                ' ' as u8,
            ],
            elements_params: ElementsParams {
                exciter_envelope_shape: 1.0,
                exciter_bow_level: 0.0,
                exciter_bow_timbre: 0.5,
                exciter_blow_level: 0.0,
                exciter_blow_meta: 0.5,
                exciter_blow_timbre: 0.5,
                exciter_strike_level: 0.800000012,
                exciter_strike_meta: 0.5,
                exciter_strike_timbre: 0.5,
                exciter_signature: 0.875,
                resonator_geometry: 0.200000003,
                resonator_brightness: 0.5,
                resonator_damping: 0.25,
                resonator_position: 0.300000012,
                resonator_modulation_frequency: 3.12500015e-05,
                resonator_modulation_offset: 0.112500004,
                reverb_diffusion: 0.606249988,
                reverb_lp: 0.824999988,
                reverb_time: 0.5,
                reverb_amount: 0.5,
                space: 0.7,
                dummy: 0.0,
            },
        }
    }
}
