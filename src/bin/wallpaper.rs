use rand::{thread_rng, Rng};
use std::num::ParseIntError;
use std::ops::Range;
use std::process::Command;
use std::time::Duration;
use walkdir::WalkDir;

#[derive(Debug)]
struct Arguments {
    /// Directory containing all the backgrounds to use
    pub background_dir: String,
    /// Interval between each change, in seconds
    pub interval_in_secs: Duration,
}

fn print_usage_and_exit(exe_name: &str) -> ! {
    eprintln!("Usage: {} background-dir interval-in-seconds", exe_name);
    std::process::exit(1);
}

fn parse_args() -> Result<Arguments, ParseIntError> {
    let mut args = std::env::args();
    let exe_name = args.next().unwrap();
    let background_dir = args
        .next()
        .unwrap_or_else(|| print_usage_and_exit(&exe_name));
    let interval_in_secs = Duration::from_secs(
        args.next()
            .unwrap_or_else(|| print_usage_and_exit(&exe_name))
            .parse::<u64>()?,
    );
    Ok(Arguments {
        background_dir,
        interval_in_secs,
    })
}

fn load_images(path: &str) -> Result<Vec<String>, walkdir::Error> {
    let mut images = Vec::new();
    let extensions = [".jpg", ".jpeg", ".png", ".webp", ".bmp"];
    for entry in WalkDir::new(path) {
        let path = entry?.path().to_str().unwrap().to_string();
        if extensions.iter().any(|ext| path.ends_with(ext)) {
            images.push(path);
        }
    }
    Ok(images)
}

fn next_index(range: Range<usize>, previous_number: Option<usize>) -> usize {
    let mut rng = thread_rng();
    loop {
        let index = rng.gen_range(range.clone());
        if Some(index) == previous_number {
            continue;
        }
        return index;
    }
}

fn set_background(path: &str) -> Result<(), std::io::Error> {
    Command::new("/usr/bin/feh")
        .args(&["--no-fehbg", "--bg-fill", path])
        .output()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;
    let images = load_images(&args.background_dir)?;
    let mut previous_index = None;

    loop {
        let index = next_index(0..images.len(), previous_index);
        previous_index = Some(index);
        set_background(&images[index])?;
        std::thread::sleep(args.interval_in_secs);
    }
}
