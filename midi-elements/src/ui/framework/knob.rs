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

use super::*;
use alloc::boxed::Box;
use core::cmp::max;
use embedded_graphics::{fonts::Font6x12, prelude::*};
use numtoa::NumToA;

pub struct Knob<'a> {
    pos: Point,
    caption: &'a str,
    input_id: InputId,
    value: u8,
    last_input_value: Option<i32>,
    dirty: bool,
    handler: Box<dyn FnMut(i8) -> u8>,
}

impl<'a> core::fmt::Debug for Knob<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Knob({})", self.caption)
    }
}

impl<'a> Knob<'a> {
    pub fn new(
        pos: Point,
        caption: &'a str,
        input_id: InputId,
        mut handler: Box<dyn FnMut(i8) -> u8>,
    ) -> Self {
        Knob {
            pos,
            caption,
            input_id,
            value: (handler)(0),
            last_input_value: None,
            dirty: true,
            handler,
        }
    }
}

impl Drawable for Knob<'_> {
    fn render(&mut self, drawing: &mut impl Drawing<BinaryColor>) -> (Point, Size) {
        let render_caption = Font6x12::render_str(&self.caption)
            .fill(Some(BinaryColor::Off))
            .stroke(Some(BinaryColor::On))
            .translate(self.pos);
        drawing.draw(render_caption);

        let mut buffer = [0u8; 3];
        self.value.numtoa(10, &mut buffer);
        if buffer[buffer.len() - 2] == 0 {
            buffer[buffer.len() - 2] = b' ';
        }
        let text = &buffer[buffer.len() - 2..buffer.len()];
        let render_value = Font6x12::render_str(unsafe { core::str::from_utf8_unchecked(text) })
            .fill(Some(BinaryColor::Off))
            .stroke(Some(BinaryColor::On))
            .translate(self.pos + Point::new(0, render_caption.size().height as i32));
        drawing.draw(render_value);
        self.dirty = false;

        (
            self.pos,
            Size::new(
                max(render_caption.size().width, render_value.size().width),
                render_caption.size().height + render_value.size().height,
            ),
        )
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl InputConsumer for Knob<'_> {
    fn input_reset(&mut self) {
        self.last_input_value = None;
    }

    fn input_update(&mut self, input_id: InputId, value: Value) {
        if let Value::Int(input_value) = value {
            if input_id == self.input_id {
                if let Some(last_input_value) = self.last_input_value {
                    let delta = input_value - last_input_value;
                    if delta != 0 {
                        self.value = (self.handler)(delta as i8);
                        self.dirty = true;
                    }
                } else {
                    self.value = (self.handler)(0);
                    self.dirty = true;
                }
                self.last_input_value = Some(input_value);
            }
        }
    }
}
