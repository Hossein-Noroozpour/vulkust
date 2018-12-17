#!/bin/sh
clear
clear
cargo build $1 --lib --target aarch64-linux-android
if [[ $1 == *"--release"* ]]; then
    echo "Compiling release mode finished"
    mv ../../target/aarch64-linux-android/release/lib*.a \
        ../../target/aarch64-linux-android/libvulkust-static.a
    rm ../../target/aarch64-linux-android/release/lib*.a
else
    echo "Compiling debug mode finished"
    mv ../../target/aarch64-linux-android/debug/lib*.a \
        ../../target/aarch64-linux-android/libvulkust-static.a
    rm ../../target/aarch64-linux-android/debug/lib*.a
fi