#!/bin/bash

# Script to create the tar gz and shasum files for gh release and homebrew
# TODO: Automate this into the CI

echo "Building release binary..."
cargo build --target=x86_64-apple-darwin --release
cd target/x86_64-apple-darwin/release
echo "Creating tar archive..."
tar -czf git-smart-checkout.tar.gz git-smart-checkout
echo "Shasum of tar archive:"
shasum -a 256 git-smart-checkout.tar.gz