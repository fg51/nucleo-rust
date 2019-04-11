extern crate stm32f4xx_hal;

use stm32f4xx_hal::{
    gpio::{gpioa::PA5, Output, PushPull},
    hal::digital::OutputPin,
};

pub trait LED {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn blinky(&mut self);
}

pub struct LED2 {
    pin: PA5<Output<PushPull>>,
    is_turn_on: bool,
}

impl LED2 {
    pub fn new(pin: PA5<Output<PushPull>>) -> Self {
        Self {
            pin,
            is_turn_on: false,
        }
    }
}

impl LED for LED2 {
    fn turn_on(&mut self) {
        self.is_turn_on = true;
        self.pin.set_high();
    }

    fn turn_off(&mut self) {
        self.is_turn_on = false;
        self.pin.set_low();
    }

    fn blinky(&mut self) {
        self.is_turn_on = !self.is_turn_on;
        if self.is_turn_on == true {
            self.pin.set_high();
        } else {
            self.pin.set_low();
        }
    }
}
