# DIY-Gamepad 🎮

A simple DIY gamepad with **two analog joysticks** and buttons using an Arduino Uno.

## Overview
This project reads two joysticks (X/Y axes) and their buttons, then sends the values to the Serial Monitor.  
It can later be extended to act as a PC game controller or control other peripherals.

## Wiring
| Joystick | Arduino Uno |
|-----------|-------------|
| VCC       | 5V          |
| GND       | GND         |
| Btn J1    | D2          |
| Btn J2    | D3          |
| Y J1      | A0          |
| X J1      | A1          |
| Y J2      | A2          |
| X J2      | A3          |

### Note about buttons
The buttons are wired between **5V and the digital pin**, so we do **not use INPUT_PULLUP** here.  
Instead, the pin is set to **INPUT**, and a HIGH reading means the button is pressed.  
This matches the existing PCB wiring.

## Usage
1. Open PlatformIO, select the **Uno environment**, and compile.
2. Upload the code to the Arduino.
3. Open Serial Monitor at 115200 baud to see joystick and button values.
