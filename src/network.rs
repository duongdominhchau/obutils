use std::fs::read_dir;
use std::io::Error;
use std::process::Command;

use crate::util;

fn get_iw_wifi_name(interface: &str) -> Option<String> {
    Command::new("iw")
        .args(vec!["dev", interface, "info"])
        .output()
        .map_or(Some(None), |output| {
            if output.status.success() {
                let output = String::from_utf8(output.stdout).expect("Decode iw output");
                let search_str = "ssid ";
                Some(output.find(search_str).map(|index| {
                    let slice = &output[index + search_str.len()..];
                    slice.lines().next().unwrap().trim().to_string()
                }))
            } else {
                None
            }
        })
        .unwrap()
}

fn get_iwgetid_wifi_name() -> Option<String> {
    Command::new("iwgetid")
        .arg("-r")
        .output()
        .map_or(None, |output| {
            Some(
                String::from_utf8(output.stdout)
                    .expect("Decode iwgetid output")
                    .trim()
                    .to_string(),
            )
        })
}

/// Use `iw dev <interface> info` and `iwgetid` to get SSID
pub fn get_wifi_name(interface: &str) -> Option<String> {
    if let Some(name) = get_iw_wifi_name(interface).or_else(get_iwgetid_wifi_name) {
        if !name.is_empty() {
            return Some(name);
        }
    }
    None
}

#[derive(Debug, Default, Clone)]
pub struct Networks {
    pub wired: Option<String>,
    pub wireless: Option<String>,
}

/// This function assumes there is only one wired interface and one wireless interface
pub fn get_networks() -> Result<Networks, Error> {
    let mut networks = Networks::default();
    for entry in read_dir("/sys/class/net")? {
        let entry = entry?;
        let name = entry
            .file_name()
            .into_string()
            .expect("Convert file name to String")
            .trim()
            .to_string();
        // It can be `eth*`, `enp*s*`, `wlan*`, `wlp*s*`
        if name.starts_with('e') {
            networks.wired = Some(name);
        } else if name.starts_with('w') {
            networks.wireless = Some(name);
        }
    }
    Ok(networks)
}

#[derive(Debug, Default, Copy, Clone)]
pub struct NetworkIo {
    pub received: u64,
    pub sent: u64,
}

fn read_stat(interface: &str, path: &str) -> u64 {
    util::read_unsigned(format!("/sys/class/net/{}/statistics/{}", interface, path).as_str())
        .unwrap_or_else(|_| panic!("Read {} of interface {}", path, interface))
}
/// Get number of bytes a network interface received and sent
pub fn get_network_io(interface: &str) -> NetworkIo {
    let received = read_stat(interface, "rx_bytes");
    let sent = read_stat(interface, "tx_bytes");
    NetworkIo { received, sent }
}
