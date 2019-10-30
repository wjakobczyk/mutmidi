use super::framework::*;
use heapless::consts::U8;
use heapless::Vec;

use super::{button::Button, knob::Knob};

pub struct Panel<'a> {
    buttons: Vec<Button<'a>, U8>,
    knobs: Vec<Knob, U8>,
}

impl<'a> Panel<'a> {
    pub fn new(buttons: Vec<Button<'a>, U8>, knobs: Vec<Knob, U8>) -> Self {
        Panel { buttons, knobs }
    }
}

fn extend_rect_to_cover(pos: &mut Point, size: &mut Size, cover_pos: &Point, cover_size: &Size) {
    if cover_pos.x < pos.x {
        pos.x = cover_pos.x;
    }
    if cover_pos.y < pos.y {
        pos.y = cover_pos.y;
    }
    if cover_pos.x + cover_size.width as i32 > pos.x + size.width as i32 {
        size.width = cover_pos.x as u32 + cover_size.width - pos.x as u32;
    }
    if cover_pos.y + cover_size.height as i32 > pos.y + size.height as i32 {
        size.height = cover_pos.y as u32 + cover_size.height - pos.y as u32;
    }
}

impl Drawable for Panel<'_> {
    fn render(&mut self, drawing: &mut impl Drawing<BinaryColor>) -> (Point, Size) {
        let mut panel_pos = Point {
            x: core::i32::MAX,
            y: core::i32::MAX,
        };
        let mut panel_size = Size {
            width: 0,
            height: 0,
        };

        for component in self.buttons.iter_mut() {
            let (pos, size) = component.render(drawing);
            extend_rect_to_cover(&mut panel_pos, &mut panel_size, &pos, &size);
        }
        for component in self.knobs.iter_mut() {
            let (pos, size) = component.render(drawing);
            extend_rect_to_cover(&mut panel_pos, &mut panel_size, &pos, &size);
        }

        (panel_pos, panel_size)
    }
}

impl<'a> InputConsumer for Panel<'a> {
    fn input_update(&mut self, input_id: InputId, value: Value) {
        for component in self.buttons.iter_mut() {
            component.input_update(input_id, value);
        }
        for component in self.knobs.iter_mut() {
            component.input_update(input_id, value);
        }
    }
}
