use super::framework::*;
use alloc::boxed::Box;
use embedded_graphics::{fonts::Font6x12, prelude::*};
use numtoa::NumToA;

static DELTA_THRESHOLD: i32 = 10;

pub struct Knob {
    pub pos: Point,
    input_id: InputId,
    value: u8,
    last_input_value: i32,
    dirty: bool,
    handler: Box<dyn FnMut(i8) -> u8>,
}

impl core::fmt::Debug for Knob {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Knob({})", self.input_id)
    }
}

impl Knob {
    pub fn new(pos: Point, input_id: InputId, mut handler: Box<dyn FnMut(i8) -> u8>) -> Self {
        Knob {
            pos,
            input_id,
            value: (handler)(0),
            last_input_value: 0,
            dirty: true,
            handler,
        }
    }
}

impl Drawable for Knob {
    fn render(&mut self, drawing: &mut impl Drawing<BinaryColor>) -> (Point, Size) {
        let mut buffer = [0u8; 3];
        self.value.numtoa(10, &mut buffer);
        if buffer[buffer.len() - 2] == 0 {
            buffer[buffer.len() - 2] = b' ';
        }
        let text = &buffer[buffer.len() - 2..buffer.len()];
        let render = Font6x12::render_str(unsafe { core::str::from_utf8_unchecked(text) })
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
        if let Value::Int(input_value) = value {
            if input_id == self.input_id {
                let delta = input_value - self.last_input_value;
                if delta != 0 && delta < DELTA_THRESHOLD && delta > -DELTA_THRESHOLD {
                    self.value = (self.handler)(delta as i8);
                    self.dirty = true;
                }
                self.last_input_value = input_value;
            }
        }
    }
}
