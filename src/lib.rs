mod app;
mod color;
pub use app::ColorPicker;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = ColorPicker::default();
    eframe::start_web(canvas_id, Box::new(app))
}
