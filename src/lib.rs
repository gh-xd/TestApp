#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod chart;
mod frac;
pub use app::TemplateApp;
pub use chart::LineDemo;
pub use frac::FractalClock;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    // eframe::start_web(canvas_id, Box::new(|cc| Box::new(TemplateApp::new(cc))))
    // eframe::start_web(canvas_id, Box::new(|cc| Box::new(LineDemo::default())))
    eframe::start_web(canvas_id, Box::new(|cc| Box::new(FractalClock::default())))


}
