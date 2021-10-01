#![cfg(windows)]

windows::include_bindings!();

use crate::color::Color;
use crate::picker::DisplayPicker;
use anyhow::{Error, Result};
use egui::Color32;
use Windows::Win32::Foundation::POINT;
use Windows::Win32::Graphics::Gdi::{GetDC, GetPixel, ReleaseDC, CLR_INVALID, HDC};
use Windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetDesktopWindow};

#[derive(Default, Debug)]
pub struct WinConn {
    device_context: HDC
}

impl WinConn {
    pub fn new() -> Self {
        WinConn {
            device_context: unsafe { GetDC(None) }
        }
    }

    pub fn get_cursor_pos(&self) -> Result<POINT> {
        let mut cursor_pos = POINT::default();

        let success = unsafe { GetCursorPos(&mut cursor_pos as *mut POINT) } .as_bool();
        if !success {
            return Err(Error::msg("failed to get cursor position"));
        }

        Ok(cursor_pos)
    }

    pub fn get_pixel(&self, pos: POINT) -> Result<u32> {
        let color = unsafe { GetPixel(&self.device_context, pos.x, pos.y) };
        if color == CLR_INVALID {
            return Err(Error::msg("failed to get pixel"));
        }
        Ok(color)
    }
}

impl Drop for WinConn {
    fn drop(&mut self) {
        unsafe { ReleaseDC(GetDesktopWindow(), self.device_context) };
    }
}

impl DisplayPicker for WinConn {
    fn get_color_under_cursor(&self) -> Result<Color> {
        let cursor_pos = self.get_cursor_pos()?;

        let color = self.get_pixel(cursor_pos)?;

        let r = (color & 0xff) as u8;
        let g = ((color >> 8) & 0xff) as u8;
        let b = ((color >> 16) & 0xff) as u8;

        Ok(Color::from(Color32::from_rgb(r, g, b)))
    }
}
