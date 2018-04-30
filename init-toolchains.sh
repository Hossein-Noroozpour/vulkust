#!/bin/bash
$NDK_HOME/build/tools/make_standalone_toolchain.py \
    --arch arm64 \
    --api 24 \
    --install-dir toolchains/android-arm64-api24
