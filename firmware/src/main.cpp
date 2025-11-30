#include <Arduino.h>
#include "joystick.h"
#include "protocol.h"

#define J1_X A1
#define J1_Y A0
#define J1_BTN 2

#define J2_X A3
#define J2_Y A2
#define J2_BTN 3


Joystick joy1(J1_X, J1_Y, J1_BTN);
Joystick joy2(J2_X, J2_Y, J2_BTN);

void setup() {
  Serial.begin(115200);
  joy1.begin();
  joy2.begin();
}

void loop() {
  joy1.update();
  joy2.update();
  sendPacket(joy1, joy2);
  delay(200);
}
