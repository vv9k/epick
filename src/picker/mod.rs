#[cfg(windows)]
pub mod windows;
#[cfg(unix)]
pub mod x11;

#[cfg(unix)]
pub use x11::DisplayPickerExt;

use crate::color::Color;
use anyhow::Result;
use std::{fmt::Debug, rc::Rc};

#[cfg(not(unix))]
pub trait DisplayPickerExt: DisplayPicker {}

pub trait DisplayPicker: Debug {
    fn get_cursor_pos(&self) -> Result<(i32, i32)>;
    fn get_color_under_cursor(&self) -> Result<Color>;
}

pub fn init_display_picker() -> Option<Rc<dyn DisplayPickerExt>> {
    #[cfg(unix)]
    return x11::X11Conn::new()
        .ok()
        .map(|conn| Rc::new(conn) as Rc<dyn DisplayPickerExt>);
    #[cfg(windows)]
    return Some(Rc::new(windows::WinConn::new()) as Rc<dyn DisplayPickerExt>);
    #[cfg(all(not(windows), not(unix)))]
    return None;
}
