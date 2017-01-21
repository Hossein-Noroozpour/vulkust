#!/bin/sh
clear
clear
cargo build --release --target=armv7-linux-androideabi
cp target/armv7-linux-androideabi/release/libvulkust.so ../Vulkust/app/src/main/jniLibs/armeabi-v7a/