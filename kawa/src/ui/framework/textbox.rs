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
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::cmp::max;
use core::str;
use embedded_graphics::{
    drawable::Drawable as EmbeddedDrawable, fonts::Font6x12, fonts::Text, prelude::*,
    style::TextStyleBuilder,
};

pub struct Content {
    pub bytes: Vec<u8>,
    pub is_dirty: bool,
    pub cursor_pos: i32,
}

pub struct ContentBox {
    pos: Point,
    highlight: bool,
    content: Rc<RefCell<Content>>,
}

pub type TextBox = ContentBox;

// impl core::fmt::Debug for ContentBox {
//     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
//         write!(f, "ContentBox({})", str::from_utf8(&self.content).unwrap())
//     }
// }

impl ContentBox {
    pub fn new(pos: Point, content: Rc<RefCell<Content>>) -> Self {
        ContentBox {
            pos,
            content,
            highlight: false,
        }
    }
}

impl Drawable for ContentBox {
    fn render(&mut self, drawing: &mut impl DrawTarget<BinaryColor>) -> (Point, Size) {
        let content = &mut self.content.borrow_mut();
        content.is_dirty = false;
        let mut size = Size {
            width: 0,
            height: 0,
        };
        let mut pos = self.pos;
        let cursor_pos = content.cursor_pos as usize;

        for (i, byte) in content.bytes.iter_mut().enumerate() {
            let buf: [u8; 1] = [*byte];
            let highlight = self.highlight || i == cursor_pos;
            let style = TextStyleBuilder::new(Font6x12)
                .text_color(if highlight {
                    BinaryColor::Off
                } else {
                    BinaryColor::On
                })
                .background_color(if highlight {
                    BinaryColor::On
                } else {
                    BinaryColor::Off
                })
                .build();
            let text = Text::new(&str::from_utf8(&buf).unwrap(), pos).into_styled(style);
            let _ = text.draw(drawing);
            let text_size = text.size();
            pos.x += text_size.width as i32;
            size.width += text_size.width;
            size.height = max(size.height, text_size.height);
        }

        (self.pos, size)
    }

    fn is_dirty(&self) -> bool {
        self.content.borrow().is_dirty
    }
}
