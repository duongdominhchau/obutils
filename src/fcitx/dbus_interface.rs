use crate::fcitx::error::FcitxError;
use crate::fcitx::error::FcitxError::FailToDeserialize;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use zbus::dbus_proxy;
use zvariant::derive::Type;
use zvariant::{Array, OwnedValue, Value};

#[derive(Default, Debug, Type, Serialize, Deserialize)]
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

impl TryFrom<OwnedValue> for InputMethod {
    type Error = FcitxError;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        // let arr = match value.downcast::<(&str, &str, &str, bool)>() {
        //     Some(value) => value,
        //     None => return Err(FailToDeserialize()),
        // };
        // let display_name = arr.0.to_owned();
        // let mut iter: Vec<Value> = arr
        //     .get()
        //     .iter()
        //     .take(3)
        //     .map(|value| value.into::<String>())
        //     .collect();
        // let display_name = match iter.next() {
        //     Some(value) => value,
        //     None => return Err(FailToDeserialize()),
        // };
        // let display_name = match iter.next() {
        //     Some(value) => value,
        //     None => return Err(FailToDeserialize()),
        // };
        // let display_name = match iter.next() {
        //     Some(value) => value,
        //     None => return Err(FailToDeserialize()),
        // };
        // let display_name = match iter.next() {
        //     Some(value) => value,
        //     None => return Err(FailToDeserialize()),
        // };
        // im.a = ;
        unimplemented!()
    }
}

#[dbus_proxy(interface = "org.fcitx.Fcitx.InputMethod")]
pub trait Fcitx {
    /// CurrentIM property
    #[dbus_proxy(property)]
    fn current_im(&self) -> zbus::Result<String>;
    #[DBusProxy(property)]
    fn set_current_im(&self, value: &str) -> zbus::Result<()>;

    // /// IMList property
    // #[dbus_proxy(property)]
    // fn imlist(&self) -> zbus::Result<Vec<(String, String, String, bool)>>;
    // #[DBusProxy(property)]
    // fn set_imlist(&self, value: &[InputMethod]) -> zbus::Result<()>;
}
