#!/bin/sh
clear
clear
cargo build &&\
cp target/aarch64-linux-android/debug/libapp.so \
    ../../../vulkust-and/app/src/main/jniLibs/arm64-v8a/libmain.so
