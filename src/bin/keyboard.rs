use std::time::Duration;
use tint2_bars::keyboard_leds::{get_input_id, get_leds_state};
use tint2_bars::util::flush_and_sleep;

fn show_fcitx_im() -> String {
    format!("")
}

fn highlight(value: &str) -> String {
    format!("<span foreground='#ff9944'>{}</span>", value)
}

fn show_leds_state(id: u8) -> String {
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

fn info(id: u8) -> String {
    format!("{}{}", show_fcitx_im(), show_leds_state(id))
}

fn main() {
    // Polling is bad, but there is no other reliable solution on X11...
    let keyboard_id = get_input_id().expect("Get keyboard ID");
    let mut old = info(keyboard_id);
    println!("{}", old);

    loop {
        let now = info(keyboard_id);
        if now != old {
            println!("{}", now);
            old = now;
        }
        flush_and_sleep(Duration::from_millis(100));
    }
}
