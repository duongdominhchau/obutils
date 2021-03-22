use std::fs::{read_dir, read_to_string};

#[derive(Default, Debug, Copy, Clone)]
pub struct LedState {
    pub caps_lock: bool,
    pub num_lock: bool,
}

/// Get the input device ID
pub fn get_input_id() -> Option<u8> {
    for path in read_dir("/sys/class/leds").unwrap() {
        let path = path.unwrap();
        let path = path.file_name().into_string().unwrap();
        let head = "input";
        let tail = "::capslock";
        if path.starts_with("input") && path.ends_with("::capslock") {
            let slice = &path[head.len()..path.len() - tail.len()];
            return slice.parse::<u8>().ok();
        }
    }
    None
}

/// Read current state of a specific led
fn read_state(input_id: u8, led_name: &str) -> Option<bool> {
    let path = format!("/sys/class/leds/input{}::{}/brightness", input_id, led_name);
    Some(read_to_string(path).ok()?.trim() == "1")
}

/// Get current state of NumLock and CapsLock
pub fn get_leds_state(id: u8) -> Option<LedState> {
    Some(LedState {
        caps_lock: read_state(id, "capslock")?,
        num_lock: read_state(id, "numlock")?,
    })
}
