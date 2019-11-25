use super::framework::*;
use super::*;

use crate::elements_handlers::*;
use crate::{InputDeviceId, PanelId, APP};

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

fn setup_knobs<'a>() -> Vec<Knob<'a>> {
    vec![
        Knob::new(
            Point::new(KNOB_POS_X[0], KNOB_POS_Y),
            "Geo",
            InputDeviceId::Knob1 as InputId,
            create_knob_handler(Param::ResGeometry),
        ),
        Knob::new(
            Point::new(KNOB_POS_X[1], KNOB_POS_Y),
            "Bri",
            InputDeviceId::Knob2 as InputId,
            create_knob_handler(Param::ResBrightness),
        ),
        Knob::new(
            Point::new(KNOB_POS_X[2], KNOB_POS_Y),
            "Damp",
            InputDeviceId::Knob3 as InputId,
            create_knob_handler(Param::ResDamping),
        ),
        Knob::new(
            Point::new(KNOB_POS_X[3], KNOB_POS_Y),
            "Pos",
            InputDeviceId::Knob4 as InputId,
            create_knob_handler(Param::ResPosition),
        ),
    ]
}

pub fn setup_resonator_buttons<'a>(active: i8) -> Vec<Button<'a>> {
    vec![
        Button::new(
            Point::new(BUTTON_POS_X[0], BUTTON_POS_Y),
            if active == 0 { "*Res1" } else { " Res1" },
            InputDeviceId::Button1 as InputId,
            Box::new(|value: bool| {
                unsafe {
                    (*APP).change_panel(&mut *APP, PanelId::PanelRes);
                    (*APP).trigger_note(value);
                }
                true
            }),
        ),
        Button::new(
            Point::new(BUTTON_POS_X[1], BUTTON_POS_Y),
            if active == 1 { "*Res2" } else { " Res2" },
            InputDeviceId::Button2 as InputId,
            Box::new(|value: bool| {
                unsafe {
                    (*APP).change_panel(&mut *APP, PanelId::PanelOutput);
                    (*APP).trigger_note(value);
                }
                true
            }),
        ),
        Button::new(
            Point::new(BUTTON_POS_X[3], BUTTON_POS_Y),
            "Exc",
            InputDeviceId::Button4 as InputId,
            Box::new(|value: bool| {
                unsafe {
                    (*APP).change_panel(&mut *APP, PanelId::PanelBow);
                    (*APP).trigger_note(value);
                }
                true
            }),
        ),
        Button::new(
            Point::new(BUTTON_POS_X[4], BUTTON_POS_Y),
            "Sys",
            InputDeviceId::Button5 as InputId,
            Box::new(|_value: bool| true),
        ),
    ]
}

pub fn setup<'a>() -> (Vec<Button<'a>>, Vec<Knob<'a>>) {
    (setup_resonator_buttons(0), setup_knobs())
}
