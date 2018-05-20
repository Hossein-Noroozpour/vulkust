#!/bin/sh
clear
clear
cargo build --target aarch64-linux-android
mv ../../target/aarch64-linux-android/debug/libandroid_empty_screen.a \
    ../../target/aarch64-linux-android/libvulkust-static.a