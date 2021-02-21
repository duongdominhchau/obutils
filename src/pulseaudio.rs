use std::process::Command;

#[derive(Default, Copy, Clone)]
pub struct SinkState {
    pub muted: bool,
    pub volume: u8,
}

pub fn get_sink_state() -> SinkState {
    let raw = String::from_utf8(
        Command::new("pactl")
            .args(vec!["list", "sinks"])
            .output()
            .expect("Read pactl output")
            .stdout,
    )
    .expect("Decode pactl output");
    let index = raw.find("Mute: ").expect("Read mute state");
    let slice = &raw[index + 6..];
    let mut state = SinkState::default();
    if slice.starts_with("yes") {
        state.muted = true;
    } else if slice.starts_with("no") {
        state.muted = false;
    } else {
        panic!(
            "Unknown value for mute state: {}",
            slice.lines().next().unwrap()
        );
    }
    let index = slice.find('%').expect("Seek forward to the percent sign");
    // We don't need to keep the percent size
    let slice = &slice[..index];
    let index = slice.rfind(' ').expect("Seek backward to the space");
    // We also skip the space
    let slice = &slice[index + 1..];
    // Now the slice wraps the number we need
    state.volume = slice.parse::<u8>().expect("Parse volume");
    state
}
