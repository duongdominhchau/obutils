use zbus::Connection;

use dbus_interface::FcitxProxy;

mod dbus_interface;
mod error;

pub fn get_current_im(connection: &Connection) -> Option<String> {
    let proxy = match FcitxProxy::new(&connection) {
        Ok(proxy) => proxy,
        Err(_) => return None,
    };
    proxy.current_im().ok()
}

#[cfg(test)]
mod tests {
    use zbus::Connection;

    use crate::fcitx::get_current_im;

    #[test]
    fn test_get_current_im() {
        let connection = Connection::new_session().expect("New session");
        assert_eq!(Some("".to_string()), get_current_im(&connection));
    }
}
