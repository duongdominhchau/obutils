use obutils::cpu::get_cpu_usage;
use obutils::disk::get_disk_io;
use obutils::memory::{get_ram_usage, get_swap_usage};
use obutils::network::{get_network_io, get_networks};
use obutils::util::DataUnit::{Byte, KiB};
use obutils::util::{flush_and_sleep, humanize};
use std::time::{Duration, Instant};

fn main() {
    let interface = get_networks().unwrap().wireless.unwrap();
    let interface = interface.as_str();

    let highlight_color = "foreground='#ff9944'";
    let separator = "  ";

    let mut old_cpu = get_cpu_usage();
    let mut old_net_io = get_network_io(interface);
    let mut old_disk_io = get_disk_io();

    loop {
        let now = Instant::now();

        print!("<span {}>C:</span> ", highlight_color);
        let cpu = get_cpu_usage();
        // Handle the first iteration to avoid division by zero
        if cpu.total == old_cpu.total {
            print!("{:3}%", 0);
        } else {
            let work_diff = (cpu.work - old_cpu.work) as f64;
            let total_diff = (cpu.total - old_cpu.total) as f64;
            print!("{:3.0}%", (work_diff / total_diff * 100f64).round());
        }
        old_cpu = cpu;
        print!("{}", separator);

        print!("<span {}>M:</span> ", highlight_color);
        let ram_info = get_ram_usage();
        let ram_current = ram_info.total - ram_info.avail;
        let ram_percent = (ram_current as f64 / ram_info.total as f64 * 100f64).round();
        print!(
            "{}/{} ({:.0}%)",
            humanize(KiB(ram_current as f64), false).trim(),
            humanize(KiB(ram_info.total as f64), false).trim(),
            ram_percent
        );
        print!("{}", separator);

        print!("<span {}>S:</span> ", highlight_color);
        let swap_info = get_swap_usage();
        if swap_info.total == 0 {
            print!("N/A");
        } else {
            let swap_current = swap_info.total - swap_info.avail;
            let swap_percent = (swap_current as f64 / swap_info.total as f64 * 100f64).round();
            print!(
                "{}/{} ({:.0}%)",
                humanize(KiB(swap_current as f64), false).trim(),
                humanize(KiB(swap_info.total as f64), false).trim(),
                swap_percent
            );
        }
        print!("{}", separator);

        let net_io = get_network_io(interface);
        let received_diff = net_io.received - old_net_io.received;
        let sent_diff = net_io.sent - old_net_io.sent;
        old_net_io = net_io;
        print!(
            "📶⬇️ {} ⬆️ {}{}",
            humanize(Byte(received_diff), true),
            humanize(Byte(sent_diff), true),
            separator
        );

        print!(
            "<span weight='bold' size='x-large' {}>🖴</span> ",
            highlight_color
        );
        let disk_io = get_disk_io();
        let read_diff = disk_io.read - old_disk_io.read;
        let write_diff = disk_io.write - old_disk_io.write;
        old_disk_io = disk_io;
        print!(
            "➡️ {} ⬅️ {}",
            humanize(Byte(read_diff), true),
            humanize(Byte(write_diff), true)
        );

        println!();
        let next_tick = now.checked_add(Duration::from_secs(1)).unwrap();
        flush_and_sleep(next_tick.duration_since(now));
    }
}
