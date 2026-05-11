use std::ffi::CStr;

use ash::vk;
use vkobjects::{errors::OutOfMemoryError, utility};

use crate::device::{get_extended_features, get_extended_properties, queues::QueueFamilyError};

use super::{
  DeviceExtensions, PhysicalDeviceFeatures, PhysicalDeviceProperties, QueueFamilies, vendor::Vendor,
};

#[derive(Debug, thiserror::Error)]
pub enum PhysicalDeviceSelectionError {
  #[error(transparent)]
  OutOfMemory(#[from] OutOfMemoryError),
  #[error("instance.enumerate_physical_devices() returned VK_ERROR_INITIALIZATION_FAILED")]
  VulkanInitializationFailed,
  #[error(transparent)]
  QueueFamilyError(#[from] QueueFamilyError),
}

impl From<vk::Result> for PhysicalDeviceSelectionError {
  fn from(value: vk::Result) -> Self {
    match value {
      vk::Result::ERROR_OUT_OF_HOST_MEMORY | vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => {
        Self::OutOfMemory(value.into())
      }
      vk::Result::ERROR_INITIALIZATION_FAILED => Self::VulkanInitializationFailed,
      _ => panic!(),
    }
  }
}

pub fn log_device_properties(properties: &vk::PhysicalDeviceProperties) {
  let vendor = Vendor::from_id(properties.vendor_id);
  let driver_version = vendor.parse_driver_version(properties.driver_version);

  log::info!(
    "\nFound physical device {:?}:
        API Version: {},
        Vendor: {},
        Driver Version: {},
        ID: {},
        Type: {},",
    unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }, // expected to be a valid cstr
    utility::parse_vulkan_api_version(properties.api_version),
    vendor,
    driver_version,
    properties.device_id,
    match properties.device_type {
      vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
      vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
      vk::PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
      vk::PhysicalDeviceType::CPU => "CPU",
      _ => "Unknown",
    },
  );
}

pub struct PhysicalDeviceSelectionSuccess<'a> {
  pub physical_device: vk::PhysicalDevice,
  pub properties: PhysicalDeviceProperties<'a>,
  pub supported_extensions: DeviceExtensions,
  pub supported_features: PhysicalDeviceFeatures<'a>,
  pub queue_families: QueueFamilies,
}

pub struct PhysicalDeviceSelection<'a> {
  pub physical_device: vk::PhysicalDevice,
  pub properties: PhysicalDeviceProperties<'a>,
  pub supported_extensions: DeviceExtensions,
  pub supported_features: PhysicalDeviceFeatures<'a>,
}

pub fn enumerate_physical_devices_for_selection<'a>(
  instance: &'a ash::Instance,
) -> Result<Vec<PhysicalDeviceSelection<'a>>, PhysicalDeviceSelectionError> {
  let devices = unsafe { instance.enumerate_physical_devices() }?;

  let mut selection = Vec::with_capacity(devices.len());
  for physical_device in devices {
    let properties = get_extended_properties(instance, physical_device);
    log_device_properties(&properties.p10);

    let supported_extensions =
      DeviceExtensions::mark_supported_by_physical_device(instance, physical_device)?;
    let supported_features =
      get_extended_features(instance, physical_device, &supported_extensions);

    selection.push(PhysicalDeviceSelection {
      physical_device,
      properties,
      supported_extensions,
      supported_features,
    });
  }

  Ok(selection)
}
