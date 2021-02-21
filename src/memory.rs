use crate::util::read_lines;

#[derive(Debug, Default, Copy, Clone)]
pub struct MemInfo {
    pub total: u64,
    pub avail: u64,
}

fn extract_number(line: &str) -> u64 {
    let value_start = line.find(':').expect("Find colon position") + 1;
    let value_end = line.rfind("kB").expect("Find 'kB' position");
    let trimmed_num = &line[value_start..value_end].trim();
    trimmed_num.parse().expect("Convert to number")
}

fn get_mem_usage(total_title: &str, avail_title: &str) -> MemInfo {
    let mut count = 0;
    let mut info = MemInfo::default();
    for line in read_lines("/proc/meminfo") {
        if line.starts_with(total_title) {
            info.total = extract_number(&line);
            count += 1;
            if count == 2 {
                break;
            }
        } else if line.starts_with(avail_title) {
            info.avail = extract_number(&line);
            count += 1;
            if count == 2 {
                break;
            }
        }
    }
    info
}

pub fn get_swap_usage() -> MemInfo {
    get_mem_usage("SwapTotal", "SwapFree")
}

pub fn get_ram_usage() -> MemInfo {
    get_mem_usage("MemTotal", "MemAvailable")
}
