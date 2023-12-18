use rust_gpiozero::*;

/// Wait time for thread
pub const IDLE: u64 = 500;

/// The Gpio Pins represented
#[derive(Debug)]
pub enum GpioPins {
    Power = 20,
    Up = 2,
    Left = 3,
    Right = 4,
    Down = 6,
    PointGain = 19,
    PointLoss = 26,
    GameOver = 13,
    PowerOff = 5,
    None = 18,
}

impl GpioPins {
    /// Default waits
    pub fn default_idle() {
        std::thread::sleep(std::time::Duration::from_millis(IDLE));
    }

    /// Up, Down, Left, Right pins
    pub fn movements(gpio: GpioPins) {
        let mut button = match gpio {
            GpioPins::Up => Button::new(GpioPins::Up as u64),
            GpioPins::Down => Button::new(GpioPins::Down as u64),
            GpioPins::Left => Button::new(GpioPins::Left as u64),
            GpioPins::Right => Button::new(GpioPins::Right as u64),
            _ => Button::new(GpioPins::None as u64),
        };

        button.wait_for_release();
    }

    /// PointGain and PointLoss Pins
    pub fn scoring(gpio: GpioPins) {
        let mut led = match gpio {
            GpioPins::PointGain => LED::new(GpioPins::PointGain as u64),
            GpioPins::PointLoss => LED::new(GpioPins::PointLoss as u64),
            _ => LED::new(GpioPins::None as u64),
        };

        led.on();
        GpioPins::default_idle();
        led.off();
    }

    /// Turns on power, also in join with app
    pub fn activate(gpio: GpioPins) {
        let mut led = LED::new(GpioPins::Power as u64);

        led.on();
    }

    /// Turns off power, in join with app
    pub fn deactivate(gpio: GpioPins) {
        let mut led = LED::new(GpioPins::Power as u64);

        LED::new(GpioPins::PointGain as u64).off();
        LED::new(GpioPins::PointLoss as u64).off();
        led.off();
    }

    /// GameOver Pins
    pub fn last(gpio: GpioPins) {
        let mut buzzer = Buzzer::new(GpioPins::GameOver as u64);

        buzzer.on();
        GpioPins::default_idle();
        buzzer.off();
    }

    pub fn power_off(gpio: GpioPins) {
        let mut button = Button::new(GpioPins::PowerOff as u64);

        button.wait_for_release();

        println!("Off");
    }
}
