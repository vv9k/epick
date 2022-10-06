mod app;
mod color;
mod color_picker;
mod context;
mod display_picker;
mod error;
mod keybinding;
mod math;
mod render;
mod screen_size;
mod settings;
mod ui;
mod zoom_picker;

pub use app::App as Epick;

use anyhow::{Context, Error};

#[cfg(not(target_arch = "wasm32"))]
fn save_to_clipboard(text: String) -> Result<(), Error> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard
        .set_text(text)
        .context("failed to save to clipboard")
}

#[cfg(target_arch = "wasm32")]
fn save_to_clipboard(_text: String) -> Result<(), Error> {
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn get_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1)
}

#[cfg(target_arch = "wasm32")]
fn get_timestamp() -> u64 {
    (js_sys::Date::now() / 1000.) as u64
}

fn elapsed(timestamp: u64) -> u64 {
    get_timestamp() - timestamp
}
