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
use core::cell::RefCell;

use patch::PATCH_NAME_SIZE;
use storage::MAX_PATCHES;
use synth::SynthRef;

use alloc::vec;
use alloc::vec::Vec;

struct State {
    patch_idx: u8,
    patch_idx_text: Rc<RefCell<Content>>,
    patch_name: Rc<RefCell<Content>>,
}

type StateRef = Rc<RefCell<State>>;

fn setup_knobs<'a>(state: &mut StateRef, storage: &mut StorageRef) -> Vec<Knob<'a>> {
    let state = state.clone();
    let mut options = KnobOptions::default();
    options.render_value = false;

    vec![
        Knob::new(
            Point::new(KNOB_POS_X[0], KNOB_POS_Y),
            "Patch",
            InputDeviceId::Knob1 as InputId,
            {
                let state = state.clone();

                let storage = storage.clone();
                let state = state.clone();

                Box::new(move |delta: i8| {
                    let mut state = state.borrow_mut();
                    let storage = storage.borrow();
                    state.patch_idx =
                        (state.patch_idx as i8 + delta).clamp(0, (MAX_PATCHES - 1) as i8) as u8;
                    let patch_idx_text = vec![
                        '0' as u8 + state.patch_idx / 10,
                        '0' as u8 + state.patch_idx % 10,
                    ];
                    state.patch_idx_text.borrow_mut().bytes = patch_idx_text;
                    state.patch_idx_text.borrow_mut().is_dirty = true;

                    let patch = storage.get_patch(state.patch_idx);
                    state.patch_name.borrow_mut().bytes = patch.name.to_vec();
                    state.patch_name.borrow_mut().is_dirty = true;
                    state.patch_idx
                })
            },
            16,
            options,
        ),
        Knob::new(
            Point::new(KNOB_POS_X[1], KNOB_POS_Y),
            "Cursor",
            InputDeviceId::Knob2 as InputId,
            {
                let state = state.clone();
                Box::new(move |delta: i8| {
                    let state = state.borrow_mut();
                    let name = &mut state.patch_name.borrow_mut();
                    name.cursor_pos =
                        (name.cursor_pos + delta as i32).clamp(0, PATCH_NAME_SIZE as i32 - 1);
                    name.is_dirty = true;
                    name.cursor_pos as u8
                })
            },
            PATCH_NAME_SIZE as i32 - 1,
            options,
        ),
        Knob::new(
            Point::new(KNOB_POS_X[2], KNOB_POS_Y),
            "Char",
            InputDeviceId::Knob3 as InputId,
            {
                let state = state.clone();
                Box::new(move |delta: i8| {
                    let state = state.borrow_mut();
                    let name = &mut state.patch_name.borrow_mut();
                    let cursor_pos = name.cursor_pos as usize;
                    name.bytes[cursor_pos] = ((name.bytes[cursor_pos] as i16 + delta as i16) as u8)
                        .clamp(' ' as u8, '~' as u8);
                    name.is_dirty = true;
                    name.bytes[cursor_pos]
                })
            },
            '~' as i32,
            options,
        ),
    ]
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
            if active == 0 { "*Load" } else { " Load" },
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
                    state.borrow_mut().patch_name.borrow_mut().bytes = patch.name.to_vec();
                    state.borrow_mut().patch_name.borrow_mut().is_dirty = true;
                    true
                })
            },
        ),
        Button::new(
            Point::new(BUTTON_POS_X[1], BUTTON_POS_Y),
            if active == 0 { "*Save" } else { " Save" },
            InputDeviceId::Button2 as InputId,
            {
                let storage = storage.clone();
                let synth = synth.clone();
                let state = state.clone();
                Box::new(move |_value: bool| {
                    let mut patch = None;
                    cortex_m::interrupt::free(|cs| {
                        let mut patch_inner = synth.borrow(cs).borrow_mut().get_patch();
                        patch_inner.name = [' ' as u8; PATCH_NAME_SIZE];
                        let mut i = 0;
                        for ch in &state.borrow().patch_name.borrow().bytes {
                            patch_inner.name[i] = *ch;
                            i += 1;
                            if i >= PATCH_NAME_SIZE {
                                break;
                            }
                        }
                        patch = Some(patch_inner);
                    });
                    let patch = patch.unwrap();
                    storage
                        .borrow_mut()
                        .save_patch(state.borrow().patch_idx, &patch);
                    true
                })
            },
        ),
        Button::new(
            Point::new(BUTTON_POS_X[2], BUTTON_POS_Y),
            "Excite",
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

fn setup_texts<'a>(state: &mut StateRef) -> Vec<TextBox> {
    let state = state.borrow_mut();
    vec![
        TextBox::new(Point::new(KNOB_POS_X[1], 20), state.patch_name.clone()),
        TextBox::new(Point::new(KNOB_POS_X[0], 20), state.patch_idx_text.clone()),
    ]
}

pub fn setup<'a>(
    synth: &mut SynthRef,
    storage: &mut StorageRef,
) -> (Vec<Button<'a>>, Vec<Knob<'a>>, Vec<TextBox>) {
    let mut patch = None;
    cortex_m::interrupt::free(|cs| {
        patch = Some(synth.borrow(cs).borrow_mut().get_patch());
    });

    if let Some(patch) = patch {
        let mut state = Rc::new(RefCell::new(State {
            patch_idx: 0,
            patch_idx_text: Rc::new(RefCell::new(Content {
                bytes: vec!['0' as u8, '0' as u8],
                is_dirty: false,
                cursor_pos: 2,
            })),
            patch_name: Rc::new(RefCell::new(Content {
                bytes: patch.name.to_vec(),
                is_dirty: false,
                cursor_pos: 0,
            })),
        }));

        let texts = setup_texts(&mut state);

        (
            setup_buttons(1, &mut state, synth, storage),
            setup_knobs(&mut state, storage),
            texts,
        )
    } else {
        panic!();
    }
}
