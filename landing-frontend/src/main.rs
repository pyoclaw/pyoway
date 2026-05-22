//! Pyoway landing page frontend.
//!
//! A Leptos WASM application with Tailwind CSS styling.

#![deny(unsafe_code)]

use landing_frontend::App;
use leptos::prelude::*;

/// Entry point for the WASM application.
#[allow(clippy::main_recursion)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    // Better panic messages in the browser console
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
