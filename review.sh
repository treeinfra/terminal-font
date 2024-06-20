export RUSTDOCFLAGS='-D warnings'
cargo fmt --check
cargo check
cargo test
cargo doc
