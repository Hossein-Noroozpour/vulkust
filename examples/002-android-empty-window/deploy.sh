#!/bin/sh
clear
clear
cargo build &&\
cp target/armv7-linux-androideabi/debug/libapp.so ../../../Vulkust/app/src/main/jniLibs/armeabi-v7a/libapp.so
