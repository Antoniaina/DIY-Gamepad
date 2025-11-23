use std::io::Read;

#[repr(C, packed)]
#[derive(Debug)]
struct Packet {
    header: u8,
    j1x: u16,
    j1y: u16,
    j1b: u8,
    j2x: u16,
    j2y: u16,
    j2b: u8,
    checksum: u8,
}

const PACKET_SIZE: usize = 12; 

fn main() {
    let port_name = "COM4";
    let baud_rate = 115200;

    let mut port = serialport::new(port_name, baud_rate)
        .timeout(std::time::Duration::from_millis(200))
        .open()
        .map_err(|e| {
            eprintln!("Failed opening port: {}", e);
        })
        .unwrap();

    let mut sync_buffer = [0u8; 1];

    loop {
        match port.read_exact(&mut sync_buffer) {
            Ok(_) => {
                if sync_buffer[0] == 0xAA {
                    let mut packet_buf = vec![0xAAu8];
                    let mut rest_buf = vec![0u8; PACKET_SIZE - 1];

                    match port.read_exact(&mut rest_buf) {
                        Ok(_) => {
                            packet_buf.extend_from_slice(&rest_buf);
                            
                            if packet_buf.len() == PACKET_SIZE {
                                let sum: u8 = packet_buf[..PACKET_SIZE - 1]
                                    .iter()
                                    .fold(0u16, |acc, &b| acc + b as u16) as u8;

                                if sum != packet_buf[PACKET_SIZE - 1] {
                                    eprintln!(
                                        "Checksum error: got {:02X}, expected {:02X}",
                                        packet_buf[PACKET_SIZE - 1], sum
                                    );
                                    continue;
                                }

                                let payload = Packet {
                                    header: packet_buf[0],
                                    j1x: u16::from_le_bytes([packet_buf[1], packet_buf[2]]),
                                    j1y: u16::from_le_bytes([packet_buf[3], packet_buf[4]]),
                                    j1b: packet_buf[5],
                                    j2x: u16::from_le_bytes([packet_buf[6], packet_buf[7]]),
                                    j2y: u16::from_le_bytes([packet_buf[8], packet_buf[9]]),
                                    j2b: packet_buf[10],
                                    checksum: packet_buf[11],
                                };

                                println!("{:#?}", payload);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading packet body: {}", e);
                        }
                    }
                } else {
                    eprintln!("Bad header: {:02X}, searching for sync", sync_buffer[0]);
                }
            }
            Err(e) => {
                eprintln!("Error reading: {}", e);
            }
        }
    }
}