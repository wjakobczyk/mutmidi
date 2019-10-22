#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Patch {
    pub exciter_envelope_shape: f32,
    pub exciter_bow_level: f32,
    pub exciter_bow_timbre: f32,
    pub exciter_blow_level: f32,
    pub exciter_blow_meta: f32,
    pub exciter_blow_timbre: f32,
    pub exciter_strike_level: f32,
    pub exciter_strike_meta: f32,
    pub exciter_strike_timbre: f32,
    pub exciter_signature: f32,
    pub resonator_geometry: f32,
    pub resonator_brightness: f32,
    pub resonator_damping: f32,
    pub resonator_position: f32,
    pub resonator_modulation_frequency: f32,
    pub resonator_modulation_offset: f32,
    pub reverb_diffusion: f32,
    pub reverb_lp: f32,
    pub space: f32,
    pub modulation_frequency: f32,
}

#[link(name = "elements")]
extern "C" {
    pub fn Init(application: bool);
    pub fn GetPatch() -> *mut Patch;
    pub fn SetGate(newGate: cty::c_int);
    pub fn Elements_DMA1_Stream5_IRQHandler();
}
