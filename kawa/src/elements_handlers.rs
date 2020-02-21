// Copyright 2019 Wojciech Jakóbczyk
//
// Author: Wojciech Jakóbczyk (jakobczyk.woj@gmail.com)
//
// This file is part of Kawa Synth.
//
// Kawa Synth is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Kawa Synth is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Kawa Synth.  If not, see <https://www.gnu.org/licenses/>.

use crate::APP;
use alloc::boxed::Box;

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
    ResGeometry,
    ResBrightness,
    ResDamping,
    ResPosition,
    Space,
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
            cortex_m::interrupt::free(|cs| {
                let mut patch = (*APP).synth.shared_state.patch.borrow(cs).borrow_mut();

                patch.elements_params.$PARAM += (delta as f32) / KNOB_SCALER;
                patch.elements_params.$PARAM =
                    clamp(patch.elements_params.$PARAM, PARAM_MIN, PARAM_MAX);
                (patch.elements_params.$PARAM * KNOB_SCALER) as u8
            })
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
        Param::ResGeometry => param_bind!(resonator_geometry),
        Param::ResBrightness => param_bind!(resonator_brightness),
        Param::ResDamping => param_bind!(resonator_damping),
        Param::ResPosition => param_bind!(resonator_position),
        Param::Space => param_bind!(space),
    }
}
