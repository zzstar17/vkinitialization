pub mod device_selector;
mod extensions;
mod features;
mod logical_device;
mod physical_device;
mod physical_device_properties;
mod queues;
mod vendor;

use std::ffi::CStr;

pub use extensions::DeviceExtensions;
pub use features::{DeviceFeatures, PhysicalDeviceFeatures, get_extended_features};
pub use logical_device::{Device, DeviceCreationError};
pub use physical_device::PhysicalDevice;
pub use physical_device_properties::{
  LifetimePhysicalDeviceProperties, PhysicalDeviceProperties, PhysicalDeviceProperties11,
  PhysicalDeviceProperties12, PhysicalDeviceProperties13, get_extended_properties,
};
pub use queues::{QueueFamilies, SingleQueues};

pub const GRAPHICS_QUEUE_LABEL: &CStr = c"GRAPHICS QUEUE";
pub const COMPUTE_QUEUE_LABEL: &CStr = c"COMPUTE QUEUE";
pub const TRANSFER_QUEUE_LABEL: &CStr = c"TRANSFER QUEUE";
pub const GRAPHICS_QUEUE_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const COMPUTE_QUEUE_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const TRANSFER_QUEUE_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
