pub mod app;
pub mod domain;

#[cfg(feature = "ssr")]
pub mod config;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[cfg(feature = "csr")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn csr() {
    use crate::app::App;

    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
