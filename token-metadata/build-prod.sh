#!/bin/bash

set -e

cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target ./riscv64ima-cartesi-linux-gnu.json --release
cp ./target/debug/mpl-token-metadata ../../rollups-examples/solana-adapter/solana_smart_contract_bin/metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s
cargo clean
echo "done."
