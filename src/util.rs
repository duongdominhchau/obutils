use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::num::ParseIntError;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub enum DataUnit {
    Byte(u64),
    KiB(f64),
    MiB(f64),
    GiB(f64),
}

fn is_integral(value: f64) -> bool {
    (value.trunc() - value).abs() < 1e-15
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

#[cfg(test)]
mod tests {
    use crate::util::DataUnit::{Byte, KiB};
    use crate::util::{humanize, is_integral};

    #[test]
    fn is_integral_one_is_true() {
        assert_eq!(is_integral(1.0), true);
    }

    #[test]
    fn is_integral_one_and_a_half_is_false() {
        assert_eq!(is_integral(1.5), false);
    }

    #[test]
    fn is_integral_first_15_digits_are_checked() {
        assert_eq!(is_integral(1.999_999_999_999_999), false);
    }

    #[test]
    fn is_integral_16th_digit_and_later_are_not_checked() {
        assert_eq!(is_integral(1.999_999_999_999_999_9), true);
    }

    #[test]
    fn humanize_zero_byte() {
        assert_eq!(humanize(Byte(0), true).trim(), "0 B");
    }

    #[test]
    fn humanize_when_value_is_less_than_10_round_to_the_first_digit() {
        assert_eq!(humanize(KiB(1.55), true).trim(), "1.6 K");
    }

    #[test]
    fn humanize_when_value_is_greater_than_10_round_to_int() {
        assert_eq!(humanize(KiB(12.5), true).trim(), "12 K");
    }

    #[test]
    fn humanize_when_value_is_1000_or_more_convert_to_next_unit() {
        assert_eq!(humanize(KiB(1572864.0), true).trim(), "1.5 G");
    }

    #[test]
    fn humanize_output_number_length_is_3() {
        assert_eq!(humanize(Byte(0), true).len(), 3 + " B".len());
        assert_eq!(humanize(KiB(12.5), true).len(), 3 + " K".len());
        assert_eq!(humanize(KiB(1572864.0), true).len(), 3 + " G".len());
    }

    #[test]
    fn humanize_right_align_value() {
        assert_eq!(humanize(Byte(0), true), "  0 B");
    }
}
