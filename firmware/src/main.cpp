#include <Arduino.h>
#include "joystick.h"

#define J1_X A1
#define J1_Y A0
#define J1_BTN 2

#define J2_X A3
#define J2_Y A2
#define J2_BTN 3

struct __attribute__((packed)) Packet {
  uint8_t header;
  uint16_t j1x;
  uint16_t j1y;
  uint8_t j1b;
  uint16_t j2x;
  uint16_t j2y;
  uint8_t j2b;
  uint8_t checksum;
};

Joystick joy1(J1_X, J1_Y, J1_BTN);
Joystick joy2(J2_X, J2_Y, J2_BTN);

void sendPacket() {
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
  for (int i=0; i< sizeof(Packet) -1; i++) sum += raw[i];

  payload.checksum = sum;

  Serial.write(raw, sizeof(Packet));
}

void setup() {
  Serial.begin(115200);
  joy1.begin();
  joy2.begin();
}

void loop() {
  joy1.update();
  joy2.update();
  sendPacket();
  delay(200);
}
