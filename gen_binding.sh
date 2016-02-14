#!/bin/bash
# Generates Vulkan bindings - specifically with XCB on Linux.

BGARGS="-DVK_USE_PLATFORM_XCB_KHR -lvulkan"

TGTS="vulkan vk_ext_debug_report vk_sdk_platform"

for tgt in $TGTS; do
    echo "$tgt"
    ./bindgen/target/release/bindgen $BGARGS -o src/ffi/gen/${tgt}.rs -match ${tgt}.h /usr/include/vulkan/${tgt}.h
done

