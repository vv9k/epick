#!/bin/bash
set -eu

OUT_DIR=docs
LISTEN_ADDR=127.0.0.1
LISTEN_PORT=8080

cargo install basic-http-server

echo "open http://${LISTEN_ADDR}:${LISTEN_PORT}"

cd $OUT_DIR && \
    basic-http-server --addr $LISTEN_ADDR:$LISTEN_PORT .
