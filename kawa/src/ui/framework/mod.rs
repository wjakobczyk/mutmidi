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

pub mod button;
pub mod knob;
pub mod panel;
pub mod test;
pub mod textbox;

pub use button::Button;
pub use knob::{Knob, KnobOptions};
pub use panel::Panel;
pub use textbox::{Content, TextBox};

pub use embedded_graphics::{
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
    DrawTarget,
};

#[derive(Copy, Clone)]
pub enum Value {
    Bool(bool),
    Int(i32),
}

pub type InputId = u32;

pub trait Drawable {
    fn render(&mut self, drawing: &mut impl DrawTarget<BinaryColor>) -> (Point, Size);
    fn is_dirty(&self) -> bool;
}

pub trait InputConsumer {
    fn input_reset(&mut self);
    fn input_update(&mut self, input_id: InputId, value: Value);
}
