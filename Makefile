.PHONY: help check clean mrproper
CARGO := env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS cargo +1.99.0
help:
	@echo "check     format, strict Clippy, native palette tests and WASM for egui36/37; legacy default smoke"
	@echo "clean     remove generated Rust build outputs"
	@echo "mrproper  clean and remove the generated Cargo.lock"
check:
	$(CARGO) fmt --all --check
	$(CARGO) check --locked --lib
	$(CARGO) test --locked --lib
	$(CARGO) clippy --locked --lib --no-default-features --features egui36 -- -D warnings
	$(CARGO) test --locked --no-default-features --features egui36 --package catppuccin-egui
	$(CARGO) check --locked --lib --target wasm32-unknown-unknown --no-default-features --features egui36
	$(CARGO) clippy --locked --lib --no-default-features --features egui37 -- -D warnings
	$(CARGO) test --locked --no-default-features --features egui37 --package catppuccin-egui
	$(CARGO) check --locked --lib --target wasm32-unknown-unknown --no-default-features --features egui37
	$(CARGO) deny --offline --exclude todo check
clean:
	$(CARGO) clean
mrproper: clean
	rm -f Cargo.lock
