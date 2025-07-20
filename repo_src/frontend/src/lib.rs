pub mod app;
pub mod components;
pub mod pages;
pub mod error_template;

// These modules contain server-side logic or depend on server-side features.
// They are conditionally compiled:
// - The `#[server]` macro in `server_fns.rs` handles its own conditional compilation.
// - `database.rs` content is gated with `#[cfg(feature = "ssr")]`.
// They are part of the `frontend` crate because Leptos server functions
// are typically defined in the same crate as the client-side app.
pub mod server_fns; 
pub mod database;

// pub mod models; // if models are separate from shared, usually on server side

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    use leptos::*;
    use leptos_meta::provide_meta_context;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(move || {
        provide_meta_context();
        view! { <App /> }
    });
} 