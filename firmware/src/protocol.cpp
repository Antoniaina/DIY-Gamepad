#include "protocol.h"
#include <Arduino.h>

void sendPacket(const Joystick& joy1, const Joystick& joy2) {
  Packet payload;

  payload.header = 0xAA;
  payload.j1x = joy1.getX();
  payload.j1y = joy1.getY();
  payload.j1b = joy1.isPressed() ? 1 : 0;
  payload.j2x = joy2.getX();
  payload.j2y = joy2.getY();
  payload.j2b = joy2.isPressed() ? 1 : 0;

  uint8_t *raw = (uint8_t *)&payload;

  uint8_t sum = 0;
  for (size_t i=0; i < sizeof(Packet) - 1; ++i) sum += raw[i];

  payload.checksum = sum;

  Serial.write(raw, sizeof(Packet));
}