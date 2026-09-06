fn main() {
    // Windows does not support the file links used by Rustlings when building
    // from source, so copy the development manifest instead.
    #[cfg(windows)]
    if let Err(err) = std::fs::copy("dev/Cargo.toml", "dev-Cargo.toml") {
        panic!("failed to copy dev/Cargo.toml: {err}");
    }
}
