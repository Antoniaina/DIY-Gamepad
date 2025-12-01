use vigem_client::{ XGamepad, XButtons };
use crate::controls::ControllerState;

pub fn map_to_xgamepad(state: &ControllerState) -> XGamepad {
    let mut pad = XGamepad::default();

    pad.thumb_lx = -state.j2x;
    pad.thumb_ly = state.j2y;
    pad.thumb_rx = -state.j1x;
    pad.thumb_ry = -state.j1y;

    let mut mask: u16 = 0;
    if state.j1b != 1  { mask |= XButtons::A; }
    if state.j2b != 1  { mask |= XButtons::B; }

    pad.buttons = XButtons::from(mask);

    pad
}