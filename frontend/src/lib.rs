pub mod app;
pub mod api;
pub mod components;
pub mod pages;
pub mod router;
pub mod store;
pub mod types;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}
