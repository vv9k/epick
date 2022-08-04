mod app;
mod color;
mod color_picker;
mod display_picker;
mod error;
mod math;
mod render;
mod screen_size;
mod ui;

pub use app::App as Epick;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use std::sync::Arc;
pub type TextureAllocator = Option<Arc<egui::mutex::RwLock<epaint::TextureManager>>>;

fn save_to_clipboard(text: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(text)
}

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    eframe::start_web(canvas_id, Box::new(|ctx| Epick::init(ctx)))
}
