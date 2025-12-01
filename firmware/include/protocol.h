#pragma once
#include <stdint.h>
#include "joystick.h"

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

void sendPacket(const Joystick& joy1, const Joystick& joy2) ;