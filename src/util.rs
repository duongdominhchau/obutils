use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::num::ParseIntError;
use std::thread::sleep;
use std::time::Duration;

pub enum DataUnit {
    Byte(u64),
    KiB(f64),
    MiB(f64),
    GiB(f64),
}

fn is_integral(value: f64) -> bool {
    (value.trunc() - value).abs() < 1e8
}

fn format_value(value: f64, unit: &str, add_space: bool) -> String {
    let value_str = if value > 10f64 || is_integral(value) {
        format!("{:3.0}", value)
    } else {
        format!("{:3.1}", value)
    };
    format!("{}{}{}", value_str, if add_space { " " } else { "" }, unit)
}

pub fn humanize(v: DataUnit, add_space: bool) -> String {
    use DataUnit::*;
    match v {
        Byte(value) => {
            if value < 1000 {
                format_value(value as f64, "B", add_space)
            } else {
                humanize(KiB(value as f64 / 1024_f64), add_space)
            }
        }
        KiB(value) => {
            if value < 1000_f64 {
                format_value(value, "K", add_space)
            } else {
                humanize(MiB(value / 1024_f64), add_space)
            }
        }
        MiB(value) => {
            if value < 1000_f64 {
                format_value(value, "M", add_space)
            } else {
                humanize(GiB(value / 1024_f64), add_space)
            }
        }
        GiB(value) => format_value(value, "G", add_space),
    }
}

pub fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let f = File::open(path).unwrap_or_else(|_| panic!("Open \"{}\" for reading", path));
    BufReader::new(f)
        .lines()
        .take_while(|line| line.is_ok())
        .map(|line| line.unwrap())
}

pub fn read_unsigned(path: &str) -> Result<u64, ParseIntError> {
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Open \"{}\" for reading", path))
        .trim()
        .parse::<u64>()
}

pub fn flush_and_sleep(dur: Duration) {
    std::io::stdout().flush().expect("Flush stdout");
    sleep(dur);
}
