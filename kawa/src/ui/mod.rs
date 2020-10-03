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

pub mod framework;
pub mod panel_blow;
pub mod panel_bow;
pub mod panel_out;
pub mod panel_patch;
pub mod panel_res;
pub mod panel_strike;

use super::framework::*;
use super::*;
use crate::ui::framework::Drawable;

use synth::SynthRef;

pub const KNOB_POS_X: [i32; 4] = [0, 32, 64, 96];
pub const KNOB_POS_Y: i32 = 40;
pub const BUTTON_POS_X: [i32; 5] = [0, 26, 51, 77, 102];
pub const BUTTON_POS_Y: i32 = 0;

#[derive(Clone, Copy)]
pub enum InputDeviceId {
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Knob1,
    Knob2,
    Knob3,
    Knob4,
}

pub enum PanelId {
    PanelBow,
    PanelBlow,
    PanelStrike,
    PanelRes,
    PanelOutput,
    PanelPatch,
}

pub struct UI<'a> {
    button_states: [bool; 5],
    panels: Option<[Panel<'a>; 6]>,
    current_panel: Option<&'a mut Panel<'a>>,
}

impl<'a> UI<'a> {
    pub fn new() -> Self {
        UI {
            button_states: [false; 5],
            panels: None,
            current_panel: None,
        }
    }

    pub fn setup(&mut self, synth: &mut SynthRef, storage: &mut StorageRef) {
        self.panels = Some([
            Panel::new(panel_bow::setup()),
            Panel::new(panel_blow::setup()),
            Panel::new(panel_strike::setup()),
            Panel::new(panel_res::setup()),
            Panel::new(panel_out::setup()),
            Panel::new(panel_patch::setup(synth, storage)),
        ])
    }

    pub fn update_knobs(&mut self, values: (Value, Value, Value, Value)) {
        if let Some(panel) = &mut self.current_panel {
            panel.input_update(InputDeviceId::Knob1 as InputId, values.0);
            panel.input_update(InputDeviceId::Knob2 as InputId, values.1);
            panel.input_update(InputDeviceId::Knob3 as InputId, values.2);
            panel.input_update(InputDeviceId::Knob4 as InputId, values.3);
        };
    }

    fn update_button(&mut self, id: InputDeviceId, value: bool) {
        if value && value != self.button_states[id as usize] {
            if let Some(panel) = &mut self.current_panel {
                panel.input_update(id as InputId, Value::Bool(value));
            };
        }
        self.button_states[id as usize] = value;
    }

    pub fn update_buttons(&mut self, values: (bool, bool, bool, bool, bool)) {
        self.update_button(InputDeviceId::Button1, values.0);
        self.update_button(InputDeviceId::Button2, values.1);
        self.update_button(InputDeviceId::Button3, values.2);
        self.update_button(InputDeviceId::Button4, values.3);
        self.update_button(InputDeviceId::Button5, values.4);
    }

    pub fn render(&mut self, drawing: &mut impl DrawTarget<BinaryColor>) -> Option<(Point, Size)> {
        if let Some(panel) = &mut self.current_panel {
            Some(panel.render(drawing))
        } else {
            None
        }
    }

    pub fn change_panel(&mut self, self2: &'a mut UI<'a>, panel: PanelId) {
        if let Some(panels) = &mut self2.panels {
            self.current_panel = Some(&mut panels[panel as usize]);
        }

        if let Some(panel) = &mut self.current_panel {
            panel.input_reset();
        }
    }
}
