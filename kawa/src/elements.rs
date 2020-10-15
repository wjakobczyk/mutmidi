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

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct ElementsParams {
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
    pub reverb_amount: f32,
    pub reverb_time: f32,
    pub space: f32,
    
    pub dummy: f32,
}

#[link(name = "elements")]
extern "C" {
    pub fn Elements_Init(application: bool);
    pub fn Elements_GetPatch() -> *mut ElementsParams;
    pub fn Elements_SetGate(newGate: bool);
    pub fn Elements_RetriggerGate();
    pub fn Elements_SetNote(newNote: f32);
    pub fn Elements_SetStrength(newStrength: f32);
    pub fn Elements_SetPitchModulation(newModulation: f32);
    pub fn Elements_Pause(pause: bool);
    pub fn Elements_DMA1_Stream5_IRQHandler();
}
