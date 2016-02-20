type Flags = u32;
type Bool32 = u32;
type DeviceSize = u64;
type SampleMask = u32;

macro_rules! newtypes {
    ($(typedef $from:ident $to:ident;)*) => {
        $(pub struct $to($from);)*
    }
}

newtypes!{
    typedef Flags InstanceCreateFlags;

    typedef Flags FormatFeatureFlags;

    typedef Flags ImageUsageFlags;

    typedef Flags ImageCreateFlags;

    typedef Flags SampleCountFlags;

    typedef Flags QueueFlags;

    typedef Flags MemoryPropertyFlags;

    typedef Flags MemoryHeapFlags;
    typedef Flags DeviceCreateFlags;
    typedef Flags DeviceQueueCreateFlags;

    typedef Flags PipelineStageFlags;
    typedef Flags MemoryMapFlags;

    typedef Flags ImageAspectFlags;

    typedef Flags SparseImageFormatFlags;

    typedef Flags SparseMemoryBindFlags;

    typedef Flags FenceCreateFlags;
    typedef Flags SemaphoreCreateFlags;
    typedef Flags EventCreateFlags;
    typedef Flags QueryPoolCreateFlags;

    typedef Flags QueryPipelineStatisticFlags;

    typedef Flags QueryResultFlags;

    typedef Flags BufferCreateFlags;

    typedef Flags BufferUsageFlags;
    typedef Flags BufferViewCreateFlags;
    typedef Flags ImageViewCreateFlags;
    typedef Flags ShaderModuleCreateFlags;
    typedef Flags PipelineCacheCreateFlags;

    typedef Flags PipelineCreateFlags;
    typedef Flags PipelineShaderStageCreateFlags;

    typedef Flags PipelineVertexInputStateCreateFlags;
    typedef Flags PipelineInputAssemblyStateCreateFlags;
    typedef Flags PipelineTessellationStateCreateFlags;
    typedef Flags PipelineViewportStateCreateFlags;
    typedef Flags PipelineRasterizationStateCreateFlags;

    typedef Flags CullModeFlags;
    typedef Flags PipelineMultisampleStateCreateFlags;
    typedef Flags PipelineDepthStencilStateCreateFlags;
    typedef Flags PipelineColorBlendStateCreateFlags;

    typedef Flags ColorComponentFlags;
    typedef Flags PipelineDynamicStateCreateFlags;
    typedef Flags PipelineLayoutCreateFlags;
    typedef Flags ShaderStageFlags;
    typedef Flags SamplerCreateFlags;
    typedef Flags DescriptorSetLayoutCreateFlags;

    typedef Flags DescriptorPoolCreateFlags;
    typedef Flags DescriptorPoolResetFlags;
    typedef Flags FramebufferCreateFlags;
    typedef Flags RenderPassCreateFlags;

    typedef Flags AttachmentDescriptionFlags;
    typedef Flags SubpassDescriptionFlags;

    typedef Flags AccessFlags;

    typedef Flags DependencyFlags;

    typedef Flags CommandPoolCreateFlags;

    typedef Flags CommandPoolResetFlags;

    typedef Flags CommandBufferUsageFlags;

    typedef Flags QueryControlFlags;

    typedef Flags CommandBufferResetFlags;

    typedef Flags StencilFaceFlags;
}
