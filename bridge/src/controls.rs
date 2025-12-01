use crate::packet::Packet;

pub struct ControllerState {
    pub j1x: i16,
    pub j1y: i16,
    pub j1b: i16,
    pub j2x: i16,
    pub j2y: i16,
    pub j2b: i16,
}

fn map_axis(val: u16) -> i16 {
    let centered = (val as f32 / 1023.0) * 2.0 - 1.0;
    ((centered * 32767.0) as i16).clamp(-32768, 32767)
}

impl ControllerState {
    pub fn from_packet(pkt: &Packet ) -> Self {
        Self {
            j1x: map_axis(pkt.j1x),
            j1y: map_axis(pkt.j1y),
            j1b: (pkt.j1b != 0) as i16,
            j2x: map_axis(pkt.j2x),
            j2y: map_axis(pkt.j2y),
            j2b: (pkt.j2b != 0) as i16,
        }
    }
}