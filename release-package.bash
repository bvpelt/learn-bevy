#!/bin/sh
set -e
cargo build --release
rm -rf dist
mkdir -p dist
cp target/release/learn-bevy dist/
cp -r assets dist/
echo "Release is in ./dist"