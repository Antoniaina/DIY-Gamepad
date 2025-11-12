#pragma once

#include <Arduino.h>

class Joystick {
public:
    Joystick(uint8_t axisX, uint8_t axisY, uint8_t pinBtn);

    void begin();
    void update();

    int getX() const { return xValue; }
    int getY() const { return yValue; }
    bool isPressed() const { return btnPressed; }

private:
    uint8_t axisX, axisY, pinBtn;
    int xValue = 0, yValue = 0;
    bool btnPressed = false;
};