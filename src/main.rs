#![allow(non_snake_case)]

use dioxus::prelude::*;

// Internal modules for organizing code
mod components;
mod models;
mod store;
mod views;
mod routes; 

use store::AppState;
use routes::Route;

// Define paths to static assets like icons and styles
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

/// THE MISSION START: This is where the application launches
fn main() {
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    // Stage 1: Initialize logging only during development to keep the app clean
    #[cfg(debug_assertions)]
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    
    // Stage 2: Configure the desktop window (Title, size, and styling)
    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("Errday")
                .with_inner_size(dioxus::desktop::LogicalSize::new(1280.0, 800.0))
                .with_min_inner_size(dioxus::desktop::LogicalSize::new(1000.0, 700.0))
        )
        // Set a dark background immediately to avoid a white flash on startup
        .with_custom_head(r#"<style>body { background-color: #0B0D17; }</style>"#.to_string());

    // Stage 3: Launch the app with our specific configuration
    LaunchBuilder::desktop().with_cfg(config).launch(App);
}

/// MAIN COMPONENT: The root UI element of the application
#[component]
fn App() -> Element {
    // Initialize the shared state (the "Brain") for the entire app
    use_context_provider(|| AppState::new());

    rsx! {
        // Link our global styles and favicon
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        
        // Define the main container with the deep space theme
        div { class: "dark bg-space-900 text-gray-100 min-h-screen font-sans selection:bg-neon-cyan selection:text-space-900",
            // Load the internal pages based on the user's current route
            Router::<Route> {}
        }
    }
}
