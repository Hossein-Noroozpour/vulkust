#!/bin/bash
clear
clear
cargo build --lib --target aarch64-apple-ios
mv ../../target/aarch64-apple-ios/debug/lib*.a ../../target/libvulkust.a