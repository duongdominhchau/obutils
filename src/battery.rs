use std::fs;

#[derive(Debug, Default, Copy, Clone)]
pub struct BatteryInfo {
    pub now: u64,
    pub full: u64,
    pub full_design: u64,
    pub charging: bool,
}

fn read_stat(name: &str) -> u64 {
    fs::read_to_string(format!("/sys/class/power_supply/BAT1/energy_{}", name))
        .unwrap_or_else(|_| panic!("Read `energy_{}`", name))
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("Convert `energy_{}` content to number", name))
}

pub fn get_battery_info() -> BatteryInfo {
    BatteryInfo {
        now: read_stat("now"),
        full: read_stat("full"),
        full_design: read_stat("full_design"),
        charging: fs::read_to_string("/sys/class/power_supply/ACAD/online")
            .expect("Read charging status")
            .trim()
            .eq("1"),
    }
}
