mod packet;
mod controls;
mod mapper;

use packet::Packet;
use controls::ControllerState;
use mapper::map_to_xgamepad;

use vigem_client::{Client, TargetId, Xbox360Wired};
use std::io::Read;

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

    let client = Client::connect().expect("Failed to connect to ViGEM");
    let mut target = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);

    target.plugin().expect("Failed to  plugin virtual controller");
    target.wait_ready().unwrap();

    let mut header = [0u8; 1];

    loop {
        if port.read_exact(&mut header).is_ok() && header[0] == 0xAAu8 {
            let mut buffer = [0u8; Packet::SIZE];
            buffer[0] = 0xAA;

            if port.read_exact(&mut buffer[1..]).is_ok() {
                if let Some(payload) = Packet::parse(&buffer) {
                    println!("{:#?}", payload);
                    let state = ControllerState::from_packet(&payload);
                    let pad = map_to_xgamepad(&state);
                    target.update(&pad).unwrap();
                }
                else {
                    println!("Error reading packet body");
                }
            }
        }
    }
        
}