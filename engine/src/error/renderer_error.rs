#[derive(Debug)]
pub enum RendererError {
    Vulkan(VulkanError),
}

#[derive(Debug)]
pub enum VulkanError {
    NotInitialized,
    InstanceCreationFailed,
}
