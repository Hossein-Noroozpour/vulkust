#!/bin/sh
clear
clear
cargo build --lib --target aarch64-linux-android
mv ../../target/aarch64-linux-android/debug/lib*.a \
    ../../target/aarch64-linux-android/libvulkust-static.a
rm ../../target/aarch64-linux-android/debug/lib*.a
