use std::process::Command;

fn main() {
    Command::new("bindgen").args(
        &[
            "-lvulkan-1",
            "-o", "src/vulkan.rs",
            "/usr/include/vulkan/vulkan.h",
        ]).status().unwrap();
}
