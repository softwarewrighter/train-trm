//! WASM entry point for web UI

#[cfg(all(target_arch = "wasm32", feature = "web"))]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<train_trm::web::App>::new().render();
}

#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
fn main() {
    eprintln!("This binary is only for WASM builds with the 'web' feature enabled.");
    eprintln!("Use: cargo build --bin web --target wasm32-unknown-unknown --features web");
    std::process::exit(1);
}
