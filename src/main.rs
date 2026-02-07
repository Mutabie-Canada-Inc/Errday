#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

mod components;
mod models;
mod store;
mod views;
mod routes; // Add routes module

use store::AppState;
use routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| AppState::new());

    rsx! {
        // Global styles
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        
        Router::<Route> {}
    }
}
