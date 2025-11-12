#include <Arduino.h>
#include "joystick.h"

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

  Serial.println(" === DIY Gamepad Ready === ");
}

void loop() {
  joy1.update();
  joy2.update();

  Serial.print("J1 X: "); Serial.print(joy1.getX());
  Serial.print(" Y: "); Serial.print(joy1.getY());
  Serial.print(" Btn: "); Serial.print(joy1.isPressed() ? "Pressed" : "Released");
  
  Serial.print(" | J2 X: "); Serial.print(joy2.getX());
  Serial.print(" Y: "); Serial.print(joy2.getY());
  Serial.print(" Btn: "); Serial.println(joy2.isPressed() ? "Pressed" : "Released");
  
  delay(200);


}
