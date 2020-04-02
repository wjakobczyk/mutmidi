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
use alloc::rc::Rc;

use storage::MAX_PATCHES;
use synth::SynthRef;

use alloc::vec;
use alloc::vec::Vec;

struct State {
    patch_idx: u8,
}

type StateRef = Rc<RefCell<State>>;

fn setup_knobs<'a>(state: &mut StateRef) -> Vec<Knob<'a>> {
    let state = state.clone();

    vec![Knob::new(
        Point::new(KNOB_POS_X[0], KNOB_POS_Y),
        "Id",
        InputDeviceId::Knob1 as InputId,
        Box::new(move |delta: i8| {
            let mut state = state.borrow_mut();
            state.patch_idx =
                (state.patch_idx as i8 + delta).clamp(0, (MAX_PATCHES - 1) as i8) as u8;
            state.patch_idx
        }),
    )]
}

fn setup_buttons<'a>(
    active: i8,
    state: &mut StateRef,
    synth: &mut SynthRef,
    storage: &mut StorageRef,
) -> Vec<Button<'a>> {
    vec![
        Button::new(
            Point::new(BUTTON_POS_X[0], BUTTON_POS_Y),
            if active == 0 { "*Ld" } else { " Ld" },
            InputDeviceId::Button1 as InputId,
            {
                let storage = storage.clone();
                let synth = synth.clone();
                let state = state.clone();
                Box::new(move |_value: bool| {
                    let storage = storage.borrow();
                    let patch = storage.get_patch(state.borrow().patch_idx);
                    cortex_m::interrupt::free(|cs| {
                        synth.borrow(cs).borrow_mut().set_patch(patch);
                    });
                    true
                })
            },
        ),
        Button::new(
            Point::new(BUTTON_POS_X[1], BUTTON_POS_Y),
            if active == 0 { "*Sav" } else { " Sav" },
            InputDeviceId::Button2 as InputId,
            {
                let storage = storage.clone();
                let synth = synth.clone();
                let state = state.clone();
                Box::new(move |_value: bool| {
                    let mut patch = None;
                    cortex_m::interrupt::free(|cs| {
                        patch = Some(synth.borrow(cs).borrow_mut().get_patch());
                    });
                    storage
                        .borrow_mut()
                        .save_patch(state.borrow().patch_idx, &patch.unwrap());
                    true
                })
            },
        ),
        Button::new(
            Point::new(BUTTON_POS_X[2], BUTTON_POS_Y),
            "Exc",
            InputDeviceId::Button3 as InputId,
            Box::new(|_value: bool| {
                unsafe {
                    (*APP).change_panel(&mut *APP, PanelId::PanelBow);
                }
                true
            }),
        ),
        Button::new(
            Point::new(BUTTON_POS_X[3], BUTTON_POS_Y),
            "Res",
            InputDeviceId::Button4 as InputId,
            Box::new(|_value: bool| {
                unsafe {
                    (*APP).change_panel(&mut *APP, PanelId::PanelRes);
                }
                true
            }),
        ),
        Button::new(
            Point::new(BUTTON_POS_X[4], BUTTON_POS_Y),
            "Cfg",
            InputDeviceId::Button5 as InputId,
            Box::new(|_value: bool| true),
        ),
    ]
}

pub fn setup<'a>(
    synth: &mut SynthRef,
    storage: &mut StorageRef,
) -> (Vec<Button<'a>>, Vec<Knob<'a>>) {
    let mut state = Rc::new(RefCell::new(State { patch_idx: 0 }));

    (
        setup_buttons(1, &mut state, synth, storage),
        setup_knobs(&mut state),
    )
}
