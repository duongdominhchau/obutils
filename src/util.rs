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

fn format_value(value: f64, unit: &str, precision: usize, add_space: bool) -> String {
    let width = if precision == 0 {
        3
    } else {
        // One for the decimal point
        3 + 1 + precision
    };
    let number = format!("{:1$.2$}", value, width, precision);
    let number = number.strip_suffix(".0").unwrap_or_else(|| number.as_str());
    format!("{}{}{}", number, if add_space {" "} else { "" }, unit)
}

pub fn humanize(v: DataUnit, precision: usize, add_space: bool) -> String {
    use DataUnit::*;
    match v {
        Byte(value) => {
            if value < 1000 {
                format_value(value as f64, "B", precision, add_space)
            } else {
                humanize(KiB(value as f64 / 1024_f64), precision, add_space)
            }
        }
        KiB(value) => {
            if value < 1000_f64 {
                format_value(value, "K", precision, add_space)
            } else {
                humanize(MiB(value / 1024_f64), precision, add_space)
            }
        }
        MiB(value) => {
            if value < 1000_f64 {
                format_value(value, "M", precision, add_space)
            } else {
                humanize(GiB(value / 1024_f64), precision, add_space)
            }
        }
        GiB(value) => format_value(value, "G", precision, add_space),
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
