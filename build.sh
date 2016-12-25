#!/bin/sh
clear
clear
cargo build --release --target=arm-linux-androideabi
cp target/arm-linux-androideabi/release/libvulkust.so ../Vulkust/app/src/main/jniLibs/armeabi/
