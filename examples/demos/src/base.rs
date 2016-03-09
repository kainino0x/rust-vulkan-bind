// Modeled after:
// https://github.com/SaschaWillems/Vulkan/blob/master/base/vulkanexamplebase.cpp

use std::mem;
use std::os::raw::c_void;
use std::ptr::{null, null_mut};

use vulkan_bind::vk;
use xcb::ffi::base::*;
use xcb::ffi::xproto::*;
use cgmath::{Vector2, Vector3};

use tools;

pub struct Swapchain {
    queue_node_index: u32,
    // TODO
}

impl Swapchain {
    fn new(instance: vk::Instance,
           physical_device: vk::PhysicalDevice,
           device: vk::Device) -> Self{
        unimplemented!()
    }
}

pub struct TextureLoader; // TODO

pub struct DepthStencil {
    pub image: vk::Image,
    pub mem:   vk::DeviceMemory,
    pub view:  vk::ImageView,
}

fn vkrq(res: vk::Result, msg: &str) {
    if res != vk::Result::SUCCESS {
        println!("Fatal: {}: {:?}", msg, res);
        ::std::process::exit(1);
    }
}

fn vkrqr<T>(res: Result<T, vk::Result>, msg: &str) {
    if let Err(e) = res {
        vkrq(e, msg);
    }
}

fn vkassert(res: vk::Result) -> Result<(), vk::Result> {
    if res == vk::Result::SUCCESS { Ok(()) } else { Err(res) }
}

macro_rules! vktry {
    ( $res:expr ) => { try!(vkassert($res)) }
}

#[derive(Default)]
pub struct ExampleBase {
    enable_validation:           bool,
    pub frame_timer:             f32,
    pub instance:                Option<vk::Instance>,
    pub physical_device:         Option<vk::PhysicalDevice>,
    pub device_memory_props:     Option<vk::PhysicalDeviceMemoryProperties>,
    pub device:                  Option<vk::Device>,
    pub queue:                   Option<vk::Queue>,
    pub color_format:            Option<vk::Format>,
    pub cmd_pool:                Option<vk::CommandPool>,
    pub setup_cmd_buffer:        Option<vk::CommandBuffer>,
    pub post_present_cmd_buffer: Option<vk::CommandBuffer>,
    pub draw_cmd_buffers:        Vec<vk::CommandBuffer>,
    pub render_pass:             Option<vk::RenderPass>,
    pub framebuffers:            Vec<vk::Framebuffer>,
    pub current_buffer:          u32,
    pub shader_modules:          Vec<vk::ShaderModule>,
    pub pipeline_cache:          Option<vk::PipelineCache>,
    pub swapchain:               Option<Swapchain>,
    pub texture_loader:          Option<Box<TextureLoader>>,

    pub prepared:                bool,
    pub width:                   u32,
    pub height:                  u32,
    pub default_clear_color:     Option<vk::ClearColorValue>,
    pub zoom:                    f32,
    pub timer:                   f32,
    pub timer_speed:             f32,
    pub paused:                  bool,
    pub rotation_speed:          f32,
    pub zoom_speed:              f32,
    pub rotation:                Option<Vector3<f32>>,
    pub mouse_pos:               Option<Vector2<f32>>,
    pub title:                   Option<String>,
    pub name:                    Option<String>,
    pub depth_stencil:           Option<DepthStencil>,
    pub mouse_left:              bool,
    pub mouse_right:             bool,
    pub quit:                    bool,
    pub connection:              Option<*mut xcb_connection_t>,
    pub screen:                  Option<*mut xcb_screen_t>,
    pub window:                  Option<xcb_window_t>,
    pub atom_wm_delete_window:   Option<xcb_intern_atom_reply_t>,
}

impl ExampleBase {
    pub fn new() -> Self {
        Self::new_with_validation(false)
    }

    pub fn new_with_validation(enable_validation: bool) -> Self {
        let mut this = ExampleBase::default();
        this.init_xcb_connection();
        this.init_vulkan(enable_validation);
        this
    }

    pub fn create_instance(&mut self, enable_validation: bool) -> Result<(), vk::Result> {
        self.enable_validation = enable_validation;
        self.title = Some(String::from("Vulkan Example"));
        self.name = Some(String::from("vulkanExample"));
        let name_cstr = self.name.as_ref().unwrap().as_ptr() as *const i8;

        let app_info = vk::ApplicationInfo {
            sType: vk::StructureType::APPLICATION_INFO,
            pApplicationName: name_cstr,
            pEngineName: name_cstr,
            apiVersion: vk::make_version(1, 0, 2),
            applicationVersion: 0,
            engineVersion: 0,
            pNext: null(),
        };

        let mut enabled_extensions = vec![
            vk::khr::surface::EXTENSION_NAME.as_ptr() as *const i8,
            vk::khr::xcb_surface::EXTENSION_NAME.as_ptr() as *const i8,
        ];

        // TODO: validation

        let instance_create_info = vk::InstanceCreateInfo {
            sType: vk::StructureType::INSTANCE_CREATE_INFO,
            pNext: null(),
            pApplicationInfo: &app_info,
            enabledExtensionCount: enabled_extensions.len() as u32,
            ppEnabledExtensionNames: enabled_extensions.as_ptr(),
            enabledLayerCount: 0, // TODO: validation
            ppEnabledLayerNames: null(), // TODO: validation
            flags: Default::default(),
        };

        let instance = unsafe {
            let mut instance = mem::uninitialized();
            vktry!(vk::vkCreateInstance(&instance_create_info, null(), &mut instance));
            instance
        };

        self.instance = Some(instance);
        Ok(())
    }

    pub fn create_device(&mut self, requested_queues: vk::DeviceQueueCreateInfo,
                         enable_validation: bool) -> Result<(), vk::Result> {
        let enabled_extensions = vec![
            vk::khr::swapchain::EXTENSION_NAME.as_ptr() as *const i8,
        ];

        let device_create_info = vk::DeviceCreateInfo {
            sType: vk::StructureType::DEVICE_CREATE_INFO,
            pNext: null(),
            queueCreateInfoCount: 1,
            pQueueCreateInfos: &requested_queues,
            pEnabledFeatures: null(),
            enabledExtensionCount: enabled_extensions.len() as u32,
            ppEnabledExtensionNames:
                if enabled_extensions.len() == 0 { null() } else { enabled_extensions.as_ptr() },
            enabledLayerCount: 0,
            ppEnabledLayerNames: null(),
            flags: Default::default(),
        };

        let device = unsafe {
            let mut device = mem::uninitialized();
            vktry!(vk::vkCreateDevice(self.physical_device.unwrap(),
                                         &device_create_info, null(), &mut device));
            device
        };
        // TODO: validation

        Ok(())
    }

    fn init_vulkan(&mut self, enable_validation: bool) {
        let res = self.create_instance(enable_validation);
        vkrqr(res, "could not create instance");

        let mut gpu_count = 0u32;
        let res = unsafe {
            vk::vkEnumeratePhysicalDevices(self.instance.unwrap(), &mut gpu_count, null_mut())
        };
        vkrq(res, "could not enumerate physical devices");
        assert!(gpu_count > 0, "no physical devices found");

        let mut physical_devices = Vec::with_capacity(gpu_count as usize);
        unsafe {
            physical_devices.set_len(gpu_count as usize);
            let res = vk::vkEnumeratePhysicalDevices(self.instance.unwrap(), &mut gpu_count,
                                                     physical_devices.as_mut_ptr());
            vkrq(res, "could not enumerate physical devices");
        }

        let physical_device = physical_devices[0];

        let mut queue_count = 0u32;
        unsafe {
            vk::vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &mut queue_count,
                                                         null_mut());
        }
        assert!(queue_count >= 1, "could not query physical device queue family");

        let mut queue_props = Vec::with_capacity(queue_count as usize);
        unsafe {
            queue_props.set_len(queue_count as usize);
            vk::vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &mut queue_count,
                                                         queue_props.as_mut_ptr());
        }

        let graphics_queue_index = queue_props.iter()
            .position(|p| (p.queueFlags & vk::QueueFlag::GRAPHICS).0 != 0);
        assert!(graphics_queue_index.is_some(), "could not find graphics queue");
        let graphics_queue_index = graphics_queue_index.unwrap();

        let queue_priorities = [0f32];
        let queue_create_info = vk::DeviceQueueCreateInfo {
            sType: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            queueFamilyIndex: graphics_queue_index as u32,
            queueCount: 1,
            pQueuePriorities: &queue_priorities[0],
            flags: Default::default(),
            pNext: null(),
        };

        vkrqr(self.create_device(queue_create_info, enable_validation),
            "unable to create device/queue");

        let mut device_memory_props = unsafe { mem::uninitialized() };
        unsafe {
            vk::vkGetPhysicalDeviceMemoryProperties(physical_device, &mut device_memory_props);
        }
        self.device_memory_props = Some(device_memory_props);

        let mut queue = unsafe { mem::uninitialized() };
        unsafe {
            vk::vkGetDeviceQueue(self.device.unwrap(), graphics_queue_index as u32, 0, &mut queue);
        }
        self.queue = Some(queue);

        let depth_format = tools::get_supported_depth_format(physical_device).unwrap();

        self.swapchain = Some(Swapchain::new(self.instance.unwrap(),
                                             physical_device, self.device.unwrap()));
    }

    fn setup_window(&mut self) -> xcb_window_t {
        unimplemented!()
    }

    fn init_xcb_connection(&mut self) {
        let mut scr = 0i32;
        let connection = unsafe { xcb_connect(null_mut(), &mut scr) };
        assert!(connection != null_mut(), "could not find compatible Vulkan ICD");

        let setup = unsafe { xcb_get_setup(connection) };
        let mut iter = unsafe { xcb_setup_roots_iterator(setup) };
        while scr > 0 {
            unsafe { xcb_screen_next(&mut iter) };
            scr -= 1;
        }

        self.screen = Some(iter.data);
    }

    fn handle_event(&mut self, event: *const xcb_generic_event_t) {
        unimplemented!()
    }

    fn get_memory_type<F>(&mut self, type_bits: u32, properties: F, type_index: &[u32; 32])
        -> vk::Bool32
        where F: Into<vk::MemoryPropertyFlags> {
        unimplemented!()
    }

    fn create_command_pool(&mut self) {
        let cmd_pool_info = vk::CommandPoolCreateInfo {
            sType: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            queueFamilyIndex: self.swapchain.unwrap().queue_node_index,
            flags: vk::CommandPoolCreateFlag::RESET_COMMAND_BUFFER.into(),
            pNext: null(),
        };

        let cmd_pool = unsafe {
            let mut cmd_pool = unsafe { mem::uninitialized() };
            assert_eq!(vk::Result::SUCCESS,
                       vk::vkCreateCommandPool(self.device.unwrap(), &cmd_pool_info, null(), cmd_pool));
            cmd_pool
        };

        self.cmd_pool = Some(unsafe { *cmd_pool });
    }

    fn setup_depth_stencil(&mut self) {
        unimplemented!()
    }

    fn setup_framebuffer(&mut self) {
        unimplemented!()
    }

    fn setup_render_pass(&mut self) {
        unimplemented!()
    }

    fn init_swapchain(&mut self) {
        unimplemented!()
    }

    fn setup_swapchain(&mut self) {
        unimplemented!()
    }

    fn check_command_buffers(&mut self) -> bool {
        for &cmd_buffer in &self.draw_cmd_buffers {
            if cmd_buffer == null_mut() {
                return false;
            }
        }
        true
    }

    fn create_command_buffers(&mut self) {
        unimplemented!()
    }

    fn destroy_command_buffers(&mut self) {
        unimplemented!()
    }

    fn create_setup_command_buffer(&mut self) {
        unimplemented!()
    }

    fn flush_setup_command_buffer(&mut self) {
        unimplemented!()
    }

    fn create_pipeline_cache(&mut self) {
        unimplemented!()
    }

    fn prepare(&mut self) {
        unimplemented!()
    }

    fn load_shader(&mut self, filename: &str,
                   stage: vk::ShaderStageFlag) -> vk::PipelineShaderStageCreateInfo {
        unimplemented!()
    }

    fn load_shader_glsl(&mut self, filename: &str,
                        stage: vk::ShaderStageFlag) -> vk::PipelineShaderStageCreateInfo {
        unimplemented!()
    }

    fn create_buffer<F>(&mut self,
                        usage: F,
                        size: vk::DeviceSize,
                        data: &mut c_void,
                        buffer: &mut vk::Buffer,
                        memory: &mut vk::DeviceMemory) -> vk::Bool32
                        where F: Into<vk::BufferUsageFlags> {
        unimplemented!()
    }

    fn create_buffer_with_descriptor<F>(&mut self,
                                        usage: F,
                                        size: vk::DeviceSize,
                                        data: &mut c_void,
                                        buffer: &mut vk::Buffer,
                                        memory: &mut vk::DeviceMemory,
                                        descriptor: &mut vk::DescriptorBufferInfo) -> vk::Bool32
                                        where F: Into<vk::BufferUsageFlags> {
        unimplemented!()
    }

    fn load_mesh(&mut self,
                 filename: &str,
                 mesh_buffer: &mut ::mesh::Buffer,
                 vertex_layout: &mut Vec<::mesh::VertexLayout>,
                 scale: f32) {
        unimplemented!()
    }

    fn render_loop(&mut self) {
        unimplemented!()
    }

    fn submit_post_present_barrier(image: vk::Image) {
        unimplemented!()
    }
}

impl Drop for ExampleBase {
    fn drop(&mut self) {
        unimplemented!()
    }
}

pub trait Example {
    fn base(&mut self) -> &mut ExampleBase;
    fn render(&mut self);
    fn view_changed(&mut self);
}
