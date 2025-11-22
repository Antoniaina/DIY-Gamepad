#include "joystick.h"

Joystick::Joystick(uint8_t axisX, uint8_t axisY, uint8_t pinBtn) 
    : axisX(axisX), axisY(axisY) , pinBtn(pinBtn) {}

void Joystick::begin() {
    pinMode(pinBtn, INPUT);
}

void Joystick::update() {
    xValue = analogRead(axisX);
    yValue = analogRead(axisY);
    btnPressed = (digitalRead(pinBtn) == HIGH) ;
}