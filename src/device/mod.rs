pub mod device_selector;
mod extensions;
mod features;
mod logical_device;
mod physical_device;
mod queues;
mod vendor;

pub use extensions::DeviceExtensions;
pub use features::{DeviceFeatures, PhysicalDeviceFeatures, get_extended_features};
pub use logical_device::{Device, DeviceCreationError};
pub use physical_device::PhysicalDevice;
pub use queues::{QueueFamilies, SingleQueues};

use std::{
  ffi::{CStr, c_void},
  mem::MaybeUninit,
  ptr::{self, addr_of_mut},
};

use ash::vk;

pub const GRAPHICS_QUEUE_LABEL: &CStr = c"GRAPHICS QUEUE";
pub const COMPUTE_QUEUE_LABEL: &CStr = c"COMPUTE QUEUE";
pub const TRANSFER_QUEUE_LABEL: &CStr = c"TRANSFER QUEUE";
pub const GRAPHICS_QUEUE_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const COMPUTE_QUEUE_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const TRANSFER_QUEUE_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

#[allow(unused)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PhysicalDeviceProperties<'a> {
  pub p10: vk::PhysicalDeviceProperties,
  pub p11: vk::PhysicalDeviceVulkan11Properties<'a>,
  pub p12: vk::PhysicalDeviceVulkan12Properties<'a>,
  pub p13: vk::PhysicalDeviceVulkan13Properties<'a>,
}

pub fn get_extended_properties(
  instance: &ash::Instance,
  physical_device: vk::PhysicalDevice,
) -> PhysicalDeviceProperties<'_> {
  // see https://doc.rust-lang.org/std/mem/union.MaybeUninit.html
  let mut props10: MaybeUninit<vk::PhysicalDeviceProperties2> = MaybeUninit::uninit();
  let mut props11: MaybeUninit<vk::PhysicalDeviceVulkan11Properties> = MaybeUninit::uninit();
  let mut props12: MaybeUninit<vk::PhysicalDeviceVulkan12Properties> = MaybeUninit::uninit();
  let mut props13: MaybeUninit<vk::PhysicalDeviceVulkan13Properties> = MaybeUninit::uninit();

  let props10_ptr = props10.as_mut_ptr();
  let props11_ptr = props11.as_mut_ptr();
  let props12_ptr = props12.as_mut_ptr();
  let props13_ptr = props13.as_mut_ptr();

  unsafe {
    addr_of_mut!((*props10_ptr).s_type).write(vk::StructureType::PHYSICAL_DEVICE_PROPERTIES_2);
    addr_of_mut!((*props11_ptr).s_type)
      .write(vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES);
    addr_of_mut!((*props12_ptr).s_type)
      .write(vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES);
    addr_of_mut!((*props13_ptr).s_type)
      .write(vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES);

    addr_of_mut!((*props10_ptr).p_next).write(props11_ptr as *mut c_void);
    addr_of_mut!((*props11_ptr).p_next).write(props12_ptr as *mut c_void);
    addr_of_mut!((*props12_ptr).p_next).write(props13_ptr as *mut c_void);
    addr_of_mut!((*props13_ptr).p_next).write(ptr::null_mut::<c_void>());

    instance.get_physical_device_properties2(physical_device, props10_ptr.as_mut().unwrap());
    PhysicalDeviceProperties {
      p10: props10.assume_init().properties,
      p11: props11.assume_init(),
      p12: props12.assume_init(),
      p13: props13.assume_init(),
    }
  }
}
