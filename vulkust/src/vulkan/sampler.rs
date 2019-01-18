use super::super::render::sampler::Filter;
use super::device::Logical as LogicalDevice;
use ash::version::DeviceV1_0;
use ash::vk;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Sampler {
    logical_device: Arc<LogicalDevice>,
    vk_data: vk::Sampler,
}

impl Sampler {
    pub(crate) fn new(logical_device: Arc<LogicalDevice>) -> Self {
        return Self::new_with_filter(logical_device, Filter::Linear);
    }

    pub(crate) fn new_with_filter(logical_device: Arc<LogicalDevice>, f: Filter) -> Self {
        let filter = Self::convert_filter(f);
        let mut info = vk::SamplerCreateInfo::builder()
            .border_color(vk::BorderColor::FLOAT_TRANSPARENT_BLACK)
            .unnormalized_coordinates(false)
            .compare_op(vk::CompareOp::ALWAYS)
            .mag_filter(filter)
            .min_filter(filter)
            .address_mode_u(vk::SamplerAddressMode::REPEAT)
            .address_mode_v(vk::SamplerAddressMode::REPEAT)
            .address_mode_w(vk::SamplerAddressMode::REPEAT);
        match f {
            Filter::Nearest => {
                info = info
                    .anisotropy_enable(false)
                    .max_anisotropy(0.0)
                    .compare_enable(false)
                    .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
                    .mip_lod_bias(0.0)
                    .min_lod(0.0)
                    .max_lod(0.0)
            }
            _ => {
                info = info
                    .anisotropy_enable(true)
                    .max_anisotropy(
                        logical_device
                            .get_physical()
                            .get_properties()
                            .limits
                            .max_sampler_anisotropy,
                    )
                    .compare_enable(false)
                    .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
                    .mip_lod_bias(0.0)
                    .min_lod(0.0)
                    .max_lod(0.0)
            }
        }
        let mut vk_data =
            vxresult!(unsafe { logical_device.get_data().create_sampler(&info, None) });
        Self {
            logical_device,
            vk_data,
        }
    }

    pub(crate) fn get_data(&self) -> &vk::Sampler {
        return &self.vk_data;
    }

    pub(super) fn convert_filter(f: Filter) -> vk::Filter {
        match f {
            Filter::Nearest => vk::Filter::NEAREST,
            Filter::Linear => vk::Filter::LINEAR,
            Filter::Cube => vk::Filter::CUBIC_IMG,
        }
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .get_data()
                .destroy_sampler(self.vk_data, None);
        }
    }
}
