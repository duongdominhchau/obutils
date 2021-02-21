use crate::util;

fn path_of(name: &str) -> String {
    format!("/sys/class/backlight/intel_backlight/{}", name)
}

/// Brightness in percent
pub fn get_brightness() -> u8 {
    let current = util::read_unsigned(&path_of("brightness")).expect("Read brightness");
    let max = util::read_unsigned(&path_of("max_brightness")).expect("Read max brightness");
    let percent = (current as f64 / max as f64 * 100f64).round();
    percent as u8
}
