#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
cargo build --release --target wasm32-unknown-unknown -p morphoid-wasm
rm -rf dist
mkdir dist
cp static/* dist/
cp target/wasm32-unknown-unknown/release/morphoid_wasm.wasm dist/morphoid.wasm
sed 's|<script src="/bundle.js">|<script src="/wasm-shim.js"></script><script src="/bundle.js">|' static/index.html > dist/index.html
