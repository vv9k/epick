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

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    eframe::start_web(canvas_id, Box::new(|ctx| Epick::init(ctx)))
}
