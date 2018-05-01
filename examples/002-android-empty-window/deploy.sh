#!/bin/sh
clear
clear
export RUSTFLAGS="-C linker=toolchains/android-arm64-api24/bin/aarch64-linux-android-clang++"
echo "Android toolchain is $RUSTFLAGS"
cargo build --target aarch64-linux-android
mv ../../target/aarch64-linux-android/debug/libandroid_empty_screen.so \
    ../../../vulkust-android/app/src/main/jniLibs/arm64-v8a/libvulkust.so