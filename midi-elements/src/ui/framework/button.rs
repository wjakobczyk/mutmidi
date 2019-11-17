use super::*;
use alloc::boxed::Box;
use embedded_graphics::{fonts::Font6x12, prelude::*};

pub struct Button<'a> {
    pos: Point,
    caption: &'a str,
    input_id: InputId,
    pressed: bool,
    dirty: bool,
    handler: Box<dyn FnMut(bool) -> bool>,
}

impl<'a> core::fmt::Debug for Button<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Button({})", self.caption)
    }
}

impl<'a> Button<'a> {
    pub fn new(
        pos: Point,
        caption: &'a str,
        input_id: InputId,
        handler: Box<dyn FnMut(bool) -> bool>,
    ) -> Self {
        Button {
            pos,
            caption,
            input_id,
            pressed: false,
            dirty: true,
            handler,
        }
    }
}

impl Drawable for Button<'_> {
    fn render(&mut self, drawing: &mut impl Drawing<BinaryColor>) -> (Point, Size) {
        let render = Font6x12::render_str(&self.caption)
            .fill(Some(if self.pressed {
                BinaryColor::On
            } else {
                BinaryColor::Off
            }))
            .stroke(Some(if self.pressed {
                BinaryColor::Off
            } else {
                BinaryColor::On
            }))
            .translate(self.pos);
        drawing.draw(render);
        self.dirty = false;

        (self.pos, render.size())
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl<'a> InputConsumer for Button<'a> {
    fn input_reset(&mut self) {
        self.dirty = true;
    }

    fn input_update(&mut self, input_id: InputId, value: Value) {
        if let Value::Bool(value) = value {
            if input_id == self.input_id && value != self.pressed {
                if (self.handler)(value) {
                    self.pressed = value;
                    self.dirty = true;
                }
            }
        }
    }
}
