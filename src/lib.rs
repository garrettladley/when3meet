mod app;

use crate::app::App;
use leptos::{logging, view};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("hydrate mode - hydrating");

    leptos::mount_to_body(|| {
        view! { <App/> }
    });
}
