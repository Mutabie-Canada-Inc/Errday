use dioxus::prelude::*;

#[component]
pub fn Credits() -> Element {
    rsx! {
        div { class: "flex-1 h-full p-12 overflow-y-auto",
            div { class: "max-w-3xl mx-auto space-y-12 text-center",
                // Header
                div { class: "border-b border-gray-800 pb-8",
                    h2 { class: "text-4xl font-bold mb-2 text-white font-sans tracking-tight", "Mission Control" }
                    p { class: "text-gray-400 font-mono text-sm", "CREDITS // MUTABIE CANADA INC" }
                }

                div { class: "glass-panel p-12 rounded-2xl relative overflow-hidden",
                    // Background glow
                    div { class: "absolute top-0 left-1/2 -translate-x-1/2 w-64 h-64 bg-neon-cyan/10 blur-[100px] rounded-full pointer-events-none" }
                    
                    div { class: "relative z-10 space-y-8",
                        div {
                            h3 { class: "text-2xl font-bold text-white mb-2", "Developed By" }
                            p { class: "text-xl text-neon-cyan tracking-wider font-mono", "Mutabie Canada Inc." }
                            p { class: "text-gray-500 mt-2", "Empowering founders to build the future." }
                        }

                        div { class: "w-16 h-1 bg-gray-800 mx-auto rounded-full" }

                        div { class: "grid grid-cols-2 gap-8 text-left",
                            div {
                                h4 { class: "font-mono text-xs text-gray-500 uppercase tracking-widest mb-3", "Version" }
                                p { class: "text-gray-300", "v1.0.0 (MVP)" }
                            }
                            div {
                                h4 { class: "font-mono text-xs text-gray-500 uppercase tracking-widest mb-3", "Stack" }
                                p { class: "text-gray-300", "Rust + Dioxus" }
                            }
                            div {
                                h4 { class: "font-mono text-xs text-gray-500 uppercase tracking-widest mb-3", "License" }
                                p { class: "text-gray-300", "Proprietary" }
                            }
                            div {
                                h4 { class: "font-mono text-xs text-gray-500 uppercase tracking-widest mb-3", "Contact" }
                                p { class: "text-gray-300", "info@mutabie.ca" }
                            }
                        }
                    }
                }
            }
        }
    }
}
