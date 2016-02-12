use std::process::Command;

fn main() {
    Command::new("./bindgen/target/release/bindgen").args(
        &[
            "-lvulkan",
            "-o", "src/vulkan.rs",
            "/usr/include/vulkan/vulkan.h",
        ]).status().unwrap();
}
