use alloc::boxed::Box;

include!("elements.rs");

#[allow(dead_code)]
pub enum Param {
    ExcEnvShape,
    ExcBowLevel,
    ExcBowTimbre,
    ExcBlowLevel,
    ExcBlowMeta,
    ExcBlowTimbre,
    ExcStrikeLevel,
    ExcStrikeMeta,
    ExcStrikeTimbre,
    ExcSignature,
    ResGeometry,
    ResBrightness,
    ResDamping,
    ResPosition,
    ResModulationFrequency,
    ResModulationOffset,
    ReverbDiffusion,
    ReverbLP,
    Space,
    ModulationFrequency,
}

const KNOB_SCALER: f32 = 20f32;
const PARAM_MIN: f32 = 0.0;
const PARAM_MAX: f32 = 0.9995;

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else {
        if value > max {
            max
        } else {
            value
        }
    }
}

macro_rules! param_bind {
    ($PARAM:ident) => {
        Box::new(|delta: i8| unsafe {
            let patch = &mut *GetPatch();
            patch.$PARAM += (delta as f32) / KNOB_SCALER;
            patch.$PARAM = clamp(patch.$PARAM, PARAM_MIN, PARAM_MAX);
            (patch.$PARAM * KNOB_SCALER) as u8
        })
    };
}

pub fn create_knob_handler(param: Param) -> Box<dyn FnMut(i8) -> u8> {
    match param {
        Param::ExcEnvShape => param_bind!(exciter_envelope_shape),
        Param::ExcBowLevel => param_bind!(exciter_bow_level),
        Param::ExcBowTimbre => param_bind!(exciter_bow_timbre),
        Param::ExcBlowLevel => param_bind!(exciter_blow_level),
        Param::ExcBlowMeta => param_bind!(exciter_blow_meta),
        Param::ExcBlowTimbre => param_bind!(exciter_blow_timbre),
        Param::ExcStrikeLevel => param_bind!(exciter_strike_level),
        Param::ExcStrikeMeta => param_bind!(exciter_strike_meta),
        Param::ExcStrikeTimbre => param_bind!(exciter_strike_timbre),
        Param::ExcSignature => param_bind!(exciter_signature),
        Param::ResGeometry => param_bind!(resonator_geometry),
        Param::ResBrightness => param_bind!(resonator_brightness),
        Param::ResDamping => param_bind!(resonator_damping),
        Param::ResPosition => param_bind!(resonator_position),
        Param::ResModulationFrequency => param_bind!(resonator_modulation_frequency),
        Param::ResModulationOffset => param_bind!(resonator_modulation_offset),
        Param::ReverbDiffusion => param_bind!(reverb_diffusion),
        Param::ReverbLP => param_bind!(reverb_lp),
        Param::Space => param_bind!(space),
        Param::ModulationFrequency => param_bind!(modulation_frequency),
    }
}
