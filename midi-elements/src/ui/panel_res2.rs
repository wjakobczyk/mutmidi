use super::framework::*;
use super::*;

use crate::elements_handlers::*;
use crate::InputDeviceId;

use heapless::consts::U8;
use heapless::Vec;

fn setup_knobs<'a>() -> Vec<Knob<'a>, U8> {
    let mut knobs = Vec::<_, U8>::new();

    // knobs
    //     .push(Knob::new(
    // Point::new(KNOB_POS_X[0], KNOB_POS_Y),
    //         "TnC",
    //         InputDeviceId::Knob1 as InputId,
    //         create_knob_handler(Param::),
    //     ))
    //     .unwrap();
    // knobs
    //     .push(Knob::new(
    // Point::new(KNOB_POS_X[1], KNOB_POS_Y),
    //         "TnF",
    //         InputDeviceId::Knob2 as InputId,
    //         create_knob_handler(Param::ResBrightness),
    //     ))
    //     .unwrap();
    knobs
        .push(Knob::new(
            Point::new(KNOB_POS_X[2], KNOB_POS_Y),
            "Spc",
            InputDeviceId::Knob3 as InputId,
            create_knob_handler(Param::Space),
        ))
        .unwrap();

    knobs
}

pub fn setup<'a>() -> (Vec<Button<'a>, U8>, Vec<Knob<'a>, U8>) {
    (super::panel_res1::setup_resonator_buttons(1), setup_knobs())
}
