use std::io::Write;
use std::time::Duration;

mod battery;
mod cpu;
mod memory;
mod util;

enum DataUnit {
    Byte(f64),
    KiB(f64),
    MiB(f64),
    GiB(f64),
}

fn format_value(value: f64, unit: &str, width: usize, precision: usize) -> String {
    format!("{0:2$.3$}{1}", value, unit, width, precision)
}

fn humanize(v: DataUnit, width: usize, precision: usize) -> String {
    use crate::DataUnit::*;
    match v {
        Byte(value) => {
            if value < 1000_f64 {
                format_value(value, "B", width, precision)
            } else {
                humanize(KiB(value / 1024_f64), width, precision)
            }
        }
        KiB(value) => {
            if value < 1000_f64 {
                format_value(value, "K", width, precision)
            } else {
                humanize(MiB(value / 1024_f64), width, precision)
            }
        }
        MiB(value) => {
            if value < 1000_f64 {
                format_value(value, "M", width, precision)
            } else {
                humanize(GiB(value / 1024_f64), width, precision)
            }
        }
        GiB(value) => format_value(value, "G", width, precision),
    }
}

fn show_ram_usage() {
    use crate::DataUnit::KiB;
    let info = memory::get_ram_usage();
    let current = info.total - info.avail;
    let percent = current as f64 * 100_f64 / info.total as f64;
    println!(
        "{}/{} ({:.0}%)",
        humanize(KiB(current as f64), 5, 1).trim(),
        humanize(KiB(info.total as f64), 5, 1).trim(),
        percent
    );
}

fn show_swap_usage() {
    use crate::DataUnit::KiB;
    let info = memory::get_swap_usage();
    let current = info.total - info.avail;
    let percent = current as f64 * 100f64 / info.total as f64;
    println!(
        "{}/{} ({:.0}%)",
        humanize(KiB(current as f64), 5, 1).trim(),
        humanize(KiB(info.total as f64), 5, 1).trim(),
        percent
    );
}

fn show_battery_info() {
    let info = battery::get_battery_info();
    let percent = info.now as f64 / info.full as f64 * 100f64;
    let wear = (1f64 - info.full as f64 / info.full_design as f64) * 100f64;
    let icon = if info.charging { '🔌' } else { '🔋' };
    println!("{}{:3.0}% ({:2.0}% wear)", icon, percent, wear);
}

// fn show_brightness_temperature_volume() {
//     loop {}
// }

fn show_cpu_usage() {
    let mut old = cpu::get_cpu_usage();
    loop {
        let current = cpu::get_cpu_usage();
        if current.total == old.total {
            println!("{:2}%", 0);
        } else {
            let usage =
                (current.work - old.work) as f64 / (current.total - old.total) as f64 * 100f64;
            println!("{:2.0}%", usage);
        }
        old = current;
        std::io::stdout().flush().expect("Flush stdout");
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    match std::env::args().nth(1).expect("Missing argument").as_str() {
        "ram" => show_ram_usage(),
        "swap" => show_swap_usage(),
        "battery" => show_battery_info(),
        // "brightness-temperature-volume" => show_brightness_temperature_volume(),
        "cpu" => show_cpu_usage(),
        _ => {}
    }
}
