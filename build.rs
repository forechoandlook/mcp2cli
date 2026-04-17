use std::process::Command;

fn main() {
    // 1. Try to get version from environment variable (set by GitHub Actions)
    if let Ok(val) = std::env::var("APP_RELEASE_VERSION") {
        println!("cargo:rustc-env=APP_VERSION={}", val);
        return;
    }

    // 2. Try to get version from git tag
    let output = Command::new("git")
        .args(&["describe", "--tags", "--always"])
        .output();

    let version = if let Ok(out) = output {
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    } else {
        "0.1.0-dev".to_string()
    };

    println!("cargo:rustc-env=APP_VERSION={}", version);
}
