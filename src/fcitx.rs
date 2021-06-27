use core::convert::TryFrom;
use serde::Serialize;
use zbus::dbus_proxy;
use zvariant::{derive::OwnedValue, Signature, Type};

#[derive(Debug, Clone, Default, Serialize, OwnedValue)]
pub struct InputMethod {
    /// The name displayed on the UI
    pub display_name: String,
    /// The internal name
    pub name: String,
    /// Language code
    pub lang: String,
    /// Whether the IM is available to switch
    pub loaded: bool,
}
impl From<(String, String, String, bool)> for InputMethod {
    fn from(tup: (String, String, String, bool)) -> InputMethod {
        Self {
            display_name: tup.0,
            name: tup.1,
            lang: tup.2,
            loaded: tup.3,
        }
    }
}
impl Type for InputMethod {
    fn signature() -> Signature<'static> {
        Signature::try_from("sssb").unwrap()
    }
}

#[dbus_proxy(
    interface = "org.fcitx.Fcitx.InputMethod",
    default_service = "org.fcitx.Fcitx",
    default_path = "/inputmethod"
)]
pub trait Fcitx {
    #[dbus_proxy(property, name = "CurrentIM")]
    fn current_im(&self) -> zbus::Result<String>;
    #[dbus_proxy(property, name = "IMList")]
    fn imlist(&self) -> zbus::Result<Vec<(String, String, String, bool)>>;
}
