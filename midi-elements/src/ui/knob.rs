use super::framework::*;
use embedded_graphics::{fonts::Font6x12, prelude::*};
use numtoa::NumToA;

#[derive(Debug)]
pub struct Knob {
    pub pos: Point,
    input_id: InputId,
    value: i8,
    dirty: bool,
}

impl Knob {
    pub fn new(pos: Point, input_id: InputId) -> Self {
        Knob {
            pos,
            input_id,
            value: 0,
            dirty: true,
        }
    }
}

impl Drawable for Knob {
    fn render(&mut self, drawing: &mut impl Drawing<BinaryColor>) -> (Point, Size) {
        let mut buffer = [0u8; 5];
        let render = Font6x12::render_str(self.value.numtoa_str(10, &mut buffer))
            .fill(Some(BinaryColor::Off))
            .stroke(Some(BinaryColor::On))
            .translate(self.pos);
        drawing.draw(render);
        self.dirty = false;

        (self.pos, render.size())
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl InputConsumer for Knob {
    fn input_update(&mut self, input_id: InputId, value: Value) {
        if let Value::Int(value) = value {
            if input_id == self.input_id && value != self.value {
                self.value = value;
                self.dirty = true;
            }
        }
    }
}
