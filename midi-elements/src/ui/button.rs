use super::framework::Widget;
use embedded_graphics::{fonts::Font6x12, pixelcolor::BinaryColor, prelude::*, Drawing};

pub struct Button<'a> {
    pub pos: Point,
    caption: &'a str,
}

impl<'a> Button<'a> {
    pub fn new(pos: Point, caption: &'a str) -> Self {
        Button { pos, caption }
    }
}

impl<'a> Widget for Button<'a> {
    fn render(&self, drawing: &mut impl Drawing<BinaryColor>) -> (Point, Size) {
        let render = Font6x12::render_str(&self.caption)
            .fill(Some(BinaryColor::Off))
            .stroke(Some(BinaryColor::On))
            .translate(self.pos);
        drawing.draw(render);
        (self.pos, render.size())
    }
}
