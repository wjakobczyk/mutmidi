// Copyright 2019 Wojciech Jakóbczyk
//
// Author: Wojciech Jakóbczyk (jakobczyk.woj@gmail.com)
//
// This file is part of MidiElements.
//
// MidiElements is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// MidiElements is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with MidiElements.  If not, see <https://www.gnu.org/licenses/>.

pub mod framework;
pub mod panel_blow;
pub mod panel_bow;
pub mod panel_out;
pub mod panel_res;
pub mod panel_strike;

pub const KNOB_POS_X: [i32; 4] = [0, 32, 64, 96];
pub const KNOB_POS_Y: i32 = 40;
pub const BUTTON_POS_X: [i32; 5] = [0, 26, 51, 77, 102];
pub const BUTTON_POS_Y: i32 = 0;
