#!/bin/sh
clear
clear
cargo build --features "intensive-debug log vulkan-debug" --target=armv7-linux-androideabi
