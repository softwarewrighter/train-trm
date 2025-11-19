//! Web UI components (requires 'web' feature)

// This module is only compiled when the 'web' feature is enabled

#[cfg(all(target_arch = "wasm32", feature = "web"))]
pub mod app {
    //! Main web application component
    // TODO: Implement Yew components
}
