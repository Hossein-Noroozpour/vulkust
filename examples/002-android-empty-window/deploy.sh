#!/bin/sh
clear
clear
cargo build --release &&\
cp target/aarch64-linux-android/release/libapp.so \
    ../../../vulkust-and/app/src/main/jniLibs/arm64-v8a/libmain.so
