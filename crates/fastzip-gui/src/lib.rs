use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod app;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(app::App);
}
