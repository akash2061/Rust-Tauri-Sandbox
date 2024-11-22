use std::thread::sleep;
use std::time::Duration;

// Simulated GPIO pin state: either high (true) or low (false)
#[derive(Debug)]
pub struct GpioPin {
    is_on: bool, // Represents the pin state (high = true, low = false)
}

impl GpioPin {
    // Constructor to create a new GPIO pin, initialized to low (false)
    pub fn new() -> GpioPin {
        GpioPin { is_on: false }
    }

    // Set the GPIO pin to high (turn it on)
    pub fn set_high(&mut self) {
        self.is_on = true;
        println!("GPIO pin set HIGH");
    }

    // Set the GPIO pin to low (turn it off)
    pub fn set_low(&mut self) {
        self.is_on = false;
        println!("GPIO pin set LOW");
    }

    // Simulate reading the state of the GPIO pin
    pub fn read(&self) -> bool {
        self.is_on
    }
}

// Simulated GpioController that controls multiple pins
pub struct GpioController {
    pin_a: GpioPin,
    pin_b: GpioPin,
    pin_c: GpioPin,
}

impl GpioController {
    // Constructor to initialize the GPIO controller with three pins
    pub fn new() -> GpioController {
        GpioController {
            pin_a: GpioPin::new(),
            pin_b: GpioPin::new(),
            pin_c: GpioPin::new(),
        }
    }

    // Turn on a specific pin (A, B, or C)
    pub fn turn_on(&mut self, pin: char) {
        match pin {
            'A' => self.pin_a.set_high(),
            'B' => self.pin_b.set_high(),
            'C' => self.pin_c.set_high(),
            _ => println!("Invalid pin selection"),
        }
    }

    // Turn off a specific pin (A, B, or C)
    pub fn turn_off(&mut self, pin: char) {
        match pin {
            'A' => self.pin_a.set_low(),
            'B' => self.pin_b.set_low(),
            'C' => self.pin_c.set_low(),
            _ => println!("Invalid pin selection"),
        }
    }

    // Check the status of a specific pin
    pub fn check_status(&self, pin: char) {
        match pin {
            'A' => println!(
                "Pin A is {}",
                if self.pin_a.read() { "HIGH" } else { "LOW" }
            ),
            'B' => println!(
                "Pin B is {}",
                if self.pin_b.read() { "HIGH" } else { "LOW" }
            ),
            'C' => println!(
                "Pin C is {}",
                if self.pin_c.read() { "HIGH" } else { "LOW" }
            ),
            _ => println!("Invalid pin selection"),
        }
    }
}

fn main() {
    // Create a new GPIO controller
    let mut gpio_controller = GpioController::new();

    // Simulate turning on LED on Pin A
    gpio_controller.turn_on('A');
    gpio_controller.check_status('A'); // Check pin status

    // Simulate a delay (e.g., LED being on for some time)
    println!("Simulating delay...");
    sleep(Duration::from_secs(2));

    // Simulate turning off LED on Pin A
    gpio_controller.turn_off('A');
    gpio_controller.check_status('A'); // Check pin status

    // Simulate turning on LED on Pin B
    gpio_controller.turn_on('B');
    gpio_controller.check_status('B'); // Check pin status

    // Simulate a delay (e.g., LED being on for some time)
    println!("Simulating delay...");
    sleep(Duration::from_secs(2));

    // Simulate turning off LED on Pin B
    gpio_controller.turn_off('B');
    gpio_controller.check_status('B'); // Check pin status
}
