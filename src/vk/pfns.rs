use vk::*;

pub type PFN_vkAllocationFunction =
    ::std::option::Option<unsafe extern "C" fn(pUserData: *mut ::std::os::raw::c_void,
                                               size: usize,
                                               alignment: usize,
                                               allocationScope: SystemAllocationScope)
                              -> *mut ::std::os::raw::c_void>;
pub type PFN_vkReallocationFunction =
    ::std::option::Option<unsafe extern "C" fn(pUserData: *mut ::std::os::raw::c_void,
                                               pOriginal: *mut ::std::os::raw::c_void,
                                               size: usize,
                                               alignment: usize,
                                               allocationScope: SystemAllocationScope)
                              -> *mut ::std::os::raw::c_void>;
pub type PFN_vkFreeFunction =
    ::std::option::Option<unsafe extern "C" fn(pUserData: *mut ::std::os::raw::c_void,
                                               pMemory: *mut ::std::os::raw::c_void)>;
pub type PFN_vkInternalAllocationNotification =
    ::std::option::Option<unsafe extern "C" fn(pUserData: *mut ::std::os::raw::c_void,
                                               size: usize,
                                               allocationType: InternalAllocationType,
                                               allocationScope: SystemAllocationScope)>;
pub type PFN_vkInternalFreeNotification =
    ::std::option::Option<unsafe extern "C" fn(pUserData: *mut ::std::os::raw::c_void,
                                               size: usize,
                                               allocationType: InternalAllocationType,
                                               allocationScope: SystemAllocationScope)>;
pub type PFN_vkVoidFunction = ::std::option::Option<extern "C" fn()>;
pub type PFN_vkCreateInstance =
    ::std::option::Option<unsafe extern "C" fn(pCreateInfo: *const InstanceCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pInstance: *mut Instance)
                              -> Result>;
pub type PFN_vkDestroyInstance =
    ::std::option::Option<unsafe extern "C" fn(instance: Instance,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkEnumeratePhysicalDevices =
    ::std::option::Option<unsafe extern "C" fn(instance: Instance,
                                               pPhysicalDeviceCount: *mut u32,
                                               pPhysicalDevices: *mut PhysicalDevice)
                              -> Result>;
pub type PFN_vkGetPhysicalDeviceFeatures =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               pFeatures: *mut PhysicalDeviceFeatures)>;
pub type PFN_vkGetPhysicalDeviceFormatProperties =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               format: Format,
                                               pFormatProperties: *mut FormatProperties)>;
pub type PFN_vkGetPhysicalDeviceImageFormatProperties =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               format: Format,
                                               _type: ImageType,
                                               tiling: ImageTiling,
                                               usage: ImageUsageFlags,
                                               flags: ImageCreateFlags,
                                               pImageFormatProperties: *mut ImageFormatProperties)
                              -> Result>;
pub type PFN_vkGetPhysicalDeviceProperties =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               pProperties: *mut PhysicalDeviceProperties)>;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               pQueueFamilyPropertyCount: *mut u32,
                                               pQueueFamilyProperties: *mut QueueFamilyProperties)>;
pub type PFN_vkGetPhysicalDeviceMemoryProperties =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               pMemoryProperties: *mut PhysicalDeviceMemoryProperties)>;
pub type PFN_vkGetInstanceProcAddr =
    ::std::option::Option<unsafe extern "C" fn(instance: Instance,
                                               pName: *const ::std::os::raw::c_char)
                              -> PFN_vkVoidFunction>;
pub type PFN_vkGetDeviceProcAddr =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pName: *const ::std::os::raw::c_char)
                              -> PFN_vkVoidFunction>;
pub type PFN_vkCreateDevice =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               pCreateInfo: *const DeviceCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pDevice: *mut Device)
                              -> Result>;
pub type PFN_vkDestroyDevice =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkEnumerateInstanceExtensionProperties =
    ::std::option::Option<unsafe extern "C" fn(pLayerName: *const ::std::os::raw::c_char,
                                               pPropertyCount: *mut u32,
                                               pProperties: *mut ExtensionProperties)
                              -> Result>;
pub type PFN_vkEnumerateDeviceExtensionProperties =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               pLayerName: *const ::std::os::raw::c_char,
                                               pPropertyCount: *mut u32,
                                               pProperties: *mut ExtensionProperties)
                              -> Result>;
pub type PFN_vkEnumerateInstanceLayerProperties =
    ::std::option::Option<unsafe extern "C" fn(pPropertyCount: *mut u32,
                                               pProperties: *mut LayerProperties)
                              -> Result>;
pub type PFN_vkEnumerateDeviceLayerProperties =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               pPropertyCount: *mut u32,
                                               pProperties: *mut LayerProperties)
                              -> Result>;
pub type PFN_vkGetDeviceQueue =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               queueFamilyIndex: u32,
                                               queueIndex: u32,
                                               pQueue: *mut Queue)>;
pub type PFN_vkQueueSubmit =
    ::std::option::Option<unsafe extern "C" fn(queue: Queue,
                                               submitCount: u32,
                                               pSubmits: *const SubmitInfo,
                                               fence: Fence) -> Result>;
pub type PFN_vkQueueWaitIdle =
    ::std::option::Option<extern "C" fn(queue: Queue) -> Result>;
pub type PFN_vkDeviceWaitIdle =
    ::std::option::Option<extern "C" fn(device: Device) -> Result>;
pub type PFN_vkAllocateMemory =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pAllocateInfo: *const MemoryAllocateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pMemory: *mut DeviceMemory)
                              -> Result>;
pub type PFN_vkFreeMemory =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               memory: DeviceMemory,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkMapMemory =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               memory: DeviceMemory,
                                               offset: DeviceSize,
                                               size: DeviceSize,
                                               flags: MemoryMapFlags,
                                               ppData: *mut *mut ::std::os::raw::c_void)
                              -> Result>;
pub type PFN_vkUnmapMemory =
    ::std::option::Option<extern "C" fn(device: Device,
                                        memory: DeviceMemory)>;
pub type PFN_vkFlushMappedMemoryRanges =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               memoryRangeCount: u32,
                                               pMemoryRanges: *const MappedMemoryRange)
                              -> Result>;
pub type PFN_vkInvalidateMappedMemoryRanges =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               memoryRangeCount: u32,
                                               pMemoryRanges: *const MappedMemoryRange)
                              -> Result>;
pub type PFN_vkGetDeviceMemoryCommitment =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               memory: DeviceMemory,
                                               pCommittedMemoryInBytes: *mut DeviceSize)>;
pub type PFN_vkBindBufferMemory =
    ::std::option::Option<extern "C" fn(device: Device, buffer: Buffer,
                                        memory: DeviceMemory,
                                        memoryOffset: DeviceSize)
                              -> Result>;
pub type PFN_vkBindImageMemory =
    ::std::option::Option<extern "C" fn(device: Device, image: Image,
                                        memory: DeviceMemory,
                                        memoryOffset: DeviceSize)
                              -> Result>;
pub type PFN_vkGetBufferMemoryRequirements =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               buffer: Buffer,
                                               pMemoryRequirements: *mut MemoryRequirements)>;
pub type PFN_vkGetImageMemoryRequirements =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               image: Image,
                                               pMemoryRequirements: *mut MemoryRequirements)>;
pub type PFN_vkGetImageSparseMemoryRequirements =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               image: Image,
                                               pSparseMemoryRequirementCount: *mut u32,
                                               pSparseMemoryRequirements: *mut SparseImageMemoryRequirements)>;
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               format: Format,
                                               _type: ImageType,
                                               samples: SampleCountFlag,
                                               usage: ImageUsageFlags,
                                               tiling: ImageTiling,
                                               pPropertyCount: *mut u32,
                                               pProperties: *mut SparseImageFormatProperties)>;
pub type PFN_vkQueueBindSparse =
    ::std::option::Option<unsafe extern "C" fn(queue: Queue,
                                               bindInfoCount: u32,
                                               pBindInfo: *const BindSparseInfo,
                                               fence: Fence) -> Result>;
pub type PFN_vkCreateFence =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const FenceCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pFence: *mut Fence)
                              -> Result>;
pub type PFN_vkDestroyFence =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               fence: Fence,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkResetFences =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               fenceCount: u32,
                                               pFences: *const Fence)
                              -> Result>;
pub type PFN_vkGetFenceStatus =
    ::std::option::Option<extern "C" fn(device: Device, fence: Fence)
                              -> Result>;
pub type PFN_vkWaitForFences =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               fenceCount: u32,
                                               pFences: *const Fence,
                                               waitAll: Bool32,
                                               timeout: u64)
                              -> Result>;
pub type PFN_vkCreateSemaphore =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const SemaphoreCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pSemaphore: *mut Semaphore)
                              -> Result>;
pub type PFN_vkDestroySemaphore =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               semaphore: Semaphore,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkCreateEvent =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const EventCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pEvent: *mut Event)
                              -> Result>;
pub type PFN_vkDestroyEvent =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               event: Event,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkGetEventStatus =
    ::std::option::Option<extern "C" fn(device: Device, event: Event)
                              -> Result>;
pub type PFN_vkSetEvent =
    ::std::option::Option<extern "C" fn(device: Device, event: Event)
                              -> Result>;
pub type PFN_vkResetEvent =
    ::std::option::Option<extern "C" fn(device: Device, event: Event)
                              -> Result>;
pub type PFN_vkCreateQueryPool =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const QueryPoolCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pQueryPool: *mut QueryPool)
                              -> Result>;
pub type PFN_vkDestroyQueryPool =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               queryPool: QueryPool,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkGetQueryPoolResults =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               queryPool: QueryPool,
                                               firstQuery: u32,
                                               queryCount: u32,
                                               dataSize: usize,
                                               pData: *mut ::std::os::raw::c_void,
                                               stride: DeviceSize,
                                               flags: QueryResultFlags)
                              -> Result>;
pub type PFN_vkCreateBuffer =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const BufferCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pBuffer: *mut Buffer)
                              -> Result>;
pub type PFN_vkDestroyBuffer =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               buffer: Buffer,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkCreateBufferView =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const BufferViewCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pView: *mut BufferView)
                              -> Result>;
pub type PFN_vkDestroyBufferView =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               bufferView: BufferView,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkCreateImage =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const ImageCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pImage: *mut Image)
                              -> Result>;
pub type PFN_vkDestroyImage =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               image: Image,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkGetImageSubresourceLayout =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               image: Image,
                                               pSubresource: *const ImageSubresource,
                                               pLayout: *mut SubresourceLayout)>;
pub type PFN_vkCreateImageView =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const ImageViewCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pView: *mut ImageView)
                              -> Result>;
pub type PFN_vkDestroyImageView =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               imageView: ImageView,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkCreateShaderModule =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const ShaderModuleCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pShaderModule: *mut ShaderModule)
                              -> Result>;
pub type PFN_vkDestroyShaderModule =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               shaderModule: ShaderModule,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkCreatePipelineCache =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const PipelineCacheCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pPipelineCache: *mut PipelineCache)
                              -> Result>;
pub type PFN_vkDestroyPipelineCache =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pipelineCache: PipelineCache,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkGetPipelineCacheData =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pipelineCache: PipelineCache,
                                               pDataSize: *mut usize,
                                               pData: *mut ::std::os::raw::c_void)
                              -> Result>;
pub type PFN_vkMergePipelineCaches =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               dstCache: PipelineCache,
                                               srcCacheCount: u32,
                                               pSrcCaches: *const PipelineCache)
                              -> Result>;
pub type PFN_vkCreateGraphicsPipelines =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pipelineCache: PipelineCache,
                                               createInfoCount: u32,
                                               pCreateInfos: *const GraphicsPipelineCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pPipelines: *mut Pipeline)
                              -> Result>;
pub type PFN_vkCreateComputePipelines =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pipelineCache: PipelineCache,
                                               createInfoCount: u32,
                                               pCreateInfos: *const ComputePipelineCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pPipelines: *mut Pipeline)
                              -> Result>;
pub type PFN_vkDestroyPipeline =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pipeline: Pipeline,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkCreatePipelineLayout =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const PipelineLayoutCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pPipelineLayout: *mut PipelineLayout)
                              -> Result>;
pub type PFN_vkDestroyPipelineLayout =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pipelineLayout: PipelineLayout,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkCreateSampler =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const SamplerCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pSampler: *mut Sampler)
                              -> Result>;
pub type PFN_vkDestroySampler =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               sampler: Sampler,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkCreateDescriptorSetLayout =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const DescriptorSetLayoutCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pSetLayout: *mut DescriptorSetLayout)
                              -> Result>;
pub type PFN_vkDestroyDescriptorSetLayout =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               descriptorSetLayout: DescriptorSetLayout,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkCreateDescriptorPool =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const DescriptorPoolCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pDescriptorPool: *mut DescriptorPool)
                              -> Result>;
pub type PFN_vkDestroyDescriptorPool =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               descriptorPool: DescriptorPool,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkResetDescriptorPool =
    ::std::option::Option<extern "C" fn(device: Device,
                                        descriptorPool: DescriptorPool,
                                        flags: DescriptorPoolResetFlags)
                              -> Result>;
pub type PFN_vkAllocateDescriptorSets =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pAllocateInfo: *const DescriptorSetAllocateInfo,
                                               pDescriptorSets: *mut DescriptorSet)
                              -> Result>;
pub type PFN_vkFreeDescriptorSets =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               descriptorPool: DescriptorPool,
                                               descriptorSetCount: u32,
                                               pDescriptorSets: *const DescriptorSet)
                              -> Result>;
pub type PFN_vkUpdateDescriptorSets =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               descriptorWriteCount: u32,
                                               pDescriptorWrites: *const WriteDescriptorSet,
                                               descriptorCopyCount: u32,
                                               pDescriptorCopies: *const CopyDescriptorSet)>;
pub type PFN_vkCreateFramebuffer =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const FramebufferCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pFramebuffer: *mut Framebuffer)
                              -> Result>;
pub type PFN_vkDestroyFramebuffer =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               framebuffer: Framebuffer,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkCreateRenderPass =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const RenderPassCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pRenderPass: *mut RenderPass)
                              -> Result>;
pub type PFN_vkDestroyRenderPass =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               renderPass: RenderPass,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkGetRenderAreaGranularity =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               renderPass: RenderPass,
                                               pGranularity: *mut Extent2D)>;
pub type PFN_vkCreateCommandPool =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const CommandPoolCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pCommandPool: *mut CommandPool)
                              -> Result>;
pub type PFN_vkDestroyCommandPool =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               commandPool: CommandPool,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkResetCommandPool =
    ::std::option::Option<extern "C" fn(device: Device,
                                        commandPool: CommandPool,
                                        flags: CommandPoolResetFlags)
                              -> Result>;
pub type PFN_vkAllocateCommandBuffers =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pAllocateInfo: *const CommandBufferAllocateInfo,
                                               pCommandBuffers: *mut CommandBuffer)
                              -> Result>;
pub type PFN_vkFreeCommandBuffers =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               commandPool: CommandPool,
                                               commandBufferCount: u32,
                                               pCommandBuffers: *const CommandBuffer)>;
pub type PFN_vkBeginCommandBuffer =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               pBeginInfo: *const CommandBufferBeginInfo)
                              -> Result>;
pub type PFN_vkEndCommandBuffer =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer)
                              -> Result>;
pub type PFN_vkResetCommandBuffer =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        flags: CommandBufferResetFlags)
                              -> Result>;
pub type PFN_vkCmdBindPipeline =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        pipelineBindPoint: PipelineBindPoint,
                                        pipeline: Pipeline)>;
pub type PFN_vkCmdSetViewport =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               firstViewport: u32,
                                               viewportCount: u32,
                                               pViewports: *const Viewport)>;
pub type PFN_vkCmdSetScissor =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               firstScissor: u32,
                                               scissorCount: u32,
                                               pScissors: *const Rect2D)>;
pub type PFN_vkCmdSetLineWidth =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        lineWidth: ::std::os::raw::c_float)>;
pub type PFN_vkCmdSetDepthBias =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        depthBiasConstantFactor: ::std::os::raw::c_float,
                                        depthBiasClamp: ::std::os::raw::c_float,
                                        depthBiasSlopeFactor: ::std::os::raw::c_float)>;
pub type PFN_vkCmdSetBlendConstants =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        blendConstants: *mut ::std::os::raw::c_float)>;
pub type PFN_vkCmdSetDepthBounds =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        minDepthBounds: ::std::os::raw::c_float,
                                        maxDepthBounds: ::std::os::raw::c_float)>;
pub type PFN_vkCmdSetStencilCompareMask =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        faceMask: StencilFaceFlags,
                                        compareMask: u32)>;
pub type PFN_vkCmdSetStencilWriteMask =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        faceMask: StencilFaceFlags,
                                        writeMask: u32)>;
pub type PFN_vkCmdSetStencilReference =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        faceMask: StencilFaceFlags,
                                        reference: u32)>;
pub type PFN_vkCmdBindDescriptorSets =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               pipelineBindPoint: PipelineBindPoint,
                                               layout: PipelineLayout,
                                               firstSet: u32,
                                               descriptorSetCount: u32,
                                               pDescriptorSets: *const DescriptorSet,
                                               dynamicOffsetCount: u32,
                                               pDynamicOffsets: *const u32)>;
pub type PFN_vkCmdBindIndexBuffer =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        buffer: Buffer,
                                        offset: DeviceSize,
                                        indexType: IndexType)>;
pub type PFN_vkCmdBindVertexBuffers =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               firstBinding: u32,
                                               bindingCount: u32,
                                               pBuffers: *const Buffer,
                                               pOffsets: *const DeviceSize)>;
pub type PFN_vkCmdDraw =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        vertexCount: u32,
                                        instanceCount: u32,
                                        firstVertex: u32,
                                        firstInstance: u32)>;
pub type PFN_vkCmdDrawIndexed =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        indexCount: u32,
                                        instanceCount: u32,
                                        firstIndex: u32,
                                        vertexOffset: i32,
                                        firstInstance: u32)>;
pub type PFN_vkCmdDrawIndirect =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        buffer: Buffer,
                                        offset: DeviceSize,
                                        drawCount: u32,
                                        stride: u32)>;
pub type PFN_vkCmdDrawIndexedIndirect =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        buffer: Buffer,
                                        offset: DeviceSize,
                                        drawCount: u32,
                                        stride: u32)>;
pub type PFN_vkCmdDispatch =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        x: u32, y: u32,
                                        z: u32)>;
pub type PFN_vkCmdDispatchIndirect =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        buffer: Buffer,
                                        offset: DeviceSize)>;
pub type PFN_vkCmdCopyBuffer =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               srcBuffer: Buffer,
                                               dstBuffer: Buffer,
                                               regionCount: u32,
                                               pRegions: *const BufferCopy)>;
pub type PFN_vkCmdCopyImage =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               srcImage: Image,
                                               srcImageLayout: ImageLayout,
                                               dstImage: Image,
                                               dstImageLayout: ImageLayout,
                                               regionCount: u32,
                                               pRegions: *const ImageCopy)>;
pub type PFN_vkCmdBlitImage =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               srcImage: Image,
                                               srcImageLayout: ImageLayout,
                                               dstImage: Image,
                                               dstImageLayout: ImageLayout,
                                               regionCount: u32,
                                               pRegions: *const ImageBlit,
                                               filter: Filter)>;
pub type PFN_vkCmdCopyBufferToImage =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               srcBuffer: Buffer,
                                               dstImage: Image,
                                               dstImageLayout: ImageLayout,
                                               regionCount: u32,
                                               pRegions: *const BufferImageCopy)>;
pub type PFN_vkCmdCopyImageToBuffer =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               srcImage: Image,
                                               srcImageLayout: ImageLayout,
                                               dstBuffer: Buffer,
                                               regionCount: u32,
                                               pRegions: *const BufferImageCopy)>;
pub type PFN_vkCmdUpdateBuffer =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               dstBuffer: Buffer,
                                               dstOffset: DeviceSize,
                                               dataSize: DeviceSize,
                                               pData: *const u32)>;
pub type PFN_vkCmdFillBuffer =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        dstBuffer: Buffer,
                                        dstOffset: DeviceSize,
                                        size: DeviceSize, data: u32)>;
pub type PFN_vkCmdClearColorImage =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               image: Image,
                                               imageLayout: ImageLayout,
                                               pColor: *const ClearColorValue,
                                               rangeCount: u32,
                                               pRanges: *const ImageSubresourceRange)>;
pub type PFN_vkCmdClearDepthStencilImage =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               image: Image,
                                               imageLayout: ImageLayout,
                                               pDepthStencil: *const ClearDepthStencilValue,
                                               rangeCount: u32,
                                               pRanges: *const ImageSubresourceRange)>;
pub type PFN_vkCmdClearAttachments =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               attachmentCount: u32,
                                               pAttachments: *const ClearAttachment,
                                               rectCount: u32,
                                               pRects: *const ClearRect)>;
pub type PFN_vkCmdResolveImage =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               srcImage: Image,
                                               srcImageLayout: ImageLayout,
                                               dstImage: Image,
                                               dstImageLayout: ImageLayout,
                                               regionCount: u32,
                                               pRegions: *const ImageResolve)>;
pub type PFN_vkCmdSetEvent =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        event: Event,
                                        stageMask: PipelineStageFlags)>;
pub type PFN_vkCmdResetEvent =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        event: Event,
                                        stageMask: PipelineStageFlags)>;
pub type PFN_vkCmdWaitEvents =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               eventCount: u32,
                                               pEvents: *const Event,
                                               srcStageMask: PipelineStageFlags,
                                               dstStageMask: PipelineStageFlags,
                                               memoryBarrierCount: u32,
                                               pMemoryBarriers: *const MemoryBarrier,
                                               bufferMemoryBarrierCount: u32,
                                               pBufferMemoryBarriers: *const BufferMemoryBarrier,
                                               imageMemoryBarrierCount: u32,
                                               pImageMemoryBarriers: *const ImageMemoryBarrier)>;
pub type PFN_vkCmdPipelineBarrier =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               srcStageMask: PipelineStageFlags,
                                               dstStageMask: PipelineStageFlags,
                                               dependencyFlags: DependencyFlags,
                                               memoryBarrierCount: u32,
                                               pMemoryBarriers: *const MemoryBarrier,
                                               bufferMemoryBarrierCount: u32,
                                               pBufferMemoryBarriers: *const BufferMemoryBarrier,
                                               imageMemoryBarrierCount: u32,
                                               pImageMemoryBarriers: *const ImageMemoryBarrier)>;
pub type PFN_vkCmdBeginQuery =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        queryPool: QueryPool,
                                        query: u32,
                                        flags: QueryControlFlags)>;
pub type PFN_vkCmdEndQuery =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        queryPool: QueryPool,
                                        query: u32)>;
pub type PFN_vkCmdResetQueryPool =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        queryPool: QueryPool,
                                        firstQuery: u32,
                                        queryCount: u32)>;
pub type PFN_vkCmdWriteTimestamp =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        pipelineStage: PipelineStageFlag,
                                        queryPool: QueryPool,
                                        query: u32)>;
pub type PFN_vkCmdCopyQueryPoolResults =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        queryPool: QueryPool,
                                        firstQuery: u32,
                                        queryCount: u32,
                                        dstBuffer: Buffer,
                                        dstOffset: DeviceSize,
                                        stride: DeviceSize,
                                        flags: QueryResultFlags)>;
pub type PFN_vkCmdPushConstants =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               layout: PipelineLayout,
                                               stageFlags: ShaderStageFlags,
                                               offset: u32,
                                               size: u32,
                                               pValues: *const ::std::os::raw::c_void)>;
pub type PFN_vkCmdBeginRenderPass =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               pRenderPassBegin: *const RenderPassBeginInfo,
                                               contents: SubpassContents)>;
pub type PFN_vkCmdNextSubpass =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer,
                                        contents: SubpassContents)>;
pub type PFN_vkCmdEndRenderPass =
    ::std::option::Option<extern "C" fn(commandBuffer: CommandBuffer)>;
pub type PFN_vkCmdExecuteCommands =
    ::std::option::Option<unsafe extern "C" fn(commandBuffer: CommandBuffer,
                                               commandBufferCount: u32,
                                               pCommandBuffers: *const CommandBuffer)>;
