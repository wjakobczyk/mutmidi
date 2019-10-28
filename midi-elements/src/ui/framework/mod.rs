use embedded_graphics::{
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
    Drawing,
};

pub trait Widget {
    fn render(&self, drawing: &mut impl Drawing<BinaryColor>) -> (Point, Size);
}
