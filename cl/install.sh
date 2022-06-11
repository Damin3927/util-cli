#! /usr/bin/env bash

set -e

cargo build --release
rm -f /usr/local/bin/cl
cp ./target/release/cl /usr/local/bin
cargo clean
