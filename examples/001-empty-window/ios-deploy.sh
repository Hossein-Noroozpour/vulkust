#!/bin/sh
clear
clear
# export RUSTFLAGS="-C linker=`xcrun -sdk iphoneos -find clang++` -isysroot `xcrun -sdk iphoneos -show-sdk-path` -F`xcrun -sdk iphoneos -show-sdk-path`/System/Library/Frameworks -arch arm64"
export RUSTFLAGS="-C linker=`xcrun -sdk iphoneos -find clang++` -C link-arg=-F`xcrun -sdk iphoneos -show-sdk-path`/System/Library/Frameworks"
echo "iOS toolchain is $RUSTFLAGS"
cargo build --target aarch64-apple-ios
# export RUSTFLAGS="-C linker=`xcrun -sdk iphoneos -find clang++` -isysroot `xcrun -sdk iphoneos -show-sdk-path` -F`xcrun -sdk iphoneos -show-sdk-path`/System/Library/Frameworks -arch x86_64"
# echo "iOS toolchain is $RUSTFLAGS"
# cargo build --target x86_64-apple-ios
# mv ../../target/aarch64-linux-android/debug/libandroid_empty_screen.so \
    # ../../../vulkust-android/app/src/main/jniLibs/arm64-v8a/libvulkust.so