#!/bin/sh
clear
clear
# cargo build --release &&\
# cp target/aarch64-linux-android/release/libapp.so \
#     ../../../vulkust-and/app/src/main/jniLibs/arm64-v8a/libmain.so
export RUSTFLAGS="-C linker=toolchains/android-arm64-api24/bin/aarch64-linux-android-clang++"
echo "Android toolchain is $RUSTFLAGS"
cargo build --target aarch64-linux-android