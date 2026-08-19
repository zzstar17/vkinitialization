use std::{
  ffi::{c_char, c_void},
  mem::MaybeUninit,
  ptr::{self, addr_of_mut},
};

use ash::vk;

// device properties without the lifetimes
#[derive(Debug, Clone, Copy)]
pub struct PhysicalDeviceProperties {
  pub p10: vk::PhysicalDeviceProperties,
  pub p11: PhysicalDeviceProperties11,
  pub p12: PhysicalDeviceProperties12,
  pub p13: PhysicalDeviceProperties13,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, Default)]
pub struct LifetimePhysicalDeviceProperties<'a> {
  pub p10: vk::PhysicalDeviceProperties,
  pub p11: vk::PhysicalDeviceVulkan11Properties<'a>,
  pub p12: vk::PhysicalDeviceVulkan12Properties<'a>,
  pub p13: vk::PhysicalDeviceVulkan13Properties<'a>,
}

impl<'a> From<LifetimePhysicalDeviceProperties<'a>> for PhysicalDeviceProperties {
  fn from(value: LifetimePhysicalDeviceProperties<'a>) -> Self {
    Self {
      p10: value.p10,
      p11: value.p11.into(),
      p12: value.p12.into(),
      p13: value.p13.into(),
    }
  }
}

pub fn get_extended_properties(
  instance: &ash::Instance,
  physical_device: vk::PhysicalDevice,
) -> LifetimePhysicalDeviceProperties<'_> {
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
    LifetimePhysicalDeviceProperties {
      p10: props10.assume_init().properties,
      p11: props11.assume_init(),
      p12: props12.assume_init(),
      p13: props13.assume_init(),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct PhysicalDeviceProperties11 {
  pub device_uuid: [u8; vk::UUID_SIZE],
  pub driver_uuid: [u8; vk::UUID_SIZE],
  pub device_luid: [u8; vk::LUID_SIZE],
  pub device_node_mask: u32,
  pub device_luid_valid: vk::Bool32,
  pub subgroup_size: u32,
  pub subgroup_supported_stages: vk::ShaderStageFlags,
  pub subgroup_supported_operations: vk::SubgroupFeatureFlags,
  pub subgroup_quad_operations_in_all_stages: vk::Bool32,
  pub point_clipping_behavior: vk::PointClippingBehavior,
  pub max_multiview_view_count: u32,
  pub max_multiview_instance_index: u32,
  pub protected_no_fault: vk::Bool32,
  pub max_per_set_descriptors: u32,
  pub max_memory_allocation_size: vk::DeviceSize,
}

impl<'a> From<vk::PhysicalDeviceVulkan11Properties<'a>> for PhysicalDeviceProperties11 {
  fn from(value: vk::PhysicalDeviceVulkan11Properties<'a>) -> Self {
    Self {
      device_uuid: value.device_uuid,
      driver_uuid: value.driver_uuid,
      device_luid: value.device_luid,
      device_node_mask: value.device_node_mask,
      device_luid_valid: value.device_luid_valid,
      subgroup_size: value.subgroup_size,
      subgroup_supported_stages: value.subgroup_supported_stages,
      subgroup_supported_operations: value.subgroup_supported_operations,
      subgroup_quad_operations_in_all_stages: value.subgroup_quad_operations_in_all_stages,
      point_clipping_behavior: value.point_clipping_behavior,
      max_multiview_view_count: value.max_multiview_view_count,
      max_multiview_instance_index: value.max_multiview_instance_index,
      protected_no_fault: value.protected_no_fault,
      max_per_set_descriptors: value.max_per_set_descriptors,
      max_memory_allocation_size: value.max_memory_allocation_size,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct PhysicalDeviceProperties12 {
  pub driver_id: vk::DriverId,
  pub driver_name: [c_char; vk::MAX_DRIVER_NAME_SIZE],
  pub driver_info: [c_char; vk::MAX_DRIVER_INFO_SIZE],
  pub conformance_version: vk::ConformanceVersion,
  pub denorm_behavior_independence: vk::ShaderFloatControlsIndependence,
  pub rounding_mode_independence: vk::ShaderFloatControlsIndependence,
  pub shader_signed_zero_inf_nan_preserve_float16: vk::Bool32,
  pub shader_signed_zero_inf_nan_preserve_float32: vk::Bool32,
  pub shader_signed_zero_inf_nan_preserve_float64: vk::Bool32,
  pub shader_denorm_preserve_float16: vk::Bool32,
  pub shader_denorm_preserve_float32: vk::Bool32,
  pub shader_denorm_preserve_float64: vk::Bool32,
  pub shader_denorm_flush_to_zero_float16: vk::Bool32,
  pub shader_denorm_flush_to_zero_float32: vk::Bool32,
  pub shader_denorm_flush_to_zero_float64: vk::Bool32,
  pub shader_rounding_mode_rte_float16: vk::Bool32,
  pub shader_rounding_mode_rte_float32: vk::Bool32,
  pub shader_rounding_mode_rte_float64: vk::Bool32,
  pub shader_rounding_mode_rtz_float16: vk::Bool32,
  pub shader_rounding_mode_rtz_float32: vk::Bool32,
  pub shader_rounding_mode_rtz_float64: vk::Bool32,
  pub max_update_after_bind_descriptors_in_all_pools: u32,
  pub shader_uniform_buffer_array_non_uniform_indexing_native: vk::Bool32,
  pub shader_sampled_image_array_non_uniform_indexing_native: vk::Bool32,
  pub shader_storage_buffer_array_non_uniform_indexing_native: vk::Bool32,
  pub shader_storage_image_array_non_uniform_indexing_native: vk::Bool32,
  pub shader_input_attachment_array_non_uniform_indexing_native: vk::Bool32,
  pub robust_buffer_access_update_after_bind: vk::Bool32,
  pub quad_divergent_implicit_lod: vk::Bool32,
  pub max_per_stage_descriptor_update_after_bind_samplers: u32,
  pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
  pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
  pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
  pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
  pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
  pub max_per_stage_update_after_bind_resources: u32,
  pub max_descriptor_set_update_after_bind_samplers: u32,
  pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
  pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
  pub max_descriptor_set_update_after_bind_storage_buffers: u32,
  pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
  pub max_descriptor_set_update_after_bind_sampled_images: u32,
  pub max_descriptor_set_update_after_bind_storage_images: u32,
  pub max_descriptor_set_update_after_bind_input_attachments: u32,
  pub supported_depth_resolve_modes: vk::ResolveModeFlags,
  pub supported_stencil_resolve_modes: vk::ResolveModeFlags,
  pub independent_resolve_none: vk::Bool32,
  pub independent_resolve: vk::Bool32,
  pub filter_minmax_single_component_formats: vk::Bool32,
  pub filter_minmax_image_component_mapping: vk::Bool32,
  pub max_timeline_semaphore_value_difference: u64,
  pub framebuffer_integer_color_sample_counts: vk::SampleCountFlags,
}

impl<'a> From<vk::PhysicalDeviceVulkan12Properties<'a>> for PhysicalDeviceProperties12 {
  fn from(value: vk::PhysicalDeviceVulkan12Properties<'a>) -> Self {
    Self {
      driver_id: value.driver_id,
      driver_name: value.driver_name,
      driver_info: value.driver_info,
      conformance_version: value.conformance_version,
      denorm_behavior_independence: value.denorm_behavior_independence,
      rounding_mode_independence: value.rounding_mode_independence,
      shader_signed_zero_inf_nan_preserve_float16: value
        .shader_signed_zero_inf_nan_preserve_float16,
      shader_signed_zero_inf_nan_preserve_float32: value
        .shader_signed_zero_inf_nan_preserve_float32,
      shader_signed_zero_inf_nan_preserve_float64: value
        .shader_signed_zero_inf_nan_preserve_float64,
      shader_denorm_preserve_float16: value.shader_denorm_preserve_float16,
      shader_denorm_preserve_float32: value.shader_denorm_preserve_float32,
      shader_denorm_preserve_float64: value.shader_denorm_preserve_float64,
      shader_denorm_flush_to_zero_float16: value.shader_denorm_flush_to_zero_float16,
      shader_denorm_flush_to_zero_float32: value.shader_denorm_flush_to_zero_float32,
      shader_denorm_flush_to_zero_float64: value.shader_denorm_flush_to_zero_float64,
      shader_rounding_mode_rte_float16: value.shader_rounding_mode_rte_float16,
      shader_rounding_mode_rte_float32: value.shader_rounding_mode_rte_float32,
      shader_rounding_mode_rte_float64: value.shader_rounding_mode_rte_float64,
      shader_rounding_mode_rtz_float16: value.shader_rounding_mode_rtz_float16,
      shader_rounding_mode_rtz_float32: value.shader_rounding_mode_rtz_float32,
      shader_rounding_mode_rtz_float64: value.shader_rounding_mode_rtz_float64,
      max_update_after_bind_descriptors_in_all_pools: value
        .max_update_after_bind_descriptors_in_all_pools,
      shader_uniform_buffer_array_non_uniform_indexing_native: value
        .shader_uniform_buffer_array_non_uniform_indexing_native,
      shader_sampled_image_array_non_uniform_indexing_native: value
        .shader_sampled_image_array_non_uniform_indexing_native,
      shader_storage_buffer_array_non_uniform_indexing_native: value
        .shader_storage_buffer_array_non_uniform_indexing_native,
      shader_storage_image_array_non_uniform_indexing_native: value
        .shader_storage_image_array_non_uniform_indexing_native,
      shader_input_attachment_array_non_uniform_indexing_native: value
        .shader_input_attachment_array_non_uniform_indexing_native,
      robust_buffer_access_update_after_bind: value.robust_buffer_access_update_after_bind,
      quad_divergent_implicit_lod: value.quad_divergent_implicit_lod,
      max_per_stage_descriptor_update_after_bind_samplers: value
        .max_per_stage_descriptor_update_after_bind_samplers,
      max_per_stage_descriptor_update_after_bind_uniform_buffers: value
        .max_per_stage_descriptor_update_after_bind_uniform_buffers,
      max_per_stage_descriptor_update_after_bind_storage_buffers: value
        .max_per_stage_descriptor_update_after_bind_storage_buffers,
      max_per_stage_descriptor_update_after_bind_sampled_images: value
        .max_per_stage_descriptor_update_after_bind_sampled_images,
      max_per_stage_descriptor_update_after_bind_storage_images: value
        .max_per_stage_descriptor_update_after_bind_storage_images,
      max_per_stage_descriptor_update_after_bind_input_attachments: value
        .max_per_stage_descriptor_update_after_bind_input_attachments,
      max_per_stage_update_after_bind_resources: value.max_per_stage_update_after_bind_resources,
      max_descriptor_set_update_after_bind_samplers: value
        .max_descriptor_set_update_after_bind_samplers,
      max_descriptor_set_update_after_bind_uniform_buffers: value
        .max_descriptor_set_update_after_bind_uniform_buffers,
      max_descriptor_set_update_after_bind_uniform_buffers_dynamic: value
        .max_descriptor_set_update_after_bind_uniform_buffers_dynamic,
      max_descriptor_set_update_after_bind_storage_buffers: value
        .max_descriptor_set_update_after_bind_storage_buffers,
      max_descriptor_set_update_after_bind_storage_buffers_dynamic: value
        .max_descriptor_set_update_after_bind_storage_buffers_dynamic,
      max_descriptor_set_update_after_bind_sampled_images: value
        .max_descriptor_set_update_after_bind_sampled_images,
      max_descriptor_set_update_after_bind_storage_images: value
        .max_descriptor_set_update_after_bind_storage_images,
      max_descriptor_set_update_after_bind_input_attachments: value
        .max_descriptor_set_update_after_bind_input_attachments,
      supported_depth_resolve_modes: value.supported_depth_resolve_modes,
      supported_stencil_resolve_modes: value.supported_stencil_resolve_modes,
      independent_resolve_none: value.independent_resolve_none,
      independent_resolve: value.independent_resolve,
      filter_minmax_single_component_formats: value.filter_minmax_single_component_formats,
      filter_minmax_image_component_mapping: value.filter_minmax_image_component_mapping,
      max_timeline_semaphore_value_difference: value.max_timeline_semaphore_value_difference,
      framebuffer_integer_color_sample_counts: value.framebuffer_integer_color_sample_counts,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct PhysicalDeviceProperties13 {
  pub min_subgroup_size: u32,
  pub max_subgroup_size: u32,
  pub max_compute_workgroup_subgroups: u32,
  pub required_subgroup_size_stages: vk::ShaderStageFlags,
  pub max_inline_uniform_block_size: u32,
  pub max_per_stage_descriptor_inline_uniform_blocks: u32,
  pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
  pub max_descriptor_set_inline_uniform_blocks: u32,
  pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
  pub max_inline_uniform_total_size: u32,
  pub integer_dot_product8_bit_unsigned_accelerated: vk::Bool32,
  pub integer_dot_product8_bit_signed_accelerated: vk::Bool32,
  pub integer_dot_product8_bit_mixed_signedness_accelerated: vk::Bool32,
  pub integer_dot_product4x8_bit_packed_unsigned_accelerated: vk::Bool32,
  pub integer_dot_product4x8_bit_packed_signed_accelerated: vk::Bool32,
  pub integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: vk::Bool32,
  pub integer_dot_product16_bit_unsigned_accelerated: vk::Bool32,
  pub integer_dot_product16_bit_signed_accelerated: vk::Bool32,
  pub integer_dot_product16_bit_mixed_signedness_accelerated: vk::Bool32,
  pub integer_dot_product32_bit_unsigned_accelerated: vk::Bool32,
  pub integer_dot_product32_bit_signed_accelerated: vk::Bool32,
  pub integer_dot_product32_bit_mixed_signedness_accelerated: vk::Bool32,
  pub integer_dot_product64_bit_unsigned_accelerated: vk::Bool32,
  pub integer_dot_product64_bit_signed_accelerated: vk::Bool32,
  pub integer_dot_product64_bit_mixed_signedness_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating8_bit_signed_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated:
    vk::Bool32,
  pub integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating16_bit_signed_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating32_bit_signed_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating64_bit_signed_accelerated: vk::Bool32,
  pub integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: vk::Bool32,
  pub storage_texel_buffer_offset_alignment_bytes: vk::DeviceSize,
  pub storage_texel_buffer_offset_single_texel_alignment: vk::Bool32,
  pub uniform_texel_buffer_offset_alignment_bytes: vk::DeviceSize,
  pub uniform_texel_buffer_offset_single_texel_alignment: vk::Bool32,
  pub max_buffer_size: vk::DeviceSize,
}

impl<'a> From<vk::PhysicalDeviceVulkan13Properties<'a>> for PhysicalDeviceProperties13 {
  fn from(value: vk::PhysicalDeviceVulkan13Properties<'a>) -> Self {
    Self {
      min_subgroup_size: value.min_subgroup_size,
      max_subgroup_size: value.max_subgroup_size,
      max_compute_workgroup_subgroups: value.max_compute_workgroup_subgroups,
      required_subgroup_size_stages: value.required_subgroup_size_stages,
      max_inline_uniform_block_size: value.max_inline_uniform_block_size,
      max_per_stage_descriptor_inline_uniform_blocks: value
        .max_per_stage_descriptor_inline_uniform_blocks,
      max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: value
        .max_per_stage_descriptor_update_after_bind_inline_uniform_blocks,
      max_descriptor_set_inline_uniform_blocks: value.max_descriptor_set_inline_uniform_blocks,
      max_descriptor_set_update_after_bind_inline_uniform_blocks: value
        .max_descriptor_set_update_after_bind_inline_uniform_blocks,
      max_inline_uniform_total_size: value.max_inline_uniform_total_size,
      integer_dot_product8_bit_unsigned_accelerated: value
        .integer_dot_product8_bit_unsigned_accelerated,
      integer_dot_product8_bit_signed_accelerated: value
        .integer_dot_product8_bit_signed_accelerated,
      integer_dot_product8_bit_mixed_signedness_accelerated: value
        .integer_dot_product8_bit_mixed_signedness_accelerated,
      integer_dot_product4x8_bit_packed_unsigned_accelerated: value
        .integer_dot_product4x8_bit_packed_unsigned_accelerated,
      integer_dot_product4x8_bit_packed_signed_accelerated: value
        .integer_dot_product4x8_bit_packed_signed_accelerated,
      integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: value
        .integer_dot_product4x8_bit_packed_mixed_signedness_accelerated,
      integer_dot_product16_bit_unsigned_accelerated: value
        .integer_dot_product16_bit_unsigned_accelerated,
      integer_dot_product16_bit_signed_accelerated: value
        .integer_dot_product16_bit_signed_accelerated,
      integer_dot_product16_bit_mixed_signedness_accelerated: value
        .integer_dot_product16_bit_mixed_signedness_accelerated,
      integer_dot_product32_bit_unsigned_accelerated: value
        .integer_dot_product32_bit_unsigned_accelerated,
      integer_dot_product32_bit_signed_accelerated: value
        .integer_dot_product32_bit_signed_accelerated,
      integer_dot_product32_bit_mixed_signedness_accelerated: value
        .integer_dot_product32_bit_mixed_signedness_accelerated,
      integer_dot_product64_bit_unsigned_accelerated: value
        .integer_dot_product64_bit_unsigned_accelerated,
      integer_dot_product64_bit_signed_accelerated: value
        .integer_dot_product64_bit_signed_accelerated,
      integer_dot_product64_bit_mixed_signedness_accelerated: value
        .integer_dot_product64_bit_mixed_signedness_accelerated,
      integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: value
        .integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated,
      integer_dot_product_accumulating_saturating8_bit_signed_accelerated: value
        .integer_dot_product_accumulating_saturating8_bit_signed_accelerated,
      integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: value
        .integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated,
      integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: value
        .integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated,
      integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: value
        .integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated,
      integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: value
        .integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated,
      integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: value
        .integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated,
      integer_dot_product_accumulating_saturating16_bit_signed_accelerated: value
        .integer_dot_product_accumulating_saturating16_bit_signed_accelerated,
      integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: value
        .integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated,
      integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: value
        .integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated,
      integer_dot_product_accumulating_saturating32_bit_signed_accelerated: value
        .integer_dot_product_accumulating_saturating32_bit_signed_accelerated,
      integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: value
        .integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated,
      integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: value
        .integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated,
      integer_dot_product_accumulating_saturating64_bit_signed_accelerated: value
        .integer_dot_product_accumulating_saturating64_bit_signed_accelerated,
      integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: value
        .integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated,
      storage_texel_buffer_offset_alignment_bytes: value
        .storage_texel_buffer_offset_alignment_bytes,
      storage_texel_buffer_offset_single_texel_alignment: value
        .storage_texel_buffer_offset_single_texel_alignment,
      uniform_texel_buffer_offset_alignment_bytes: value
        .uniform_texel_buffer_offset_alignment_bytes,
      uniform_texel_buffer_offset_single_texel_alignment: value
        .uniform_texel_buffer_offset_single_texel_alignment,
      max_buffer_size: value.max_buffer_size,
    }
  }
}
