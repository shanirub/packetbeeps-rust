use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;
use std::thread;
use std::time::Duration;

pub struct Buzzer {
    pin: OutputPin, // Represents the GPIO pin controlling the buzzer
    frequencies: [u64; 8], // Frequencies for tones in Hz
}

impl Buzzer {
    pub fn new(pin_number: u8) -> Result<Self, Box<dyn Error>> {
        let gpio = Gpio::new()?;
        let pin = gpio.get(pin_number)?.into_output();
        let frequencies = [261, 294, 329, 349, 392, 440, 466, 523]; // Example tones: C4 to C5
        Ok(Self { pin, frequencies })
    }

    pub fn play_tone(&mut self, tone_index: usize, duration_ms: u64) {
        if tone_index >= self.frequencies.len() {
            println!("Invalid tone index: {}", tone_index);
            return;
        }

        let frequency = self.frequencies[tone_index];
        let period = Duration::from_secs_f64(1.0 / frequency as f64);
        let half_period = period / 2;

        let cycles = duration_ms * frequency / 1000;
        for _ in 0..cycles {
            self.pin.set_high();
            thread::sleep(half_period);
            self.pin.set_low();
            thread::sleep(half_period);
        }
    }

    pub fn test_buzzer(&mut self) {
        println!("Testing buzzer...");
        for (i, _) in self.frequencies.clone().iter().enumerate() {
            println!("Playing tone {}", i);
            self.play_tone(i, 500); // Play each tone for 500ms
            thread::sleep(Duration::from_millis(200)); // Short delay between tones
        }
        println!("Buzzer test completed.");
    }
}
