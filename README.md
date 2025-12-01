## DIY Gamepad

DIY Gamepad is a custom USB gamepad project, split into two main parts:

- **Firmware**: code running on the microcontroller (in `firmware`) exposing buttons, axes and protocol.
- **Bridge**: a Windows host application (in `bridge`, Rust) that talks to the device and presents a virtual gamepad to the OS.

### Features

- **Fully DIY**: tweak firmware, mapping and protocol to fit your own hardware.
- **Virtual controller**: exposes a standard gamepad to games using the ViGEm driver stack.
- **Modular design**: firmware and bridge are decoupled so you can swap hardware or host side independently.

### Status

This project is experimental and intended for tinkerers. Expect to read the source, adjust pin mappings, and adapt to your specific hardware.


