use crate::ElementsParams;

#[derive(Debug, Default, Copy, Clone)]
pub struct Patch {
    pub elements_params: ElementsParams,
}

impl Patch {
    pub fn new() -> Self {
        Patch {
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
                space: 0.5,
                modulation_frequency: 0.0,
            },
        }
    }
}
