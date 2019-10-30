pub use embedded_graphics::{
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
    Drawing,
};

#[derive(Copy, Clone)]
pub enum Value {
    Bool(bool),
    Int(i8),
}

pub type InputId = u32;

pub trait Drawable {
    fn render(&mut self, drawing: &mut impl Drawing<BinaryColor>) -> (Point, Size);
}

pub trait InputConsumer {
    fn input_update(&mut self, input_id: InputId, value: Value);
}
