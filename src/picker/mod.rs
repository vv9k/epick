#[cfg(windows)]
pub mod windows;
#[cfg(target_os = "linux")]
pub mod x11;

#[cfg(windows)]
pub use self::windows::DisplayPickerExt;
#[cfg(target_os = "linux")]
pub use x11::DisplayPickerExt;
#[cfg(not(any(target_os = "linux", windows)))]
pub trait DisplayPickerExt: DisplayPicker {}

use crate::color::Color;
use anyhow::Result;
use std::{fmt::Debug, rc::Rc};

pub trait DisplayPicker: Debug {
    fn get_cursor_pos(&self) -> Result<(i32, i32)>;
    fn get_color_under_cursor(&self) -> Result<Color>;
}

pub fn init_display_picker() -> Option<Rc<dyn DisplayPickerExt>> {
    #[cfg(target_os = "linux")]
    return x11::X11Conn::new()
        .ok()
        .map(|conn| Rc::new(conn) as Rc<dyn DisplayPickerExt>);
    #[cfg(windows)]
    return Some(Rc::new(windows::WinConn::new()) as Rc<dyn DisplayPickerExt>);
    #[cfg(all(not(windows), not(target_os = "linux")))]
    return None;
}
