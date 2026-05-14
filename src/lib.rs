pub mod device;
mod entry;
mod instance;
#[cfg(feature = "surface")]
mod surface;

#[cfg(feature = "vl")]
mod validation_layers;

use std::ffi::CStr;

pub use entry::get_entry;
pub use instance::{InstanceCreationError, InstanceOptionalExtensions, create_instance};

#[cfg(feature = "surface")]
pub use surface::{Surface, SurfaceError};
#[cfg(feature = "vl")]
pub use validation_layers::{DebugUtils, DebugUtilsMarker};

use std::fmt::{self, Write};

use ash::vk;

#[cfg(all(feature = "surface", not(feature = "graphics_family")))]
compile_error!(
  "\
    Feature \"surface\" requires feature \"graphics_queue\""
);

#[cfg(feature = "vl")]
const VALIDATION_LAYERS: [&CStr; 1] = [c"VK_LAYER_KHRONOS_validation"];
#[cfg(feature = "vl")]
const ADDITIONAL_VALIDATION_FEATURES: [vk::ValidationFeatureEnableEXT; 2] = [
  vk::ValidationFeatureEnableEXT::BEST_PRACTICES,
  vk::ValidationFeatureEnableEXT::SYNCHRONIZATION_VALIDATION,
];

static SURFACE_MAINTENANCE_EXT_NAME: &CStr = c"VK_KHR_surface_maintenance1";
static SWAPCHAIN_MAINTENANCE_EXT_NAME: &CStr = c"VK_KHR_swapchain_maintenance1";

pub fn debug_print_device_memory_info(
  mem_properties: &vk::PhysicalDeviceMemoryProperties,
) -> fmt::Result {
  let mut output = String::new();

  output.write_fmt(format_args!(
    "\nAvailable memory heaps: ({} heaps, {} memory types)\n",
    mem_properties.memory_heap_count, mem_properties.memory_type_count
  ))?;
  for heap_i in 0..mem_properties.memory_heap_count {
    let heap = mem_properties.memory_heaps[heap_i as usize];
    let heap_flags = if heap.flags.is_empty() {
      String::from("no heap flags")
    } else {
      format!("heap flags [{:?}]", heap.flags)
    };

    output.write_fmt(format_args!(
      "    {} -> {}MiB with {} and attributed memory types:\n",
      heap_i,
      heap.size / 1000000,
      heap_flags
    ))?;
    for type_i in 0..mem_properties.memory_type_count {
      let mem_type = mem_properties.memory_types[type_i as usize];
      if mem_type.heap_index != heap_i {
        continue;
      }

      let flags = mem_type.property_flags;
      output.write_fmt(format_args!(
        "        {} -> {}\n",
        type_i,
        if flags.is_empty() {
          "<no flags>".to_owned()
        } else {
          format!("[{:?}]", flags)
        }
      ))?;
    }
  }
  log::debug!("{}", output);

  Ok(())
}

#[cfg(test)]
mod tests {
  use std::{marker::PhantomData, ptr};

  use vkobjects::ManuallyDestroyed;

  #[cfg(feature = "surface")]
  use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
  #[cfg(feature = "surface")]
  use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
  };

  use crate::{
    device::{
      Device, DeviceExtensions, DeviceFeatures, PhysicalDevice, QueueFamilies,
      device_selector::{
        PhysicalDeviceSelectionError, PhysicalDeviceSelectionSuccess,
        enumerate_physical_devices_for_selection,
      },
    },
    instance::InstanceOptionalExtensions,
  };

  #[cfg(feature = "surface")]
  use winit::platform::wayland::EventLoopBuilderExtWayland;

  use super::*;

  const TEST_API_VERSION: u32 = vk::API_VERSION_1_3;
  const TEST_APP_NAME: &CStr = c"Testing";
  const TEST_APP_VERSION: u32 = vk::make_api_version(0, 1, 0, 0);

  fn get_testing_app_info<'a>() -> vk::ApplicationInfo<'a> {
    vk::ApplicationInfo {
      s_type: vk::StructureType::APPLICATION_INFO,
      api_version: TEST_API_VERSION,
      p_application_name: TEST_APP_NAME.as_ptr(),
      application_version: TEST_APP_VERSION,
      p_engine_name: ptr::null(),
      engine_version: vk::make_api_version(0, 1, 0, 0),
      p_next: ptr::null(),
      _marker: PhantomData,
    }
  }

  fn select_physical_device<'a>(
    instance: &'a ash::Instance,
    #[cfg(feature = "surface")] surface: &Surface,
  ) -> Result<Option<PhysicalDeviceSelectionSuccess<'a>>, PhysicalDeviceSelectionError> {
    let selected_devices = enumerate_physical_devices_for_selection(instance)?;
    let selected_device = selected_devices.into_iter().next().unwrap();
    let queue_families = QueueFamilies::get_from_physical_device(
      instance,
      selected_device.physical_device,
      #[cfg(feature = "surface")]
      surface,
    )?;

    Ok(Some(PhysicalDeviceSelectionSuccess {
      physical_device: selected_device.physical_device,
      properties: selected_device.properties,
      supported_extensions: selected_device.supported_extensions,
      supported_features: selected_device.supported_features,
      queue_families,
    }))
  }

  #[cfg(feature = "surface")]
  struct App {
    window: Option<Window>,
    entry: ash::Entry,
    instance: ash::Instance,
    #[cfg(feature = "vl")]
    pub debug_utils: super::DebugUtils,
  }

  #[cfg(feature = "surface")]
  impl App {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
      let entry: ash::Entry = unsafe { super::get_entry() };

      let display_handle = event_loop
        .display_handle()
        .expect("Failed to retrieve display handle");

      let app_info = get_testing_app_info();
      let wanted_optional_extensions = InstanceOptionalExtensions::full();
      #[cfg(feature = "vl")]
      let (instance, activated_optional_extensions, debug_utils) =
        super::create_instance(&entry, app_info, wanted_optional_extensions, display_handle)
          .expect("Failed to create instance");
      #[cfg(not(feature = "vl"))]
      let (instance, activated_optional_extensions) =
        super::create_instance(&entry, app_info, wanted_optional_extensions, display_handle)
          .expect("Failed to create instance");

      log::info!(
        "Activated instance extensions:\n{:#?}",
        activated_optional_extensions
      );

      Self {
        window: None,
        entry,
        instance,
        #[cfg(feature = "vl")]
        debug_utils,
      }
    }
  }

  #[cfg(feature = "surface")]
  impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
      let window_attributes = Window::default_attributes().with_title("Testing");
      let window = event_loop
        .create_window(window_attributes)
        .expect("Failed to create window");
      self.window = Some(window);

      let surface = Surface::new(
        &self.entry,
        &self.instance,
        event_loop
          .display_handle()
          .expect("Failed to get active event loop display handle"),
        self
          .window
          .as_ref()
          .unwrap()
          .window_handle()
          .expect("Failed to get window handle"),
      )
      .expect("Failed to create surface");

      let physical_device_creation =
        unsafe { PhysicalDevice::select(&self.instance, &surface, select_physical_device) }
          .expect("Failed to get physical device")
          .unwrap();

      #[allow(unused)]
      let (device, queues) = Device::create(
        &self.instance,
        &physical_device_creation,
        DeviceExtensions {
          swapchain: true,
          ..Default::default()
        },
        DeviceExtensions {
          memory_priority: true,
          pageable_device_local_memory: true,
          swapchain_maintenance1: true,
          ..Default::default()
        },
        DeviceFeatures::default(),
        DeviceFeatures {
          swapchain_maintenance1: true,
          synchronization2: true,
          ..Default::default()
        },
      )
      .expect("Failed to create device");

      #[cfg(feature = "vl")]
      let debug_utils_marker = DebugUtilsMarker::new(&self.instance, &device);
      #[cfg(feature = "vl")]
      unsafe {
        debug_utils_marker.set_queue_labels(queues);
      }

      unsafe {
        ManuallyDestroyed::destroy_self(&surface);
        ManuallyDestroyed::destroy_self(&device);

        #[cfg(feature = "vl")]
        {
          ManuallyDestroyed::destroy_self(&self.debug_utils);
        }
        ManuallyDestroyed::destroy_self(&self.instance);
      }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
      match event {
        WindowEvent::CloseRequested => {
          event_loop.exit();
        }
        WindowEvent::RedrawRequested => {
          event_loop.exit();
        }
        _ => (),
      }
    }
  }

  #[test]
  #[cfg(feature = "surface")]
  fn initializes_fine() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let event_loop = EventLoop::builder()
      .with_any_thread(true)
      .build()
      .expect("Failed to initialize event loop");
    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app).unwrap();
  }

  #[test]
  #[cfg(not(feature = "surface"))]
  fn initializes_fine() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let entry: ash::Entry = unsafe { super::get_entry() };

    let app_info = get_testing_app_info();
    let wanted_optional_extensions = InstanceOptionalExtensions::default();
    #[cfg(feature = "vl")]
    let (instance, activated_optional_extensions, debug_utils) =
      super::create_instance(&entry, app_info, wanted_optional_extensions)
        .expect("Failed to create instance");
    #[cfg(not(feature = "vl"))]
    let (instance, activated_optional_extensions) =
      super::create_instance(&entry, app_info, wanted_optional_extensions)
        .expect("Failed to create instance");

    log::info!(
      "Activated instance extensions:\n{:#?}",
      activated_optional_extensions
    );

    let physical_device_creation =
      unsafe { PhysicalDevice::select(&instance, select_physical_device) }
        .expect("Failed to get physical device")
        .unwrap();

    #[allow(unused)]
    let (device, queues) = Device::create(
      &instance,
      &physical_device_creation,
      DeviceExtensions::default(),
      DeviceExtensions {
        memory_priority: true,
        pageable_device_local_memory: true,
        ..Default::default()
      },
      DeviceFeatures::default(),
      DeviceFeatures {
        synchronization2: true,
        ..Default::default()
      },
    )
    .expect("Failed to create device");

    #[cfg(feature = "vl")]
    let debug_utils_marker = DebugUtilsMarker::new(&instance, &device);
    #[cfg(feature = "vl")]
    unsafe {
      debug_utils_marker.set_queue_labels(queues);
    }

    unsafe {
      ManuallyDestroyed::destroy_self(&device);

      #[cfg(feature = "vl")]
      {
        ManuallyDestroyed::destroy_self(&debug_utils);
      }
      ManuallyDestroyed::destroy_self(&instance);
    }
  }
}
