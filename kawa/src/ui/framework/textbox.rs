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

use super::*;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::str;
use embedded_graphics::{fonts::Font6x12, prelude::*};

pub struct Content {
    pub bytes: Vec<u8>,
    pub is_dirty: bool,
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
    fn render(&mut self, drawing: &mut impl Drawing<BinaryColor>) -> (Point, Size) {
        let content = &mut self.content.borrow_mut();
        content.is_dirty = false;

        let render = Font6x12::render_str(&str::from_utf8(&content.bytes).unwrap())
            .fill(Some(if self.highlight {
                BinaryColor::On
            } else {
                BinaryColor::Off
            }))
            .stroke(Some(if self.highlight {
                BinaryColor::Off
            } else {
                BinaryColor::On
            }))
            .translate(self.pos);
        drawing.draw(render);

        (self.pos, render.size())
    }

    fn is_dirty(&self) -> bool {
        self.content.borrow().is_dirty
    }
}
