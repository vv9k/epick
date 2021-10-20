fn main() {
    #[cfg(windows)]
    windows::build! {
        Windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, POINT, PWSTR, WPARAM, HWND},
        Windows::Win32::Graphics::Gdi::{
            BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, GetDC, GetPixel, ReleaseDC,
            SelectObject, SetStretchBltMode, UpdateWindow, CLR_INVALID, HBITMAP, HDC,
            StretchBlt, Rectangle
        },
        Windows::Win32::System::LibraryLoader::GetModuleHandleW,
        Windows::Win32::UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, GetCursorPos, GetDesktopWindow, MoveWindow,
            SHOW_WINDOW_CMD, WINDOW_EX_STYLE, WINDOW_STYLE, RegisterClassExW, ShowWindow, WNDCLASSEXW,
        }
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AppKit");
    }
}
