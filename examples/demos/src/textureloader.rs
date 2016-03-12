use vulkan_bind::vk;

pub struct TextureLoader; // TODO

impl TextureLoader {
    pub fn new(physical_device: vk::PhysicalDevice, device: vk::Device, queue: vk::Queue, cmd_pool: vk::CommandPool) -> Self {
        unimplemented!();
    }
}
