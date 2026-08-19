use std::{
  ffi::c_void,
  mem::MaybeUninit,
  ptr::{self, addr_of_mut},
};

use ash::vk;

use crate::device::DeviceExtensions;

#[derive(Debug, Default, Clone, Copy)]
pub struct DeviceFeatures {
  pub synchronization2: bool,
  pub dynamic_rendering: bool,
  pub swapchain_maintenance1: bool,
}

pub struct SubmissionDeviceFeatures<'a> {
  pub features10: vk::PhysicalDeviceFeatures,
  pub features12: vk::PhysicalDeviceVulkan12Features<'a>,
  pub features13: vk::PhysicalDeviceVulkan13Features<'a>,
  pub swapchain_maintenance1: Option<vk::PhysicalDeviceSwapchainMaintenance1FeaturesEXT<'a>>,
}

impl<'a> SubmissionDeviceFeatures<'a> {
  /// Only valid as long as self isn't moved
  pub fn get_features_2(&'a mut self) -> vk::PhysicalDeviceFeatures2<'a> {
    let mut features2 = vk::PhysicalDeviceFeatures2::default()
      .features(vk::PhysicalDeviceFeatures::default())
      .push_next(&mut self.features13)
      .push_next(&mut self.features12);

    if let Some(feature_obj) = self.swapchain_maintenance1.as_mut() {
      features2 = features2.push_next(feature_obj);
    }

    features2
  }
}

impl DeviceFeatures {
  pub fn from_full_physical_device_features(features: &PhysicalDeviceFeatures) -> Self {
    Self {
      synchronization2: features.f13.synchronization2 == vk::TRUE,
      dynamic_rendering: features.f13.dynamic_rendering == vk::TRUE,
      swapchain_maintenance1: features.swapchain_maintenance1,
    }
  }

  pub fn to_full_physical_device_features<'a>(&'a self) -> SubmissionDeviceFeatures<'a> {
    let swapchain_maintenance1 = if self.swapchain_maintenance1 {
      let mut swapchain_maintenance1 =
        vk::PhysicalDeviceSwapchainMaintenance1FeaturesEXT::default();
      // vk::PhysicalDeviceSwapchainMaintenance1FeaturesKHR does not exist, but this should be equivalent
      assert_eq!(
        swapchain_maintenance1.s_type,
        vk::StructureType::from_raw(1000275000)
      );
      swapchain_maintenance1.swapchain_maintenance1 = vk::TRUE;
      Some(swapchain_maintenance1)
    } else {
      None
    };

    SubmissionDeviceFeatures {
      features10: vk::PhysicalDeviceFeatures::default(),
      features12: vk::PhysicalDeviceVulkan12Features::default(),
      features13: vk::PhysicalDeviceVulkan13Features {
        synchronization2: self.synchronization2 as u32,
        dynamic_rendering: self.dynamic_rendering as u32,
        ..Default::default()
      },
      swapchain_maintenance1,
    }
  }

  /// Returns other's features that are missing in self (None if nothing)
  pub fn filter_missing(&self, other: Self) -> Option<Self> {
    let mut none_missing = true;
    let mut missing = Self::default();

    if other.swapchain_maintenance1 && !self.swapchain_maintenance1 {
      missing.swapchain_maintenance1 = true;
      none_missing = false;
    }
    if other.synchronization2 && !self.synchronization2 {
      missing.synchronization2 = true;
      none_missing = false;
    }
    if other.dynamic_rendering && !self.dynamic_rendering {
      missing.dynamic_rendering = true;
      none_missing = false;
    }

    if none_missing {
      return None;
    }

    Some(missing)
  }

  pub fn and(&self, other: Self) -> Self {
    let mut result = Self::default();

    if other.swapchain_maintenance1 && self.swapchain_maintenance1 {
      result.swapchain_maintenance1 = true;
    }
    if other.synchronization2 && self.synchronization2 {
      result.synchronization2 = true;
    }
    if other.dynamic_rendering && self.dynamic_rendering {
      result.dynamic_rendering = true;
    }

    result
  }

  pub fn or(&self, other: Self) -> Self {
    let mut result = Self::default();

    if other.swapchain_maintenance1 || self.swapchain_maintenance1 {
      result.swapchain_maintenance1 = true;
    }
    if other.synchronization2 || self.synchronization2 {
      result.synchronization2 = true;
    }
    if other.dynamic_rendering || self.dynamic_rendering {
      result.dynamic_rendering = true;
    }

    result
  }

  pub fn filter_extension_required(&mut self, extensions: &DeviceExtensions) {
    if self.swapchain_maintenance1 {
      self.swapchain_maintenance1 = extensions.swapchain_maintenance1
    }
  }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PhysicalDeviceFeatures<'a> {
  pub f10: vk::PhysicalDeviceFeatures,
  pub f11: vk::PhysicalDeviceVulkan11Features<'a>,
  pub f12: vk::PhysicalDeviceVulkan12Features<'a>,
  pub f13: vk::PhysicalDeviceVulkan13Features<'a>,
  pub swapchain_maintenance1: bool,
}

// https://www.reddit.com/r/vulkan/comments/1j16wut/if_an_extension_defines_a_feature_struct_and_this/
pub fn get_extended_features<'a>(
  instance: &ash::Instance,
  physical_device: vk::PhysicalDevice,
  supported_extensions: &DeviceExtensions,
) -> PhysicalDeviceFeatures<'a> {
  let mut features10: MaybeUninit<vk::PhysicalDeviceFeatures2> = MaybeUninit::uninit();
  let mut features11: MaybeUninit<vk::PhysicalDeviceVulkan11Features> = MaybeUninit::uninit();
  let mut features12: MaybeUninit<vk::PhysicalDeviceVulkan12Features> = MaybeUninit::uninit();
  let mut features13: MaybeUninit<vk::PhysicalDeviceVulkan13Features> = MaybeUninit::uninit();

  // vk::PhysicalDeviceSwapchainMaintenance1FeaturesKHR does not exist, but this should be equivalent
  let mut swapchain_maintenance1: MaybeUninit<vk::PhysicalDeviceSwapchainMaintenance1FeaturesEXT> =
    MaybeUninit::uninit();

  let features10_ptr = features10.as_mut_ptr();
  let features11_ptr = features11.as_mut_ptr();
  let features12_ptr = features12.as_mut_ptr();
  let features13_ptr = features13.as_mut_ptr();

  let swapchain_maintenance1_ptr = swapchain_maintenance1.as_mut_ptr();

  unsafe {
    addr_of_mut!((*features10_ptr).s_type).write(vk::StructureType::PHYSICAL_DEVICE_FEATURES_2);
    addr_of_mut!((*features11_ptr).s_type)
      .write(vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES);
    addr_of_mut!((*features12_ptr).s_type)
      .write(vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES);
    addr_of_mut!((*features13_ptr).s_type)
      .write(vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES);

    addr_of_mut!((*features10_ptr).p_next).write(features11_ptr as *mut c_void);
    addr_of_mut!((*features11_ptr).p_next).write(features12_ptr as *mut c_void);
    addr_of_mut!((*features12_ptr).p_next).write(features13_ptr as *mut c_void);
    addr_of_mut!((*features13_ptr).p_next).write(ptr::null_mut::<c_void>());

    if supported_extensions.swapchain_maintenance1 {
      addr_of_mut!((*swapchain_maintenance1_ptr).s_type)
        .write(vk::StructureType::from_raw(1000275000));

      addr_of_mut!((*features13_ptr).p_next).write(swapchain_maintenance1_ptr as *mut c_void);
      addr_of_mut!((*swapchain_maintenance1_ptr).p_next).write(ptr::null_mut::<c_void>());
    }

    instance.get_physical_device_features2(physical_device, features10_ptr.as_mut().unwrap());

    let swapchain_maintenance1_feature_supported = if supported_extensions.swapchain_maintenance1 {
      swapchain_maintenance1.assume_init().swapchain_maintenance1 == vk::TRUE
    } else {
      false
    };

    PhysicalDeviceFeatures {
      f10: features10.assume_init().features,
      f11: features11.assume_init(),
      f12: features12.assume_init(),
      f13: features13.assume_init(),
      swapchain_maintenance1: swapchain_maintenance1_feature_supported,
    }
  }
}
