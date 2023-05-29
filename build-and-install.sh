#!/bin/bash
EIP_LOOKUP_DIR=$(pwd)
cargo build --release
sudo mv $EIP_LOOKUP_DIR/target/release/eip-lookup /usr/local/bin/
