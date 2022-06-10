#! /usr/bin/env bash

cargo build --release
rm -f /usr/local/bin/pwdgen
ln -s ${PWD}/target/release/pwdgen /usr/local/bin/pwdgen
