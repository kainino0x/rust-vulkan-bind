#![feature(raw)]

extern crate vulkan;

fn main() {
    use vulkan::ffi::*;
    use std::ptr::null;
    use std::raw::Repr;

    unsafe {
        let enabled_layers: [*const i8; 1] = [null()];
        let enabled_extensions: [*const i8; 1] = [null()];
        let create_info = VkInstanceCreateInfo {
            sType: Enum_VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            pApplicationInfo: null(),
            enabledLayerCount: 0,
            ppEnabledLayerNames: enabled_layers.repr().data,
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: enabled_extensions.repr().data,
        };
        let mut instance: VkInstance = std::mem::uninitialized();
        assert_eq!(Enum_VkResult::VK_SUCCESS,
                   vkCreateInstance(&create_info, null(), &mut instance));
    }
}
