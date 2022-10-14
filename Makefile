PROJECT := epick


.PHONY: all
all: clean test build


.PHONY: all_debug
all_debug: clean test build_debug


.PHONY: run_debug
run_debug: build_debug
	@./target/debug/$(PROJECT)


.PHONY: run
run: build
	@./target/release/$(PROJECT)


.PHONY: build_debug
build_debug: ./target/debug/$(PROJECT)


.PHONY: build
build: ./target/release/$(PROJECT)


.PHONY: get_trunk
get_trunk:
	cargo install --locked trunk

.PHONY: get_wasm_bindgen
get_wasm_bindgen:
	cargo install --locked wasm-bindgen-cli

.PHONY: get_wasm_target
get_wasm_target:
	@rustup target add wasm32-unknown-unknown

.PHONY: setup_wasm
setup_wasm: get_trunk get_wasm_bindgen get_wasm_target


.PHONY: start_web
start_web: setup_wasm
	trunk serve


.PHONY: build_web
build_web: dist/$(PROJECT)_bg.wasm dist/$(PROJECT).js

.PHONY: build_web_ghpages
build_web_ghpages: build_web
	@sed -i 's/epick-/epick\/epick-/g' dist/index.html

.PHONY: test
test:
	cargo t --all-targets --all-features

.PHONY: fmt
fmt:
	cargo fmt --all -- --check

.PHONY: clippy
clippy:
	@rustup component add clippy
	cargo clippy --all-targets --all-features -- -D clippy::all

.PHONY: lint
lint: fmt clippy

.PHONY: clean
clean: clean_web
	@rm -rf target/*

.PHONY: clean_web
clean_web:
	@rm -rf dist/$(PROJECT)_bg.wasm dist/$(PROJECT).js


./target/debug/$(PROJECT):
	@cargo build


./target/release/$(PROJECT):
	@cargo build --release


dist/$(PROJECT)_bg.wasm dist/$(PROJECT).js: setup_wasm
	@trunk build --release

