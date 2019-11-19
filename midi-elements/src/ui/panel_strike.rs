use super::framework::*;
use super::*;

use crate::elements_handlers::*;
use crate::InputDeviceId;

use alloc::vec;
use alloc::vec::Vec;

fn setup_knobs<'a>() -> Vec<Knob<'a>> {
    vec![
        Knob::new(
            Point::new(KNOB_POS_X[0], KNOB_POS_Y),
            "Lvl",
            InputDeviceId::Knob1 as InputId,
            create_knob_handler(Param::ExcStrikeLevel),
        ),
        Knob::new(
            Point::new(KNOB_POS_X[1], KNOB_POS_Y),
            "Tmbr",
            InputDeviceId::Knob2 as InputId,
            create_knob_handler(Param::ExcStrikeTimbre),
        ),
        Knob::new(
            Point::new(KNOB_POS_X[3], KNOB_POS_Y),
            "Mllt",
            InputDeviceId::Knob4 as InputId,
            create_knob_handler(Param::ExcStrikeMeta),
        ),
    ]
}

pub fn setup<'a>() -> (Vec<Button<'a>>, Vec<Knob<'a>>) {
    (super::panel_bow::setup_exciter_buttons(2), setup_knobs())
}
