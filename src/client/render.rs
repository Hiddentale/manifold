use vulkan_rust::{vk, Version};

pub mod graphical_core;

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);

const VALIDATION_LAYER: &std::ffi::CStr = c"VK_LAYER_KHRONOS_validation";
const DEVICE_EXTENSIONS: &[&std::ffi::CStr] = &[vk::extension_names::KHR_SWAPCHAIN_EXTENSION_NAME];
