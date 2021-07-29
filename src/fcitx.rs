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
impl From<(String, String, String, String, String, String, bool)> for InputMethod {
    fn from(tup: (String, String, String, String, String, String, bool)) -> InputMethod {
        Self {
            display_name: tup.1,
            name: tup.0,
            lang: tup.5,
            loaded: tup.6,
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

#[dbus_proxy(
    interface = "org.fcitx.Fcitx.Controller1",
    default_service = "org.fcitx.Fcitx5",
    default_path = "/controller"
)]
pub trait Fcitx5Controller {
    #[dbus_proxy(name = "CurrentInputMethod")]
    fn current_input_method(&self) -> zbus::Result<String>;
    #[dbus_proxy(name = "CurrentInputMethodGroup")]
    fn current_input_method_group(&self) -> zbus::Result<String>;
    #[dbus_proxy(name = "AvailableInputMethods")]
    fn input_methods(
        &self,
    ) -> zbus::Result<Vec<(String, String, String, String, String, String, bool)>>;
    #[dbus_proxy(name = "InputMethodGroupInfo")]
    fn input_method_group_info(
        &self,
        group_name: &str,
    ) -> zbus::Result<(String, Vec<(String, String)>)>;
}
