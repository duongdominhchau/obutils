use crate::util::read_lines;

#[derive(Debug, Default, Copy, Clone)]
pub struct CpuUsage {
    pub work: u64,
    pub total: u64,
}

pub fn get_cpu_usage() -> CpuUsage {
    for line in read_lines("/proc/stat") {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] == "cpu" {
            let user: u64 = parts[1].parse().expect("Read CPU `user` time");
            let nice: u64 = parts[2].parse().expect("Read CPU `nice` time");
            let system: u64 = parts[3].parse().expect("Read CPU `system` time");
            let idle: u64 = parts[4].parse().expect("Read CPU `idle` time");
            let iowait: u64 = parts[5].parse().expect("Read CPU `iowait` time");
            let irq: u64 = parts[5].parse().expect("Read CPU `irq` time");
            let softirq: u64 = parts[5].parse().expect("Read CPU `softirq` time");
            let work_time = user + nice + system;
            return CpuUsage {
                work: work_time,
                total: work_time + idle + iowait + irq + softirq,
            };
        }
    }
    CpuUsage::default()
}
