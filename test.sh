#!/bin/bash

set -e

mkdir -p mountdir
cargo build
cp /bin/bash ./mountdir/
clear
sudo ./target/debug/crabcan --debug -u 0 -m ./mountdir/ -c "/bash" -a /lib64:/lib64 -a /lib:/lib
