#![cfg(windows)]

windows::include_bindings!();

use std::ptr::{null, null_mut};

use crate::color::Color;
use crate::picker::DisplayPicker;
use anyhow::{Error, Result};
use egui::Color32;
use Windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, POINT, PWSTR, WPARAM};
use Windows::Win32::Graphics::Gdi::{GetDC, GetPixel, ReleaseDC, UpdateWindow, CLR_INVALID, HDC};
use Windows::Win32::System::LibraryLoader::GetModuleHandleW;
use Windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, GetCursorPos, GetDesktopWindow, MoveWindow,
    RegisterClassExW, ShowWindow, WNDCLASSEXW,
};

pub use Windows::Win32::Foundation::HWND;
pub use Windows::Win32::UI::WindowsAndMessaging::{
    SHOW_WINDOW_CMD, SW_SHOWDEFAULT, WINDOW_EX_STYLE, WINDOW_STYLE, WS_BORDER, WS_POPUP,
};

pub trait DisplayPickerExt: DisplayPicker {
    #[allow(clippy::too_many_arguments)]
    fn spawn_window(
        &self,
        class_name: &str,
        window_name: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        style: WINDOW_STYLE,
    ) -> Result<HWND>;

    fn show_window(&self, hwnd: HWND, display_mode: SHOW_WINDOW_CMD) -> Result<()>;
    fn destroy_window(&self, hwnd: HWND) -> Result<()>;
    fn update_window(&self, hwnd: HWND) -> Result<()>;
    fn move_window(&self, hwnd: HWND, x: i32, y: i32, width: i32, height: i32) -> Result<()>;
}

#[derive(Default, Debug, Clone)]
pub struct WinConn {
    device_context: HDC,
    hinstance: HINSTANCE,
}

impl WinConn {
    pub fn new() -> Self {
        WinConn {
            device_context: unsafe { GetDC(None) },
            hinstance: unsafe { GetModuleHandleW(PWSTR(null_mut())) },
        }
    }

    pub fn get_cursor_pos(&self) -> Result<POINT> {
        let mut cursor_pos = POINT::default();

        let success = unsafe { GetCursorPos(&mut cursor_pos as *mut POINT) }.as_bool();
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
    fn get_cursor_pos(&self) -> Result<(i32, i32)> {
        self.get_cursor_pos().map(|pos| (pos.x, pos.y))
    }

    fn get_color_under_cursor(&self) -> Result<Color> {
        let cursor_pos = self.get_cursor_pos()?;

        let color = self.get_pixel(cursor_pos)?;

        let r = (color & 0xff) as u8;
        let g = ((color >> 8) & 0xff) as u8;
        let b = ((color >> 16) & 0xff) as u8;

        Ok(Color::from(Color32::from_rgb(r, g, b)))
    }
}

impl DisplayPickerExt for WinConn {
    fn spawn_window(
        &self,
        class_name: &str,
        window_name: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        style: WINDOW_STYLE,
    ) -> Result<HWND> {
        let mut class_name = class_name.encode_utf16().collect::<Vec<_>>();

        let mut class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            ..Default::default()
        };
        class.lpszClassName = PWSTR(class_name.as_mut_ptr());
        class.hInstance = self.hinstance;
        class.lpfnWndProc = Some(wnd_proc);

        let mut window_name = window_name.encode_utf16().collect::<Vec<_>>();

        unsafe { RegisterClassExW(&class as *const WNDCLASSEXW) };

        unsafe {
            Ok(CreateWindowExW(
                WINDOW_EX_STYLE(0),
                PWSTR(class_name.as_mut_ptr()),
                PWSTR(window_name.as_mut_ptr()),
                style,
                x,
                y,
                width,
                height,
                None,
                None,
                self.hinstance,
                null(),
            ))
        }
    }

    fn show_window(&self, hwnd: HWND, display_mode: SHOW_WINDOW_CMD) -> Result<()> {
        if !unsafe { ShowWindow(hwnd, display_mode) }.as_bool() {
            return Err(Error::msg(format!("failed to show window {:?}", hwnd)));
        }
        Ok(())
    }

    fn destroy_window(&self, hwnd: HWND) -> Result<()> {
        if !unsafe { DestroyWindow(hwnd) }.as_bool() {
            return Err(Error::msg(format!("failed to close window {:?}", hwnd)));
        }
        Ok(())
    }

    fn update_window(&self, hwnd: HWND) -> Result<()> {
        if !unsafe { UpdateWindow(hwnd) }.as_bool() {
            return Err(Error::msg(format!("failed to update window {:?}", hwnd)));
        }
        Ok(())
    }

    fn move_window(&self, hwnd: HWND, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        if !unsafe { MoveWindow(hwnd, x, y, width, height, true) }.as_bool() {
            return Err(Error::msg(format!("failed to move window {:?}", hwnd)));
        }
        Ok(())
    }
}

unsafe extern "system" fn wnd_proc(
    param0: HWND,
    param1: u32,
    param2: WPARAM,
    param3: LPARAM,
) -> LRESULT {
    DefWindowProcW(param0, param1, param2, param3)
}
