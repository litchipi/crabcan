#!/bin/bash

set -e

mkdir -p mountdir
cd testbin
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target="x86_64-unknown-linux-gnu"
cp target/x86_64-unknown-linux-gnu/release/testbin ../mountdir/
cd ..
cargo build
clear
sudo ./target/debug/crabcan --debug -u 0 -m ./mountdir/ -c "/testbin"
