#!/bin/bash
export CROSS_CONTAINER_ENGINE=podman
#cross build --target aarch64-unknown-linux-musl $*
export RUSTFLAGS="-C target-feature=+crt-static"
cross build --target aarch64-unknown-linux-gnu $*
