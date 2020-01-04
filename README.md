# sc-examples-rs
Smart contract examples for Elrond's Arwen VM, written in Rust.

# Note:

The Rust framework is not yet published, so we are using the unpublished version, via relative path. To build or debug the examples, you must clone elrond-wasm-rs in the same parent directory as this project.


# utils

To build any of the Rust projects:
```
cargo build --bin wasm --target=wasm32-unknown-unknown --release
```

To debug macros:
```
cargo +nightly rustc -- -Z unstable-options --pretty=expanded > demacroed.rs
```