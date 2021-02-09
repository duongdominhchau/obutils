use std::time::Duration;

use util::{humanize, DataUnit::*};

mod battery;
mod brightness;
mod cpu;
mod disk;
mod memory;
mod network;
mod util;

fn show_ram_usage() {
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

fn show_brightness() {
    loop {
        let brightness = brightness::get_brightness();
        let icon = match brightness {
            0..=19 => "🌑",
            20..=39 => "🌘",
            40..=59 => "🌗",
            60..=79 => "🌖",
            80..=100 => "🌕",
            _ => panic!("Invalid brightness value"),
        };
        println!("{} {}%", icon, brightness);
        util::flush_and_sleep(Duration::from_secs(1));
    }
}

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
        util::flush_and_sleep(Duration::from_secs(1));
    }
}

fn show_network_io(interface: &str) {
    let mut old = network::get_network_io(interface);
    loop {
        let current = network::get_network_io(interface);
        println!(
            "⬇️ {}/s ⬆️ {}/s",
            humanize(Byte(current.received - old.received), 5, 1).trim(),
            humanize(Byte(current.sent - old.sent), 5, 1).trim()
        );
        util::flush_and_sleep(Duration::from_secs(1));
        old = current;
    }
}

fn show_disk_io() {
    let mut old = disk::get_disk_io();
    loop {
        let current = disk::get_disk_io();
        println!(
            "➡️ {}/s ⬅️ {}/s",
            humanize(Byte(current.read - old.read), 5, 1),
            humanize(Byte(current.write - old.write), 5, 1)
        );
        old = current;
        util::flush_and_sleep(Duration::from_secs(1));
    }
}

fn main() {
    match std::env::args().nth(1).expect("Missing argument").as_str() {
        "ram" => show_ram_usage(),
        "swap" => show_swap_usage(),
        "battery" => show_battery_info(),
        "brightness" => show_brightness(),
        "cpu" => show_cpu_usage(),
        "network" => {
            let interface = std::env::args()
                .nth(2)
                .expect("Read interface name from command-line args");
            show_network_io(&interface);
        }
        "disk" => show_disk_io(),
        // "temperature" => show_temperature(),
        // "volume" => show_volume(),
        _ => {}
    }
}
