use std::time::Duration;
use obutils::fcitx::{FcitxProxy, InputMethod};
use obutils::keyboard_leds::{get_input_id, get_leds_state};
use obutils::util::flush_and_sleep;
use zbus::Connection;

#[derive(Debug, Clone, Copy)]
enum Mode {
    Both,
    Fcitx,
    Leds,
}
fn highlight(value: &str) -> String {
    format!("<span foreground='#ff9944'>{}</span>", value)
}

fn fcitx_im(proxy: &FcitxProxy, imlist: &[InputMethod]) -> String {
    let current_im = proxy.current_im().unwrap();
    imlist
        .iter()
        .find(|im| im.name == current_im)
        .unwrap()
        .display_name
        .clone()
}
fn leds_state(id: u8) -> String {
    let state = get_leds_state(id).expect("Read keyboard leds state");
    let mut result = String::new();
    if state.num_lock {
        result.push_str(&*highlight("[Num]"));
    }
    if state.caps_lock {
        result.push_str(&*highlight("[Caps]"));
    }
    result
}

fn fetch(id: u8, proxy: &FcitxProxy, imlist: &[InputMethod], mode: Mode) -> String {
    match mode {
        Mode::Both => format!("{} {}", fcitx_im(&proxy, &imlist), leds_state(id)),
        Mode::Fcitx => format!("{}", fcitx_im(&proxy, &imlist)),
        Mode::Leds => format!("{}", leds_state(id)),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let exe = args.next().unwrap();
    let mode = match args.next() {
        None => Mode::Both,
        Some(mode) => {
            if mode == "fcitx" {
                Mode::Fcitx
            } else if mode == "led" {
                Mode::Leds
            } else {
                eprintln!("Usage: {} [led|fcitx]", exe);
                std::process::exit(1);
            }
        }
    };

    // Polling is bad, but there is no other reliable solution on X11...
    let keyboard_id = get_input_id().expect("Get keyboard ID");
    let zbus_conn = Connection::new_session()?;
    let zbus_proxy = FcitxProxy::new(&zbus_conn)?;
    let imlist: Vec<InputMethod> = zbus_proxy
        .imlist()
        .unwrap()
        .into_iter()
        .filter_map(|x| {
            let mut im: InputMethod = x.into();
            let prefix_to_ignore = "Keyboard - ";
            if im.loaded {
                if im.display_name.starts_with(prefix_to_ignore) {
                    im.display_name.drain(..prefix_to_ignore.len());
                }
                Some(im)
            } else {
                None
            }
        })
        .collect();

    let mut old = fetch(keyboard_id, &zbus_proxy, &imlist, mode);
    println!("{}", old);
    loop {
        let now = fetch(keyboard_id, &zbus_proxy, &imlist, mode);
        if now != old {
            println!("{}", now);
            old = now;
        }
        flush_and_sleep(Duration::from_millis(50));
    }
}
