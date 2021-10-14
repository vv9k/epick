mod app;
mod color;
mod wm_picker;

pub use app::App as Epick;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn save_to_clipboard(text: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(text)
}

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = Epick::default();
    eframe::start_web(canvas_id, Box::new(app))
}
