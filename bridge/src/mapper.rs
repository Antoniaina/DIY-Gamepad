use vigem_client::{ XGamepad, XButtons };
use crate::controls::ControllerState;

pub fn map_to_xgamepad(state: &ControllerState) -> XGamepad {
    let mut pad = XGamepad::default();

    pad.thumb_lx = state.j1x;
    pad.thumb_ly = state.j1y;
    pad.thumb_rx = state.j2x;
    pad.thumb_ry = state.j2y;

    let mut mask: u16 = 0;
    if state.j1b != 0  { mask |= XButtons::A; }
    if state.j2b != 0  { mask |= XButtons::B; }

    pad.buttons = XButtons::from(mask);

    pad
}