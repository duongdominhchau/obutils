use spin_sleep::sleep;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::num::ParseIntError;
use std::time::Duration;

pub enum DataUnit {
    Byte(u64),
    KiB(f64),
    MiB(f64),
    GiB(f64),
}

fn format_value(value: f64, unit: &str, width: usize, precision: usize) -> String {
    format!("{0:2$.3$}{1}", value, unit, width, precision)
}

pub fn humanize(v: DataUnit, width: usize, precision: usize) -> String {
    use DataUnit::*;
    match v {
        Byte(value) => {
            if value < 1000 {
                format_value(value as f64, "B", width, precision)
            } else {
                humanize(KiB(value as f64 / 1024_f64), width, precision)
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
