use leptos::*;
use crate::app::App;

mod app;
mod components;
mod pages;
mod data;

fn main() {
    logging::log!("csr mode - mounting to body");
    mount_to_body(App)
}
