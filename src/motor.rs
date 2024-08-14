use rppal::gpio::Gpio;
use rppal::gpio::Pin;
use rppal::gpio::OutputPin;
use rppal::pwm::Pwm;
use rppal::pwm::Polarity;
use rppal::pwm::Channel;

pub struct Motor {
    pwm_pin: OutputPin,
    direction_pin_0: OutputPin,
    direction_pin_1: OutputPin,
    power: i8
}

impl Motor {
    pub fn new(gpio: &Gpio, pwm_pin: u8, direction_pin_0: u8, direction_pin_1: u8) -> Motor {
        let pwm_pin = gpio.get(pwm_pin).unwrap().into_output_low();//Pwm::with_frequency(pwm_channel, 100.0, 0.0, Polarity::Normal, true).unwrap();
        let direction_pin_0 = gpio.get(direction_pin_0).unwrap().into_output_high();
        let direction_pin_1 = gpio.get(direction_pin_1).unwrap().into_output_low();
        return Motor {pwm_pin, direction_pin_0, direction_pin_1, power: 0};
    }

    pub fn set_power(self: &mut Motor, power: i8) {
        self.power = power;
        let magnitude = power.abs();
        if power > 0 {
            self.direction_pin_0.set_high();
            self.direction_pin_1.set_low();
        } else {
            self.direction_pin_1.set_high();
            self.direction_pin_0.set_low();
        }
        let set_pwm_error = self.pwm_pin.set_pwm_frequency(200.0, (magnitude as f64)/128.0);
        match set_pwm_error {
            Ok(_) => {},
            Err(_) => panic!(),
        }
    }
}
