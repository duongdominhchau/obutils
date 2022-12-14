use obutils::fcitx::{Fcitx5ControllerProxy, InputMethod};
use obutils::keyboard_leds::{get_input_id, get_leds_state};
use obutils::util::flush_and_sleep;
use std::time::Duration;
use zbus::Connection;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Fcitx/Fcitx5 DBus interface not found")]
    FcitxNotFound,
    #[error("Zbus error: {0}")]
    ZbusError(#[from] zbus::Error),
}

fn highlight(value: &str) -> String {
    format!("<span foreground='#ff9944'>{}</span>", value)
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

fn render(id: u8, current_im: &str, imlist: &[InputMethod]) -> String {
    if current_im.is_empty() {
        return String::new();
    }
    let display_name = imlist
        .iter()
        .find(|im| im.name == current_im)
        .unwrap()
        .display_name
        .clone();
    format!("{} {}", display_name, leds_state(id))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Polling is bad, but there is no other reliable solution on X11...
    let keyboard_id = get_input_id().expect("Get keyboard ID");
    let zbus_conn = Connection::session().await.unwrap();
    let fcitx5_proxy = Fcitx5ControllerProxy::new(&zbus_conn).await.unwrap();
    let imlist: Vec<InputMethod> = {
        // This doesn't have display name, so we need another call
        let current_group = fcitx5_proxy.current_input_method_group().await.unwrap();
        let active_input_methods: Vec<String> = fcitx5_proxy
            .input_method_group_info(&current_group)
            .await
            .unwrap()
            .1
            .into_iter()
            .map(|t| t.0)
            .collect();
        fcitx5_proxy.input_methods().await.ok().map(|list| {
            list.into_iter()
                .filter_map(|x| {
                    let mut im: InputMethod = x.into();
                    let prefix_to_ignore = "Keyboard - ";
                    if active_input_methods.contains(&im.name) {
                        if im.display_name.starts_with(prefix_to_ignore) {
                            im.display_name.drain(..prefix_to_ignore.len());
                        }
                        Some(im)
                    } else {
                        None
                    }
                })
                .collect()
        })
    }
    .ok_or(Error::FcitxNotFound)?;
    let mut old = String::new();
    loop {
        let current_im = fcitx5_proxy.current_input_method().await?;
        let now = render(keyboard_id, &current_im, &imlist);
        if !now.is_empty() && now != old {
            println!("{}", now);
            old = now;
        }
        flush_and_sleep(Duration::from_millis(50));
    }
}
