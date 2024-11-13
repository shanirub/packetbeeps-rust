// Miscellaneous utility functions like platform detection.

use std::fs;

pub fn is_raspberry_pi() -> bool {
    if cfg!(target_os = "linux") {
        if let Ok(model) = fs::read_to_string("/proc/device-tree/model") {
            return model.to_lowercase().contains("raspberry pi");
        }
    }
    false
}
