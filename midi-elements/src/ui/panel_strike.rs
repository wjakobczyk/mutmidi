use super::framework::*;

use crate::elements_handlers::*;
use crate::{InputDeviceId, PanelId, APP};

use alloc::boxed::Box;
use heapless::consts::U8;
use heapless::Vec;

fn setup_knobs<'a>() -> Vec<Knob<'a>, U8> {
    let mut knobs = Vec::<_, U8>::new();

    knobs
        .push(Knob::new(
            Point::new(0, 40),
            "Lvl",
            InputDeviceId::Knob1 as InputId,
            create_knob_handler(Param::ExcStrikeLevel),
        ))
        .unwrap();
    knobs
        .push(Knob::new(
            Point::new(32, 40),
            "Tmbr",
            InputDeviceId::Knob2 as InputId,
            create_knob_handler(Param::ExcStrikeTimbre),
        ))
        .unwrap();
    knobs
        .push(Knob::new(
            Point::new(96, 40),
            "Mllt",
            InputDeviceId::Knob4 as InputId,
            create_knob_handler(Param::ExcStrikeMeta),
        ))
        .unwrap();

    knobs
}

fn setup_buttons<'a>() -> Vec<Button<'a>, U8> {
    let mut buttons = Vec::<_, U8>::new();

    buttons
        .push(Button::new(
            Point::new(0, 0),
            " Bow",
            InputDeviceId::Button1 as InputId,
            Box::new(|_value: bool| {
                unsafe {
                    (*APP).change_panel(&mut *APP, PanelId::PanelBow);
                }
                true
            }),
        ))
        .unwrap();
    buttons
        .push(Button::new(
            Point::new(26, 0),
            "*Str",
            InputDeviceId::Button2 as InputId,
            Box::new(|value: bool| {
                unsafe {
                    (*APP).trigger_note(value);
                }
                true
            }),
        ))
        .unwrap();
    buttons
        .push(Button::new(
            Point::new(51, 0),
            "P3",
            InputDeviceId::Button3 as InputId,
            Box::new(|_value: bool| true),
        ))
        .unwrap();
    buttons
        .push(Button::new(
            Point::new(77, 0),
            "P4",
            InputDeviceId::Button4 as InputId,
            Box::new(|_value: bool| true),
        ))
        .unwrap();
    buttons
        .push(Button::new(
            Point::new(102, 0),
            "P5",
            InputDeviceId::Button5 as InputId,
            Box::new(|_value: bool| true),
        ))
        .unwrap();

    buttons
}

pub fn setup<'a>() -> (Vec<Button<'a>, U8>, Vec<Knob<'a>, U8>) {
    (setup_buttons(), setup_knobs())
}
