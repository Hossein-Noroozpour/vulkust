#!/bin/sh
clear
clear
cargo build --target=arm-linux-androideabi
cp target/arm-linux-androideabi/debug/libvulkust.so ../Vulkust/app/src/main/jniLibs/armeabi/
