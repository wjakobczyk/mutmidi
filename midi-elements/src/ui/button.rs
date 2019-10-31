use super::framework::*;
use embedded_graphics::{fonts::Font6x12, prelude::*};

#[derive(Debug)]
pub struct Button<'a> {
    pub pos: Point,
    caption: &'a str,
    input_id: InputId,
    pressed: bool,
    dirty: bool,
}

impl<'a> Button<'a> {
    pub fn new(pos: Point, caption: &'a str, input_id: InputId) -> Self {
        Button {
            pos,
            caption,
            input_id,
            pressed: false,
            dirty: true,
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
    fn input_update(&mut self, input_id: InputId, value: Value) {
        if let Value::Bool(value) = value {
            if input_id == self.input_id && value != self.pressed {
                self.pressed = value;
                self.dirty = true;
            }
        }
    }
}
