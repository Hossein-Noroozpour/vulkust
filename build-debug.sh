#!/bin/sh
clear
clear
cargo build --target=armv7-linux-androideabi --features "log"
cp target/armv7-linux-androideabi/debug/libvulkust.so ../Vulkust/app/src/main/jniLibs/armeabi-v7a/
