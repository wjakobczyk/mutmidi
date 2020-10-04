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
use embedded_graphics::{
    drawable::Drawable as EmbeddedDrawable,
    fonts::Font6x6,
    fonts::Text,
    prelude::*,
    primitives::{Circle, Line},
    style::{PrimitiveStyle, PrimitiveStyleBuilder, Styled, TextStyle, TextStyleBuilder},
    DrawTarget,
};
use micromath::F32Ext;

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
    max_value: i32,
    dirty: bool,
    dirty_text: bool,
    options: KnobOptions,
    handler: Box<dyn FnMut(i8) -> u8>,
    text: Styled<Text<'a>, TextStyle<BinaryColor, Font6x6>>,
    circle: Styled<Circle, PrimitiveStyle<BinaryColor>>,
    line: Styled<Line, PrimitiveStyle<BinaryColor>>,
}

const TEXT_OFFSET_Y: i32 = 14;
const KNOB_SIZE: u32 = 10;

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
        max_value: i32,
        options: KnobOptions,
    ) -> Self {
        let text_style = TextStyleBuilder::new(Font6x6)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();
        let text_x = Text::new(&caption, pos)
            .into_styled(text_style)
            .size()
            .width as i32
            / -2;
        let text =
            Text::new(&caption, pos + Point::new(text_x, TEXT_OFFSET_Y)).into_styled(text_style);
        let circle_style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(1)
            .fill_color(BinaryColor::Off)
            .build();
        let circle = Circle::new(pos, KNOB_SIZE).into_styled(circle_style);
        let line = Line::new(pos, pos).into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 3));
        Knob {
            pos,
            caption,
            input_id,
            value: (handler)(0),
            last_input_value: None,
            max_value,
            dirty: true,
            dirty_text: true,
            options,
            handler,
            text,
            circle,
            line,
        }
    }
}

impl Drawable for Knob<'_> {
    fn render(&mut self, drawing: &mut impl DrawTarget<BinaryColor>) -> (Point, Size) {
        if self.dirty_text {
            if !self.text.draw(drawing).is_ok() {
                panic!();
            }
            self.dirty_text = false;                
        }

        if self.options.render_value {
            if !self.circle.draw(drawing).is_ok() {
                panic!();
            }

            let angle = self.value as f32 * core::f32::consts::PI * 2.0 * 6.0 / 8.0 / (self.max_value as f32)
                + core::f32::consts::FRAC_PI_4 * 3.0;
            self.line.primitive.end = self.pos
                + Point::new(
                    (KNOB_SIZE as f32 * angle.cos()) as i32,
                    (KNOB_SIZE as f32 * angle.sin()) as i32,
                );
            if !self.line.draw(drawing).is_ok() {
                panic!();
            }

            self.dirty = false;

            (
                self.pos - Point::new(KNOB_SIZE as i32, KNOB_SIZE as i32),
                Size::new(KNOB_SIZE * 2, KNOB_SIZE * 2),
            )
        } else {
            (Point::new(0, 0), Size::new(0, 0))
        }
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl InputConsumer for Knob<'_> {
    fn input_reset(&mut self) {
        self.last_input_value = None;
        self.dirty_text = true;
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
