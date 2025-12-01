#[repr(C, packed)]
#[derive(Debug)]
pub struct Packet {
    pub header: u8,
    pub j1x: u16,
    pub j1y: u16,
    pub j1b: u8,
    pub j2x: u16,
    pub j2y: u16,
    pub j2b: u8,
    checksum: u8,
}


impl Packet {
    pub const SIZE: usize = 12;

    pub fn parse(buf: &[u8]) -> Option<Self> {
        if (buf.len() != Self::SIZE) || (buf[0] != 0xAAu8) { return None; }

        let checksum: u8 = buf[..Self::SIZE - 1]
            .iter()
            .fold(0u16, |acc, &b| acc + b as u16) as u8;

        if checksum != buf[Self::SIZE - 1] {
            eprintln!(
                "Checksum error: got {:02X}, expected {:02X}",
                buf[Self::SIZE - 1], checksum
            );
            return None;
        }

        Some(Packet {
            header: buf[0],
            j1x: u16::from_le_bytes([buf[1], buf[2]]),
            j1y: u16::from_le_bytes([buf[3], buf[4]]),
            j1b: buf[5],
            j2x: u16::from_le_bytes([buf[6], buf[7]]),
            j2y: u16::from_le_bytes([buf[8], buf[9]]),
            j2b: buf[10],
            checksum: buf[11],
        })
    }
}