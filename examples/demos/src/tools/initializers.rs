#![allow(non_snake_case)]

use vulkan_bind::vk;
use std::ptr::{null, null_mut};

fn df<T: Default>() -> T {
    Default::default()
}

pub fn memoryAllocateInfo() -> vk::MemoryAllocateInfo {
    vk::MemoryAllocateInfo {
        sType: vk::StructureType::MEMORY_ALLOCATE_INFO,
        pNext: null(),
        allocationSize: 0,
        memoryTypeIndex: 0,
    }
}

pub fn commandBufferAllocateInfo(commandPool: vk::CommandPool, level: vk::CommandBufferLevel, bufferCount: u32) -> vk::CommandBufferAllocateInfo {
    vk::CommandBufferAllocateInfo {
        sType: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        commandPool: commandPool,
        level: level,
        commandBufferCount: bufferCount,
        pNext: null(),
    }
}

pub fn commandBufferBeginInfo() -> vk::CommandBufferBeginInfo {
    vk::CommandBufferBeginInfo {
        sType: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
        pNext: null(),
        flags: df(),
        pInheritanceInfo: null(),
    }
}

pub fn renderPassBeginInfo() -> vk::RenderPassBeginInfo {
    vk::RenderPassBeginInfo {
        sType: vk::StructureType::RENDER_PASS_BEGIN_INFO,
        pNext: null(),
        // Fields that were omitted in C++:
        renderPass: null_mut(),
        framebuffer: null_mut(),
        renderArea: df(),
        clearValueCount: 0,
        pClearValues: null(),
    }
}

pub fn imageMemoryBarrier() -> vk::ImageMemoryBarrier {
    vk::ImageMemoryBarrier {
        sType: vk::StructureType::IMAGE_MEMORY_BARRIER,
        pNext: null(),
        // Some default values
        srcQueueFamilyIndex: vk::QUEUE_FAMILY_IGNORED,
        dstQueueFamilyIndex: vk::QUEUE_FAMILY_IGNORED,
        // Fields that were omitted in C++:
        srcAccessMask: df(),
        dstAccessMask: df(),
        oldLayout: vk::ImageLayout::UNDEFINED,
        newLayout: vk::ImageLayout::UNDEFINED,
        image: null_mut(),
        subresourceRange: df(),
    }
}

pub fn bufferMemoryBarrier() -> vk::BufferMemoryBarrier {
    vk::BufferMemoryBarrier {
        sType: vk::StructureType::BUFFER_MEMORY_BARRIER,
        pNext: null(),
        // Fields that were omitted in C++:
        srcAccessMask: df(),
        dstAccessMask: df(),
        srcQueueFamilyIndex: 0,
        dstQueueFamilyIndex: 0,
        buffer: null_mut(),
        offset: df(),
        size: df(),
    }
}

pub fn memoryBarrier() -> vk::MemoryBarrier {
    vk::MemoryBarrier {
        sType: vk::StructureType::MEMORY_BARRIER,
        pNext: null(),
        // Fields that were omitted in C++:
        srcAccessMask: df(),
        dstAccessMask: df(),
    }
}

pub fn imageCreateInfo() -> vk::ImageCreateInfo {
    vk::ImageCreateInfo {
        sType: vk::StructureType::IMAGE_CREATE_INFO,
        pNext: null(),
        // Fields that were omitted in C++:
        flags: df(),
        imageType: vk::ImageType::E_1D,
        format: vk::Format::UNDEFINED,
        extent: df(),
        mipLevels: 0,
        arrayLayers: 0,
        samples: df(),
        tiling: vk::ImageTiling::OPTIMAL,
        usage: df(),
        sharingMode: vk::SharingMode::EXCLUSIVE,
        queueFamilyIndexCount: 0,
        pQueueFamilyIndices: null(),
        initialLayout: vk::ImageLayout::UNDEFINED,
    }
}

pub fn samplerCreateInfo() -> vk::SamplerCreateInfo {
    vk::SamplerCreateInfo {
        sType: vk::StructureType::SAMPLER_CREATE_INFO,
        pNext: null(),
        // Fields that were omitted in C++:
        flags: df(),
        magFilter: vk::Filter::NEAREST,
        minFilter: vk::Filter::NEAREST,
        mipmapMode: vk::SamplerMipmapMode::NEAREST,
        addressModeU: vk::SamplerAddressMode::REPEAT,
        addressModeV: vk::SamplerAddressMode::REPEAT,
        addressModeW: vk::SamplerAddressMode::REPEAT,
        mipLodBias: 0.0,
        anisotropyEnable: vk::Bool32::False,
        maxAnisotropy: 0.0,
        compareEnable: vk::Bool32::False,
        compareOp: vk::CompareOp::NEVER,
        minLod: 0.0,
        maxLod: 0.0,
        borderColor: vk::BorderColor::FLOAT_TRANSPARENT_BLACK,
        unnormalizedCoordinates: vk::Bool32::False,
    }
}

pub fn imageViewCreateInfo() -> vk::ImageViewCreateInfo {
    vk::ImageViewCreateInfo {
        sType: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
        pNext: null(),
        // Fields that were omitted in C++:
        flags: df(),
        image: null_mut(),
        viewType: vk::ImageViewType::E_1D,
        format: vk::Format::UNDEFINED,
        components: df(),
        subresourceRange: df(),
    }
}

pub fn framebufferCreateInfo() -> vk::FramebufferCreateInfo {
    vk::FramebufferCreateInfo {
        sType: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
        pNext: null(),
        // Fields that were omitted in C++:
        flags: df(),
        renderPass: null_mut(),
        attachmentCount: 0,
        pAttachments: null(),
        width: 0,
        height: 0,
        layers: 0,
    }
}

pub fn semaphoreCreateInfo(flags: vk::SemaphoreCreateFlags) -> vk::SemaphoreCreateInfo {
    vk::SemaphoreCreateInfo {
        sType: vk::StructureType::SEMAPHORE_CREATE_INFO,
        pNext: null(),
        flags: flags,
    }
}

pub fn submitInfo() -> vk::SubmitInfo {
    vk::SubmitInfo {
        sType: vk::StructureType::SUBMIT_INFO,
        pNext: null(),
        // Fields that were omitted in C++:
        waitSemaphoreCount: 0,
        pWaitSemaphores: null(),
        pWaitDstStageMask: null(),
        commandBufferCount: 0,
        pCommandBuffers: null(),
        signalSemaphoreCount: 0,
        pSignalSemaphores: null(),
    }
}

pub fn viewport(width: f32,
            height: f32,
            minDepth: f32,
            maxDepth: f32) -> vk::Viewport {
    vk::Viewport {
        x: 0.0,
        y: 0.0,
        width: width,
        height: height,
        minDepth: minDepth,
        maxDepth: maxDepth,
    }
}

pub fn rect2D(width: u32,
          height: u32,
          offsetX: i32,
          offsetY: i32) -> vk::Rect2D {
    vk::Rect2D {
        extent: vk::Extent2D { width: width, height: height },
        offset: vk::Offset2D { x: offsetX, y: offsetY },
    }
}

pub fn bufferCreateInfo(usage: vk::BufferUsageFlags, size: vk::DeviceSize) -> vk::BufferCreateInfo {
    vk::BufferCreateInfo {
        sType: vk::StructureType::BUFFER_CREATE_INFO,
        pNext: null(),
        usage: usage,
        size: size,
        flags: df(),
        // Fields that were omitted in C++:
        sharingMode: vk::SharingMode::EXCLUSIVE,
        queueFamilyIndexCount: 0,
        pQueueFamilyIndices: null(),
    }
}

pub fn descriptorPoolCreateInfo(poolSizes: &[vk::DescriptorPoolSize],
                            maxSets: u32) -> vk::DescriptorPoolCreateInfo {
    vk::DescriptorPoolCreateInfo {
        sType: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
        pNext: null(),
        poolSizeCount: poolSizes.len() as u32,
        pPoolSizes: poolSizes.as_ptr(),
        maxSets: maxSets,
        // Fields that were omitted in C++:
        flags: df(),
    }
}

pub fn descriptorPoolSize(typ: vk::DescriptorType, descriptorCount: u32) -> vk::DescriptorPoolSize {
    vk::DescriptorPoolSize {
        typ: typ,
        descriptorCount: descriptorCount,
    }
}

pub fn descriptorSetLayoutBinding(typ: vk::DescriptorType,
                              stageFlags: vk::ShaderStageFlags,
                              binding: u32) -> vk::DescriptorSetLayoutBinding {
    vk::DescriptorSetLayoutBinding {
        descriptorType: typ,
        stageFlags: stageFlags,
        binding: binding,
        // Default value in all examples
        descriptorCount: 1,
        // Fields that were omitted in C++:
        pImmutableSamplers: null(),
    }
}

pub fn descriptorSetLayoutCreateInfo(bindings: &[vk::DescriptorSetLayoutBinding]
                                ) -> vk::DescriptorSetLayoutCreateInfo {
    vk::DescriptorSetLayoutCreateInfo {
        sType: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        pNext: null(),
        pBindings: bindings.as_ptr(),
        bindingCount: bindings.len() as u32,
        // Fields that were omitted in C++:
        flags: df(),
    }
}

pub fn pipelineLayoutCreateInfo(setLayouts: &[vk::DescriptorSetLayout]) -> vk::PipelineLayoutCreateInfo {
    vk::PipelineLayoutCreateInfo {
        sType: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
        pNext: null(),
        setLayoutCount: setLayouts.len() as u32,
        pSetLayouts: setLayouts.as_ptr(),
        // Fields that were omitted in C++:
        flags: df(),
        pushConstantRangeCount: 0,
        pPushConstantRanges: null(),
    }
}

pub fn descriptorSetAllocateInfo(descriptorPool: vk::DescriptorPool,
                             setLayouts: &[vk::DescriptorSetLayout],
                             descriptorSetCount: u32) -> vk::DescriptorSetAllocateInfo {
    vk::DescriptorSetAllocateInfo {
        sType: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
        pNext: null(),
        descriptorPool: descriptorPool,
        pSetLayouts: setLayouts.as_ptr(),
        descriptorSetCount: descriptorSetCount,
    }
}

pub fn descriptorImageInfo(sampler: vk::Sampler, imageView: vk::ImageView, imageLayout: vk::ImageLayout) -> vk::DescriptorImageInfo {
    vk::DescriptorImageInfo {
        sampler: sampler,
        imageView: imageView,
        imageLayout: imageLayout,
    }
}

pub fn writeDescriptorSetBuffer(dstSet: vk::DescriptorSet,
                            typ: vk::DescriptorType,
                            binding: u32,
                            bufferInfo: &vk::DescriptorBufferInfo) -> vk::WriteDescriptorSet {
    vk::WriteDescriptorSet {
        sType: vk::StructureType::WRITE_DESCRIPTOR_SET,
        pNext: null(),
        dstSet: dstSet,
        descriptorType: typ,
        dstBinding: binding,
        pBufferInfo: bufferInfo,
        // Default value in all examples
        descriptorCount: 1,
        // Fields that were omitted in C++:
        dstArrayElement: 0,
        pImageInfo: null(),
        pTexelBufferView: null(),
    }
}

pub fn writeDescriptorSetImage(dstSet: vk::DescriptorSet,
                           typ: vk::DescriptorType,
                           binding: u32,
                           imageInfo: &vk::DescriptorImageInfo) -> vk::WriteDescriptorSet {
    vk::WriteDescriptorSet {
        sType: vk::StructureType::WRITE_DESCRIPTOR_SET,
        pNext: null(),
        dstSet: dstSet,
        descriptorType: typ,
        dstBinding: binding,
        pImageInfo: imageInfo,
        // Default value in all examples
        descriptorCount: 1,
        // Fields that were omitted in C++:
        dstArrayElement: 0,
        pBufferInfo: null(),
        pTexelBufferView: null(),
    }
}

pub fn vertexInputBindingDescription(binding: u32, stride: u32,
                                 inputRate: vk::VertexInputRate) -> vk::VertexInputBindingDescription {
    vk::VertexInputBindingDescription {
        binding: binding,
        stride: stride,
        inputRate: inputRate,
    }
}

pub fn vertexInputAttributeDescription(binding: u32,
                                   location: u32,
                                   format: vk::Format,
                                   offset: u32) -> vk::VertexInputAttributeDescription {
    vk::VertexInputAttributeDescription {
        location: location,
        binding: binding,
        format: format,
        offset: offset,
    }
}

pub fn pipelineVertexInputStateCreateInfo() -> vk::PipelineVertexInputStateCreateInfo {
    vk::PipelineVertexInputStateCreateInfo {
        sType: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        pNext: null(),
        // Fields that were omitted in C++:
        flags: df(),
        vertexBindingDescriptionCount: 0,
        pVertexBindingDescriptions: null(),
        vertexAttributeDescriptionCount: 0,
        pVertexAttributeDescriptions: null(),
    }
}

pub fn pipelineInputAssemblyStateCreateInfo(topology: vk::PrimitiveTopology,
                                        flags: vk::PipelineInputAssemblyStateCreateFlags,
                                        primitiveRestartEnable: vk::Bool32,
                                        ) -> vk::PipelineInputAssemblyStateCreateInfo {
    vk::PipelineInputAssemblyStateCreateInfo {
        sType: vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        topology: topology,
        flags: flags,
        primitiveRestartEnable: primitiveRestartEnable,
        pNext: null(),
    }
}

pub fn pipelineRasterizationStateCreateInfo(polygonMode: vk::PolygonMode,
                                        cullMode: vk::CullModeFlags,
                                        frontFace: vk::FrontFace,
                                        flags: vk::PipelineRasterizationStateCreateFlags,
                                        ) -> vk::PipelineRasterizationStateCreateInfo {
    vk::PipelineRasterizationStateCreateInfo {
        sType: vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        polygonMode: polygonMode,
        cullMode: cullMode,
        frontFace: frontFace,
        flags: flags,
        depthClampEnable: vk::Bool32::True,
        // Fields that were omitted in C++:
        pNext: null(),
        rasterizerDiscardEnable: vk::Bool32::False,
        depthBiasEnable: vk::Bool32::False,
        depthBiasConstantFactor: 0.0,
        depthBiasClamp: 0.0,
        depthBiasSlopeFactor: 0.0,
        lineWidth: 0.0,
    }
}

pub fn pipelineColorBlendAttachmentState(colorWriteMask: vk::ColorComponentFlags,
                                     blendEnable: vk::Bool32,
                                     ) -> vk::PipelineColorBlendAttachmentState {
    vk::PipelineColorBlendAttachmentState {
        colorWriteMask: colorWriteMask,
        blendEnable: blendEnable,
        // Fields that were omitted in C++:
        srcColorBlendFactor: vk::BlendFactor::ZERO,
        dstColorBlendFactor: vk::BlendFactor::ZERO,
        colorBlendOp: vk::BlendOp::ADD,
        srcAlphaBlendFactor: vk::BlendFactor::ZERO,
        dstAlphaBlendFactor: vk::BlendFactor::ZERO,
        alphaBlendOp: vk::BlendOp::ADD,
    }
}

pub fn pipelineColorBlendStateCreateInfo(attachments: &[vk::PipelineColorBlendAttachmentState]) -> vk::PipelineColorBlendStateCreateInfo {
    vk::PipelineColorBlendStateCreateInfo {
        sType: vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        pNext: null(),
        attachmentCount: attachments.len() as u32,
        pAttachments: attachments.as_ptr(),
        // Fields that were omitted in C++:
        flags: df(),
        logicOpEnable: vk::Bool32::False,
        logicOp: vk::LogicOp::CLEAR,
        blendConstants: [0.0; 4],
    }
}

pub fn pipelineDepthStencilStateCreateInfo(depthTestEnable: vk::Bool32, depthWriteEnable: vk::Bool32,
                                       depthCompareOp: vk::CompareOp,
                                       ) -> vk::PipelineDepthStencilStateCreateInfo {
    vk::PipelineDepthStencilStateCreateInfo {
        sType: vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
        depthTestEnable: depthTestEnable,
        depthWriteEnable: depthWriteEnable,
        depthCompareOp: depthCompareOp,
        front: df(),
        back: vk::StencilOpState { compareOp: vk::CompareOp::ALWAYS, ..df() },
        // Fields that were omitted in C++:
        pNext: null(),
        flags: df(),
        depthBoundsTestEnable: vk::Bool32::False,
        stencilTestEnable: vk::Bool32::False,
        minDepthBounds: 0.0,
        maxDepthBounds: 0.0,
    }
}

pub fn pipelineViewportStateCreateInfo(viewportCount: u32, scissorCount: u32,
                                   flags: vk::PipelineViewportStateCreateFlags,
                                   ) -> vk::PipelineViewportStateCreateInfo {
    vk::PipelineViewportStateCreateInfo {
        sType: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        viewportCount: viewportCount,
        scissorCount: scissorCount,
        flags: flags,
        // Fields that were omitted in C++:
        pNext: null(),
        pViewports: null(),
        pScissors: null(),
    }
}

pub fn pipelineMultisampleStateCreateInfo(rasterizationSamples: vk::SampleCountFlag,
                                      flags: vk::PipelineMultisampleStateCreateFlags,
                                      ) -> vk::PipelineMultisampleStateCreateInfo {
    vk::PipelineMultisampleStateCreateInfo {
        sType: vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        rasterizationSamples: rasterizationSamples.into(),
        // Fields that were omitted in C++:
        pNext: null(),
        flags: flags,
        sampleShadingEnable: vk::Bool32::False,
        minSampleShading: 0.0,
        pSampleMask: null(),
        alphaToCoverageEnable: vk::Bool32::False,
        alphaToOneEnable: vk::Bool32::False,
    }
}

pub fn pipelineDynamicStateCreateInfo(dynamicStates: &[vk::DynamicState],
                                  flags: vk::PipelineDynamicStateCreateFlags,
                                  ) -> vk::PipelineDynamicStateCreateInfo {
    vk::PipelineDynamicStateCreateInfo {
        sType: vk::StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
        pDynamicStates: dynamicStates.as_ptr(),
        dynamicStateCount: dynamicStates.len() as u32,
        // Fields that were omitted in C++:
        pNext: null(),
        flags: flags,
    }
}

pub fn pipelineTessellationStateCreateInfo(patchControlPoints: u32) -> vk::PipelineTessellationStateCreateInfo {
    vk::PipelineTessellationStateCreateInfo {
        sType: vk::StructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO,
        patchControlPoints: patchControlPoints,
        // Fields that were omitted in C++:
        pNext: null(),
        flags: df(),
    }
}

pub fn pipelineCreateInfo(layout: vk::PipelineLayout,
                      renderPass: vk::RenderPass,
                      flags: vk::PipelineCreateFlags) -> vk::GraphicsPipelineCreateInfo {
    vk::GraphicsPipelineCreateInfo {
        sType: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
        pNext: null(),
        layout: layout,
        renderPass: renderPass,
        flags: flags,
        // Fields that were omitted in C++:
        stageCount: 0,
        pStages: null(),
        pVertexInputState: null(),
        pInputAssemblyState: null(),
        pTessellationState: null(),
        pViewportState: null(),
        pRasterizationState: null(),
        pMultisampleState: null(),
        pDepthStencilState: null(),
        pColorBlendState: null(),
        pDynamicState: null(),
        subpass: 0,
        basePipelineHandle: null_mut(),
        basePipelineIndex: 0,
    }
}

pub fn computePipelineCreateInfo(layout: vk::PipelineLayout, flags: vk::PipelineCreateFlags) -> vk::ComputePipelineCreateInfo {
    vk::ComputePipelineCreateInfo {
        sType: vk::StructureType::COMPUTE_PIPELINE_CREATE_INFO,
        layout: layout,
        flags: flags,
        // Fields that were omitted in C++:
        pNext: null(),
        stage: vk::PipelineShaderStageCreateInfo {
            sType: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: null(),
            flags: df(),
            stage: df(),
            module: null_mut(),
            pName: null(),
            pSpecializationInfo: null(),
        },
        basePipelineHandle: null_mut(),
        basePipelineIndex: 0,
    }
}

pub fn pushConstantRange(stageFlags: vk::ShaderStageFlags,
                     size: u32, offset: u32) -> vk::PushConstantRange {
    vk::PushConstantRange {
        stageFlags: stageFlags,
        offset: offset,
        size: size,
    }
}
