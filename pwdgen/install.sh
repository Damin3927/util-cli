#! /usr/bin/env bash

set -e

cargo build --release
rm -f /usr/local/bin/pwdgen
cp ./target/release/pwdgen /usr/local/bin
cargo clean
