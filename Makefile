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


.PHONY: start_web
start_web: build_web
	@./scripts/start_server.sh


.PHONY: build_web
build_web: docs/$(PROJECT)_bg.wasm docs/$(PROJECT).js


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


.PHONY: clean
clean: clean_web
	@rm -rf target/*

.PHONY: clean_web
clean_web:
	@rm -rf docs/$(PROJECT)_bg.wasm docs/$(PROJECT).js


./target/debug/$(PROJECT):
	@cargo build


./target/release/$(PROJECT):
	@cargo build --release


docs/$(PROJECT)_bg.wasm docs/$(PROJECT).js:
	@./scripts/build_web.sh

