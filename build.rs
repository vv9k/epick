fn main() {
    #[cfg(windows)]
    windows::build! {
        Windows::Win32::Graphics::Gdi::{GetDC, GetPixel, ReleaseDC, CLR_INVALID, HDC},
        Windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetDesktopWindow},
        Windows::Win32::Foundation::POINT,
    };
}
