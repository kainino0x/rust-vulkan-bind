use std::process::Command;

fn main() {
    Command::new("./bindgen/target/release/bindgen").args(
        &[
            "-lvulkan",
            "-DVK_USE_PLATFORM_XCB_KHR",
            "-o", "src/ffi/generated.rs",
            "-match", "vulkan.h",
            "/usr/include/vulkan/vulkan.h",
        ]).status().unwrap();
}
