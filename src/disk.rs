use std::fs;

use crate::util;

#[derive(Debug, Default, Copy, Clone)]
pub struct DiskIo {
    pub read: u64,
    pub write: u64,
}

const DEVICE_NAME: usize = 2;
const READ_SECTORS: usize = 5;
const WRITE_SECTORS: usize = 9;
// Sector is not device-specific, it is the standard UNIX 512 bytes sector
// https://www.kernel.org/doc/Documentation/block/stat.txt
const SECTOR_SIZE: usize = 512;

pub fn get_disk_io() -> DiskIo {
    let drives: Vec<String> = fs::read_dir("/sys/block")
        .expect("Read directory /sys/block")
        .map(|entry| {
            entry
                .expect("Read metadata of file")
                .file_name()
                .into_string()
                .expect("Convert OsString to String")
        })
        .collect();
    let mut read: usize = 0;
    let mut write: usize = 0;
    for line in util::read_lines("/proc/diskstats") {
        let device_name = line
            .split_whitespace()
            .nth(DEVICE_NAME)
            .expect("Get device name from disk stats");
        if drives.contains(&device_name.to_string()) {
            read += line
                .split_whitespace()
                .nth(READ_SECTORS)
                .expect("Get read sectors from disk stats")
                .parse::<usize>()
                .unwrap()
                * SECTOR_SIZE;
            write += line
                .split_whitespace()
                .nth(WRITE_SECTORS)
                .expect("Get write sectors from disk stats")
                .parse::<usize>()
                .unwrap()
                * SECTOR_SIZE;
        }
    }
    DiskIo {
        read: read as u64,
        write: write as u64,
    }
}
