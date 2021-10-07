#!/bin/bash
set -eu

FOLDER_NAME=${PWD##*/}
CRATE_NAME=$FOLDER_NAME # assume crate name is the same as the folder name
CRATE_NAME_SNAKE_CASE="${CRATE_NAME//-/_}" # for those who name crates with-kebab-case

BUILD=release
TARGET=wasm32-unknown-unknown
CARGO_BUILD_FLAGS="-p ${CRATE_NAME} \
                  --lib \
                  --target ${TARGET}"

if [[ $BUILD =~ release ]]
then
    CARGO_BUILD_FLAGS="--release ${CARGO_BUILD_FLAGS}"
fi

OUT_DIR=docs
OUT_FILE="${CRATE_NAME_SNAKE_CASE}.wasm"
WASM_FLAGS="--out-dir ${OUT_DIR} \
            --target no-modules  \
            --no-typescript      \
            --omit-imports"



# This is required to enable the web_sys clipboard API which egui_web uses
# https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Clipboard.html
# https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html
export RUSTFLAGS=--cfg=web_sys_unstable_apis


# Clear output from old stuff:
rm -f ${OUT_DIR}/$OUT_FILE


echo "Building rust…"
cargo build $CARGO_BUILD_FLAGS

echo "Generating JS bindings for wasm…"
wasm-bindgen "target/${TARGET}/${BUILD}/${OUT_FILE}" $WASM_FLAGS

echo "Finished: ${OUT_DIR}/${OUT_FILE//.wasm/_bg.wasm}"
