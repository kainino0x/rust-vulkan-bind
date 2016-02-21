#![feature(raw)]

extern crate vulkan_bind;

fn main() {
    use vulkan_bind::vk;
    use std::ptr::null;
    use std::raw::Repr;

    unsafe {
        let enabled_layers: [*const i8; 1] = [null()];
        let enabled_extensions: [*const i8; 1] = [null()];
        let create_info = vk::InstanceCreateInfo {
            sType: vk::StructureType::INSTANCE_CREATE_INFO,
            pNext: null(),
            flags: Default::default(),
            pApplicationInfo: null(),
            enabledLayerCount: 0,
            ppEnabledLayerNames: enabled_layers.repr().data,
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: enabled_extensions.repr().data,
        };
        let mut instance: vk::Instance = std::mem::uninitialized();
        assert_eq!(vk::Result::SUCCESS,
                   vk::vkCreateInstance(&create_info, null(), &mut instance));
    }
}
