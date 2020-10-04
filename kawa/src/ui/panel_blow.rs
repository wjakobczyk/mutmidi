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

use super::framework::*;
use super::*;

use crate::elements_handlers::*;

use alloc::vec;
use alloc::vec::Vec;

fn setup_knobs<'a>() -> Vec<Knob<'a>> {
    vec![
        Knob::new(
            Point::new(KNOB_POS_X[0], KNOB_POS_Y),
            "Level",
            InputDeviceId::Knob1 as InputId,
            create_knob_handler(Param::ExcBlowLevel),
            KNOB_MAX_VALUE,
            KnobOptions::default(),
        ),
        Knob::new(
            Point::new(KNOB_POS_X[1], KNOB_POS_Y),
            "Timbre",
            InputDeviceId::Knob2 as InputId,
            create_knob_handler(Param::ExcBlowTimbre),
            KNOB_MAX_VALUE,
            KnobOptions::default(),
        ),
        Knob::new(
            Point::new(KNOB_POS_X[2], KNOB_POS_Y),
            "Cont",
            InputDeviceId::Knob3 as InputId,
            create_knob_handler(Param::ExcEnvShape),
            KNOB_MAX_VALUE,
            KnobOptions::default(),
        ),
        Knob::new(
            Point::new(KNOB_POS_X[3], KNOB_POS_Y),
            "Flow",
            InputDeviceId::Knob4 as InputId,
            create_knob_handler(Param::ExcBlowMeta),
            KNOB_MAX_VALUE,
            KnobOptions::default(),
        ),
    ]
}

pub fn setup<'a>() -> (Vec<Button<'a>>, Vec<Knob<'a>>, Vec<TextBox>) {
    (
        super::panel_bow::setup_exciter_buttons(1),
        setup_knobs(),
        vec![],
    )
}
