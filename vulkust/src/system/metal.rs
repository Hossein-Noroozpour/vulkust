use super::super::objc::runtime::Object;

// types ------------------------------------------------------------------------------------------

#[cfg(target_pointer_width = "32")]
pub type NSInteger = i32;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = u32;
#[cfg(target_pointer_width = "32")]
pub type CGFloat = f32;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = i64;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = u64;
#[cfg(target_pointer_width = "64")]
pub type CGFloat = f64;

pub type Id = *mut Object;

// structs ----------------------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl ClearColor {
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        ClearColor {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }
}

// enums ------------------------------------------------------------------------------------------

bitflags! {
    pub struct PixelFormat: NSUInteger {
        const PIXEL_FORMAT_INVALID                              = 0;
        const PIXEL_FORMAT_A8_UNORM                             = 1;
        const PIXEL_FORMAT_R8_UNORM                             = 10;
        const PIXEL_FORMAT_R8_UNORM_SRGB                        = 11;
        const PIXEL_FORMAT_R8_SNORM                             = 12;
        const PIXEL_FORMAT_R8_UINT                              = 13;
        const PIXEL_FORMAT_R8_SINT                              = 14;
        const PIXEL_FORMAT_R16_UNORM                            = 20;
        const PIXEL_FORMAT_R16_SNORM                            = 22;
        const PIXEL_FORMAT_R16_UINT                             = 23;
        const PIXEL_FORMAT_R16_SINT                             = 24;
        const PIXEL_FORMAT_R16_FLOAT                            = 25;
        const PIXEL_FORMAT_RG8_UNORM                            = 30;
        const PIXEL_FORMAT_RG8_UNORM_SRGB                       = 31;
        const PIXEL_FORMAT_RG8_SNORM                            = 32;
        const PIXEL_FORMAT_RG8_UINT                             = 33;
        const PIXEL_FORMAT_RG8_SINT                             = 34;
        const PIXEL_FORMAT_B5G6R5_UNORM                         = 40;
        const PIXEL_FORMAT_A1BGR5_UNORM                         = 41;
        const PIXEL_FORMAT_ABGR4_UNORM                          = 42;
        const PIXEL_FORMAT_BGR5A1_UNORM                         = 43;
        const PIXEL_FORMAT_R32_UINT                             = 53;
        const PIXEL_FORMAT_R32_SINT                             = 54;
        const PIXEL_FORMAT_R32_FLOAT                            = 55;
        const PIXEL_FORMAT_RG16_UNORM                           = 60;
        const PIXEL_FORMAT_RG16_SNORM                           = 62;
        const PIXEL_FORMAT_RG16_UINT                            = 63;
        const PIXEL_FORMAT_RG16_SINT                            = 64;
        const PIXEL_FORMAT_RG16_FLOAT                           = 65;
        const PIXEL_FORMAT_RGBA8_UNORM                          = 70;
        const PIXEL_FORMAT_RGBA8_UNORM_SRGB                     = 71;
        const PIXEL_FORMAT_RGBA8_SNORM                          = 72;
        const PIXEL_FORMAT_RGBA8_UINT                           = 73;
        const PIXEL_FORMAT_RGBA8_SINT                           = 74;
        const PIXEL_FORMAT_BGRA8_UNORM                          = 80;
        const PIXEL_FORMAT_BGRA8_UNORM_SRGB                     = 81;
        const PIXEL_FORMAT_RGB10A2_UNORM                        = 90;
        const PIXEL_FORMAT_RGB10A2_UINT                         = 91;
        const PIXEL_FORMAT_RG11B10_FLOAT                        = 92;
        const PIXEL_FORMAT_RGB9E5_FLOAT                         = 93;
        const PIXEL_FORMAT_BGR10_XR                            = 554;
        const PIXEL_FORMAT_BGR10_XR_SRGB                       = 555;
        const PIXEL_FORMAT_RG32_UINT                            = 103;
        const PIXEL_FORMAT_RG32_SINT                            = 104;
        const PIXEL_FORMAT_RG32_FLOAT                           = 105;
        const PIXEL_FORMAT_RGBA16_UNORM                         = 110;
        const PIXEL_FORMAT_RGBA16_SNORM                         = 112;
        const PIXEL_FORMAT_RGBA16_UINT                          = 113;
        const PIXEL_FORMAT_RGBA16_SINT                          = 114;
        const PIXEL_FORMAT_RGBA16_FLOAT                         = 115;
        const PIXEL_FORMAT_BGRA10_XR                           = 552;
        const PIXEL_FORMAT_BGRA10_XR_SRGB                      = 553;
        const PIXEL_FORMAT_RGBA32_UINT                          = 123;
        const PIXEL_FORMAT_RGBA32_SINT                          = 124;
        const PIXEL_FORMAT_RGBA32_FLOAT                         = 125;
        const PIXEL_FORMAT_BC1_RGBA                            = 130;
        const PIXEL_FORMAT_BC1_RGBA_SRGB                       = 131;
        const PIXEL_FORMAT_BC2_RGBA                            = 132;
        const PIXEL_FORMAT_BC2_RGBA_SRGB                       = 133;
        const PIXEL_FORMAT_BC3_RGBA                            = 134;
        const PIXEL_FORMAT_BC3_RGBA_SRGB                       = 135;
        const PIXEL_FORMAT_BC4_R_UNORM                          = 140;
        const PIXEL_FORMAT_BC4_R_SNORM                          = 141;
        const PIXEL_FORMAT_BC5_RG_UNORM                         = 142;
        const PIXEL_FORMAT_BC5_RG_SNORM                         = 143;
        const PIXEL_FORMAT_BC6H_RGB_FLOAT                       = 150;
        const PIXEL_FORMAT_BC6H_RGBU_FLOAT                      = 151;
        const PIXEL_FORMAT_BC7_RGBA_UNORM                       = 152;
        const PIXEL_FORMAT_BC7_RGBA_UNORM_SRGB                  = 153;
        const PIXEL_FORMAT_PVRTC_RGB_2BPP                      = 160;
        const PIXEL_FORMAT_PVRTC_RGB_2BPP_SRGB                 = 161;
        const PIXEL_FORMAT_PVRTC_RGB_4BPP                      = 162;
        const PIXEL_FORMAT_PVRTC_RGB_4BPP_SRGB                 = 163;
        const PIXEL_FORMAT_PVRTC_RGBA_2BPP                     = 164;
        const PIXEL_FORMAT_PVRTC_RGBA_2BPP_SRGB                = 165;
        const PIXEL_FORMAT_PVRTC_RGBA_4BPP                     = 166;
        const PIXEL_FORMAT_PVRTC_RGBA_4BPP_SRGB                = 167;
        const PIXEL_FORMAT_EAC_R11_UNORM                        = 170;
        const PIXEL_FORMAT_EAC_R11_SNORM                        = 172;
        const PIXEL_FORMAT_EAC_RG11_UNORM                       = 174;
        const PIXEL_FORMAT_EAC_RG11_SNORM                       = 176;
        const PIXEL_FORMAT_EAC_RGBA8                           = 178;
        const PIXEL_FORMAT_EAC_RGBA8_SRGB                      = 179;
        const PIXEL_FORMAT_ETC2_RGB8                           = 180;
        const PIXEL_FORMAT_ETC2_RGB8_SRGB                      = 181;
        const PIXEL_FORMAT_ETC2_RGB8A1                         = 182;
        const PIXEL_FORMAT_ETC2_RGB8A1_SRGB                    = 183;
        const PIXEL_FORMAT_ASTC_4X4_SRGB                       = 186;
        const PIXEL_FORMAT_ASTC_5X4_SRGB                       = 187;
        const PIXEL_FORMAT_ASTC_5X5_SRGB                       = 188;
        const PIXEL_FORMAT_ASTC_6X5_SRGB                       = 189;
        const PIXEL_FORMAT_ASTC_6X6_SRGB                       = 190;
        const PIXEL_FORMAT_ASTC_8X5_SRGB                       = 192;
        const PIXEL_FORMAT_ASTC_8X6_SRGB                       = 193;
        const PIXEL_FORMAT_ASTC_8X8_SRGB                       = 194;
        const PIXEL_FORMAT_ASTC_10X5_SRGB                      = 195;
        const PIXEL_FORMAT_ASTC_10X6_SRGB                      = 196;
        const PIXEL_FORMAT_ASTC_10X8_SRGB                      = 197;
        const PIXEL_FORMAT_ASTC_10X10_SRGB                     = 198;
        const PIXEL_FORMAT_ASTC_12X10_SRGB                     = 199;
        const PIXEL_FORMAT_ASTC_12X12_SRGB                     = 200;
        const PIXEL_FORMAT_ASTC_4X4_LDR                        = 204;
        const PIXEL_FORMAT_ASTC_5X4_LDR                        = 205;
        const PIXEL_FORMAT_ASTC_5X5_LDR                        = 206;
        const PIXEL_FORMAT_ASTC_6X5_LDR                        = 207;
        const PIXEL_FORMAT_ASTC_6X6_LDR                        = 208;
        const PIXEL_FORMAT_ASTC_8X5_LDR                        = 210;
        const PIXEL_FORMAT_ASTC_8X6_LDR                        = 211;
        const PIXEL_FORMAT_ASTC_8X8_LDR                        = 212;
        const PIXEL_FORMAT_ASTC_10X5_LDR                       = 213;
        const PIXEL_FORMAT_ASTC_10X6_LDR                       = 214;
        const PIXEL_FORMAT_ASTC_10X8_LDR                       = 215;
        const PIXEL_FORMAT_ASTC_10X10_LDR                      = 216;
        const PIXEL_FORMAT_ASTC_12X10_LDR                      = 217;
        const PIXEL_FORMAT_ASTC_12X12_LDR                      = 218;
        const PIXEL_FORMAT_GBGR422                             = 240;
        const PIXEL_FORMAT_BGRG422                             = 241;
        const PIXEL_FORMAT_DEPTH16_UNORM                        = 250;
        const PIXEL_FORMAT_DEPTH32_FLOAT                        = 252;
        const PIXEL_FORMAT_STENCIL8                            = 253;
        const PIXEL_FORMAT_DEPTH24_UNORM_STENCIL8               = 255;
        const PIXEL_FORMAT_DEPTH32_FLOAT_STENCIL8               = 260;
        const PIXEL_FORMAT_X32_STENCIL8                        = 261;
        const PIXEL_FORMAT_X24_STENCIL8                        = 262;
    }
}

// external linkages ------------------------------------------------------------------------------

#[link(name = "Metal", kind = "framework")]
extern "C" {
    fn MTLCreateSystemDefaultDevice() -> Id;
}

#[link(name = "MetalKit", kind = "framework")]
extern "C" {
    // pub fn MTKMetalVertexFormatFromModelIO() -> *mut Object;
}

// functions --------------------------------------------------------------------------------------

pub fn create_system_default_device() -> Id {
    unsafe {
        MTLCreateSystemDefaultDevice()
    }
}
