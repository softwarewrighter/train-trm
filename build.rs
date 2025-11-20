use std::process::Command;

fn main() {
    // Get the git commit SHA
    let commit = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get the build timestamp (ISO 8601 format)
    let timestamp = chrono::Utc::now().to_rfc3339();

    // Get the hostname
    let hostname = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string());

    // Set environment variables for compile time
    println!("cargo:rustc-env=BUILD_GIT_COMMIT={}", commit);
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
    println!("cargo:rustc-env=BUILD_HOSTNAME={}", hostname);

    // Rerun if .git/HEAD changes (to catch new commits)
    println!("cargo:rerun-if-changed=.git/HEAD");
}
