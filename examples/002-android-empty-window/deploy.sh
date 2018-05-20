#!/bin/sh
clear
clear
export RUSTFLAGS="\
-C linker=toolchains/android-arm64-api24/bin/aarch64-linux-android-clang++ \
-C link-arg=-fno-omit-frame-pointer \
-C link-arg=-g \
"
echo "Android toolchain is $RUSTFLAGS"
cargo build --target aarch64-linux-android
mv ../../target/aarch64-linux-android/debug/libandroid_empty_screen.so \
    ../../../vulkust-android/app/src/main/jniLibs/arm64-v8a/libvulkust.so
# export RUSTFLAGS="-C linker=toolchains/android-arm-api24/bin/arm-linux-androideabi-clang++ \
# -C link-arg=-fembed-bitcode-marker \
# -C link-arg=-fembed-bitcode \
# -C link-arg=-isysroot \
# -C link-arg=`xcrun -sdk iphoneos -show-sdk-path` \
# -C link-arg=-F`xcrun -sdk iphoneos -show-sdk-path`/System/Library/Frameworks \
# -C link-arg=-arch \
# -C link-arg=arm64 \
# "
# echo "Android toolchain is $RUSTFLAGS"
# cargo build --target armv7-linux-androideabi
# mv ../../target/armv7-linux-androideabi/debug/libandroid_empty_screen.so \
#     ../../../vulkust-android/app/src/main/jniLibs/armeabi-v7a/libvulkust.so