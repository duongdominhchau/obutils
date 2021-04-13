use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::thread::{sleep, spawn};
use std::time::Duration;

use notify::{watcher, RecursiveMode, Watcher};

use tint2_bars::battery::get_battery_info;
use tint2_bars::brightness::get_brightness;
use tint2_bars::pulseaudio::get_sink_state;
use tint2_bars::util::Percent;

fn show_brightness() {
    let brightness = get_brightness();
    let icon = match brightness {
        0..=19 => "🌑",
        20..=39 => "🌘",
        40..=59 => "🌗",
        60..=79 => "🌖",
        80..=100 => "🌕",
        _ => panic!("Invalid brightness value"),
    };
    print!("{} {}%", icon, brightness);
}

fn show_volume() {
    let state = get_sink_state();
    let icon = if state.muted {
        "🔇"
    } else {
        match state.volume {
            0..=35 => "🔈",
            36..=70 => "🔉",
            _ => "🔊",
        }
    };
    print!("{} {}%", icon, state.volume);
}

fn show_battery() {
    let info = get_battery_info();
    let remaining = (info.now as f64 / info.full as f64 * 100.0).clamp(0.0, 100.0);
    let wear_out =  (1.0 - (info.full as f64 / info.full_design as f64)) * 100.0;
    let icon = if info.charging { '🔌' } else { '🔋' };
    print!("{}{:3.0}% ({:2.0}% wear)", icon, remaining, wear_out);
}

fn print_info() {
    show_brightness();
    print!(" ");
    show_volume();
    print!(" ");
    show_battery();
    println!();
}

fn main() {
    print_info();

    // Volume
    let mut volume_watcher = Command::new("pactl")
        .arg("subscribe")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let reader = match volume_watcher.stdout.take() {
        Some(stdout) => BufReader::new(stdout),
        None => {
            volume_watcher.kill().unwrap();
            panic!("Can't take stdout of child process");
        }
    };
    spawn(|| {
        for line in reader.lines() {
            if line.unwrap().contains(" sink ") {
                print_info();
            }
        }
    });

    // Battery
    spawn(|| {
        sleep(Duration::from_secs(60));
        print_info();
    });

    // Brightness
    let (tx, rx) = channel();
    let mut brightness_watcher = watcher(tx, Duration::from_millis(100)).unwrap();
    brightness_watcher
        .watch(
            "/sys/class/backlight/intel_backlight/brightness",
            RecursiveMode::NonRecursive,
        )
        .unwrap_or_else(|_| volume_watcher.kill().unwrap());
    loop {
        if rx.recv().is_ok() {
            print_info();
        }
    }
}
