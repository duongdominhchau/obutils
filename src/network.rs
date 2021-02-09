use crate::util;

fn read_stat(interface: &str, fname: &str) -> u64 {
    util::read_unsigned(format!("/sys/class/net/{}/statistics/{}", interface, fname).as_str())
        .unwrap_or_else(|_| panic!("Read {} of interface {}", fname, interface))
}

#[derive(Default, Copy, Clone)]
pub struct NetworkIo {
    pub received: u64,
    pub sent: u64,
}

pub fn get_network_io(interface: &str) -> NetworkIo {
    let received = read_stat(interface, "rx_bytes");
    let sent = read_stat(interface, "tx_bytes");
    NetworkIo { received, sent }
}
