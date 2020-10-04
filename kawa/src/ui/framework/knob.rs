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

use super::Drawable;
use super::*;
use alloc::boxed::Box;
use core::cmp::max;
use embedded_graphics::{
    drawable::Drawable as EmbeddedDrawable,
    fonts::Text,
    fonts::{Font6x6, Font6x8},
    prelude::*,
    style::TextStyleBuilder,
    DrawTarget,
};
use numtoa::NumToA;

#[derive(Copy, Clone)]
pub struct KnobOptions {
    pub render_value: bool,
}

pub struct Knob<'a> {
    pos: Point,
    caption: &'a str,
    input_id: InputId,
    value: u8,
    last_input_value: Option<i32>,
    dirty: bool,
    options: KnobOptions,
    handler: Box<dyn FnMut(i8) -> u8>,
}

impl<'a> core::fmt::Debug for Knob<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Knob({})", self.caption)
    }
}

impl KnobOptions {
    pub fn default() -> Self {
        KnobOptions { render_value: true }
    }
}

impl<'a> Knob<'a> {
    pub fn new(
        pos: Point,
        caption: &'a str,
        input_id: InputId,
        mut handler: Box<dyn FnMut(i8) -> u8>,
        options: KnobOptions,
    ) -> Self {
        Knob {
            pos,
            caption,
            input_id,
            value: (handler)(0),
            last_input_value: None,
            dirty: true,
            options,
            handler,
        }
    }
}

impl Drawable for Knob<'_> {
    fn render(&mut self, drawing: &mut impl DrawTarget<BinaryColor>) -> (Point, Size) {
        let style = TextStyleBuilder::new(Font6x6)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();

        const TEXT_OFFSET_Y: i32 = 18;
        let text = Text::new(&self.caption, self.pos + Point::new(0, TEXT_OFFSET_Y)).into_styled(style);
        let _ = text.draw(drawing);

        let text_size = text.size();

        let mut value_size = Size {
            width: 0,
            height: 0,
        };

        let style_value = TextStyleBuilder::new(Font6x8)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();

        if self.options.render_value {
            let mut buffer = [0u8; 3];
            self.value.numtoa(10, &mut buffer);
            if buffer[buffer.len() - 2] == 0 {
                buffer[buffer.len() - 2] = b' ';
            }
            let text = &buffer[buffer.len() - 2..buffer.len()];

            let text = Text::new(
                unsafe { core::str::from_utf8_unchecked(text) },
                self.pos,
            )
            .into_styled(style_value);
            if !text.draw(drawing).is_ok() {
                panic!();
            }

            value_size = text.size();
        }

        self.dirty = false;

        (
            self.pos,
            Size::new(
                max(text_size.width, value_size.width),
                text_size.height + TEXT_OFFSET_Y as u32,
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
                        self.dirty |= self.options.render_value;
                    }
                } else {
                    self.value = (self.handler)(0);
                    self.dirty |= self.options.render_value;
                }
                self.last_input_value = Some(input_value);
            }
        }
    }
}
