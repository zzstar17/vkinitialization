use ash::vk;
use vkobjects::{errors::OutOfMemoryError, utility};

use crate::SWAPCHAIN_MAINTENANCE_EXT_NAME;

#[derive(Debug, Default, Clone, Copy)]
pub struct DeviceExtensions {
  pub memory_priority: bool,
  pub pageable_device_local_memory: bool,
  pub swapchain: bool,
  pub swapchain_maintenance1: bool,
  pub dynamic_rendering_local_read: bool,
}

impl DeviceExtensions {
  pub fn mark_supported_by_physical_device(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
  ) -> Result<Self, OutOfMemoryError> {
    let properties = unsafe { instance.enumerate_device_extension_properties(physical_device)? };

    let mut supported = Self::default();

    // a bit inefficient as it retests for valid cstrings but at least doesn't do any allocations
    let is_supported = |ext| {
      properties
        .iter()
        .any(|props| unsafe { utility::i8_array_as_cstr(&props.extension_name) }.unwrap() == ext)
    };

    if is_supported(ash::ext::memory_priority::NAME) {
      supported.memory_priority = true;
    }
    if is_supported(ash::ext::pageable_device_local_memory::NAME) {
      supported.pageable_device_local_memory = true;
    }
    if is_supported(ash::khr::swapchain::NAME) {
      supported.swapchain = true;
    }
    if is_supported(SWAPCHAIN_MAINTENANCE_EXT_NAME) {
      supported.swapchain_maintenance1 = true;
    }
    if is_supported(ash::khr::dynamic_rendering_local_read::NAME) {
      supported.swapchain = true;
    }

    Ok(supported)
  }

  pub fn get_extension_list(&self) -> Vec<*const i8> {
    let mut ptrs = Vec::new();
    if self.memory_priority {
      ptrs.push(ash::ext::memory_priority::NAME.as_ptr());
    }
    if self.pageable_device_local_memory {
      ptrs.push(ash::ext::pageable_device_local_memory::NAME.as_ptr());
    }
    if self.swapchain {
      ptrs.push(ash::khr::swapchain::NAME.as_ptr());
    }
    if self.swapchain_maintenance1 {
      ptrs.push(SWAPCHAIN_MAINTENANCE_EXT_NAME.as_ptr());
    }
    if self.dynamic_rendering_local_read {
      ptrs.push(ash::khr::dynamic_rendering_local_read::NAME.as_ptr());
    }
    ptrs
  }

  /// Returns other's extensions that are missing in self (None if nothing)
  pub fn filter_missing(&self, other: Self) -> Option<Self> {
    let mut none_missing = true;
    let mut missing = Self::default();

    if other.memory_priority && !self.memory_priority {
      missing.memory_priority = true;
      none_missing = false;
    }
    if other.pageable_device_local_memory && !self.pageable_device_local_memory {
      missing.pageable_device_local_memory = true;
      none_missing = false;
    }
    if other.swapchain && !self.swapchain {
      missing.swapchain = true;
      none_missing = false;
    }
    if other.swapchain_maintenance1 && !self.swapchain_maintenance1 {
      missing.swapchain_maintenance1 = true;
      none_missing = false;
    }
    if other.dynamic_rendering_local_read && !self.dynamic_rendering_local_read {
      missing.dynamic_rendering_local_read = true;
      none_missing = false;
    }

    if none_missing {
      return None;
    }

    Some(missing)
  }

  pub fn and(&self, other: Self) -> Self {
    let mut result = Self::default();

    if other.memory_priority && self.memory_priority {
      result.memory_priority = true;
    }
    if other.pageable_device_local_memory && self.pageable_device_local_memory {
      result.pageable_device_local_memory = true;
    }
    if other.swapchain && self.swapchain {
      result.swapchain = true;
    }
    if other.swapchain_maintenance1 && self.swapchain_maintenance1 {
      result.swapchain_maintenance1 = true;
    }
    if other.dynamic_rendering_local_read && self.dynamic_rendering_local_read {
      result.dynamic_rendering_local_read = true;
    }

    result
  }

  pub fn or(&self, other: Self) -> Self {
    let mut result = Self::default();

    if other.memory_priority || self.memory_priority {
      result.memory_priority = true;
    }
    if other.pageable_device_local_memory || self.pageable_device_local_memory {
      result.pageable_device_local_memory = true;
    }
    if other.swapchain || self.swapchain {
      result.swapchain = true;
    }
    if other.swapchain_maintenance1 || self.swapchain_maintenance1 {
      result.swapchain_maintenance1 = true;
    }
    if other.dynamic_rendering_local_read || self.dynamic_rendering_local_read {
      result.dynamic_rendering_local_read = true;
    }

    result
  }
}
