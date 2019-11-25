use super::framework::*;
use super::*;

use crate::elements_handlers::*;
use crate::InputDeviceId;

use alloc::vec;
use alloc::vec::Vec;

fn setup_knobs<'a>() -> Vec<Knob<'a>> {
    vec![Knob::new(
        Point::new(KNOB_POS_X[2], KNOB_POS_Y),
        "Spc",
        InputDeviceId::Knob3 as InputId,
        create_knob_handler(Param::Space),
    )]
}

pub fn setup<'a>() -> (Vec<Button<'a>>, Vec<Knob<'a>>) {
    (super::panel_res::setup_resonator_buttons(1), setup_knobs())
}
