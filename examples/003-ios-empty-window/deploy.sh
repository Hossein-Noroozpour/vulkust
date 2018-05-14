#!/bin/bash
clear
clear
export RUSTFLAGS="-C linker=\"`xcrun -sdk iphoneos -find clang++`\" \
-C link-arg=-isysroot \
-C link-arg=`xcrun -sdk iphoneos -show-sdk-path` \
-C link-arg=-F`xcrun -sdk iphoneos -show-sdk-path`/System/Library/Frameworks \
-C link-arg=-arch \
-C link-arg=arm64 \
-C link-arg=-fembed-bitcode"
cargo build --target aarch64-apple-ios
export RUSTFLAGS="-C linker=\"`xcrun -sdk iphoneos -find clang++`\" \
-C link-arg=-isysroot \
-C link-arg=`xcrun -sdk iphonesimulator -show-sdk-path` \
-C link-arg=-F`xcrun -sdk iphonesimulator -show-sdk-path`/System/Library/Frameworks \
-C link-arg=-arch \
-C link-arg=x86_64' \
-C link-arg=-fembed-bitcode"
cargo build --target x86_64-apple-ios
lipo ../../target/aarch64-apple-ios/debug/lib*.a \
    ../../target/x86_64-apple-ios/debug/lib*.a \
    -create -output ../../target/libvulkust.a