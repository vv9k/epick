fn main() {
    #[cfg(windows)]
    windows::build! {
        Windows::Win32::Graphics::Gdi::{GetDC, GetPixel, ReleaseDC, CLR_INVALID, HDC, UpdateWindow},
        Windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetDesktopWindow, CreateWindowExW, WINDOW_STYLE, WINDOW_EX_STYLE, HMENU, ShowWindow, WNDCLASSEXW, RegisterClassExW, CW_USEDEFAULT, DefWindowProcW, SHOW_WINDOW_CMD, MoveWindow},
        Windows::Win32::Foundation::{PWSTR, HWND, POINT, WPARAM, LPARAM, LRESULT},
        Windows::Win32::System::{
            LibraryLoader::GetModuleHandleW,
        }
    };
}
