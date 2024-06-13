server:
	cargo build --lib --manifest-path=./aayojak-server/Cargo.toml
	cargo test --manifest-path=./aayojak-server/Cargo.toml
	cargo doc --manifest-path=./aayojak-server/Cargo.toml
