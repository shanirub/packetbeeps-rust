mod buzzer;
mod utils;
mod packet_processing;
mod event_loop;

use buzzer::Buzzer;

fn main() {
    // Detect Raspberry Pi environment
    let is_raspi = utils::is_raspberry_pi(); // Simplified check for Linux (update with detailed detection logic if needed)
    if is_raspi {
        match Buzzer::new(17) {
            Ok(mut buzzer) => {
                buzzer.test_buzzer();
            }
            Err(e) => eprintln!("Failed to initialize buzzer: {}", e),
        }
    } else {
        println!("Buzzer test skipped. Not running on Raspberry Pi.");
    }
}
