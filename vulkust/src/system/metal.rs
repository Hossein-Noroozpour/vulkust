use std::os::raw::{c_void};
use std::mem::transmute;
use super::super::objc;
use super::super::objc::runtime::{Object, Class};
use super::super::objc::declare::{ClassDecl};

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

// const ------------------------------------------------------------------------------------------

pub const NS_AUTO_RELEASE_POOL: &str = "NSAutoreleasePool";
pub const RESOURCE_CPU_CACHE_MODE_SHIFT: NSUInteger = 0;
pub const RESOURCE_CPU_CACHE_MODE_MASK: NSUInteger = 0xF << RESOURCE_CPU_CACHE_MODE_SHIFT;
pub const RESOURCE_STORAGE_MODE_SHIFT: NSUInteger = 4;
pub const RESOURCE_STORAGE_MODE_MASK: NSUInteger = 0xF << RESOURCE_STORAGE_MODE_SHIFT;
pub const RESOURCE_HAZARD_TRACKING_MODE_SHIFT: NSUInteger = 8;
pub const RESOURCE_HAZARD_TRACKING_MODE_MASK: NSUInteger =
    0x1 << RESOURCE_HAZARD_TRACKING_MODE_SHIFT;

// structs ----------------------------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

unsafe impl objc::Encode for ClearColor {
    fn encode() -> objc::Encoding { unsafe { objc::Encoding::from_str("{?=dddd}") } }
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

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct NSSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

unsafe impl objc::Encode for NSSize {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{CGSize={}{}}}", CGFloat::encode().as_str(), CGFloat::encode().as_str());
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct NSPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

unsafe impl objc::Encode for NSPoint {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{CGPoint={}{}}}", CGFloat::encode().as_str(), CGFloat::encode().as_str());
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
}

unsafe impl objc::Encode for NSRect {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{CGRect={}{}}}", NSPoint::encode().as_str(), NSSize::encode().as_str());
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

impl NSRect {
    pub fn new(x: CGFloat, y: CGFloat, w: CGFloat, h: CGFloat) -> Self {
        NSRect {
            origin: NSPoint {
                x: x,
                y: y,
            },
            size: NSSize {
                width: w,
                height: h,
            },
        }
    }
}

// TODO: I don't like auto release pool try to remove it, and you must right the drop
pub struct NSString {
    pub s: Id,
}

impl NSString {
    pub fn new(string: &str) -> Self {
        let s: Id = alloc("NSString");
        let s: Id = unsafe {
            let string_ptr: *const c_void = transmute(string.as_ptr());
            msg_send![
                s,
                initWithBytes:string_ptr
                length:string.len()
                encoding:NS_UTF8_STRING_ENCODING]
        };
        NSString {
            s: s
        }
    }
}

#[repr(C)]
pub struct IdPtr {
    pub id: *mut Id,
}

unsafe impl objc::Encode for IdPtr {
    fn encode() -> objc::Encoding { unsafe { objc::Encoding::from_str("^@") } }
}

// TODO: remove auto release pool in future.
// impl Drop for NSString {
//     fn drop(&mut self) {
//
//     }
// }

// enums ------------------------------------------------------------------------------------------

bitflags! {
    pub struct CPUCacheMode: NSUInteger {
        const CPU_CACHE_MODE_DEFAULT_CACHE = 0;
        const CPU_CACHE_MODE_WRITE_COMBINED = 1;
    }
}

bitflags! {
    pub struct StorageMode: NSUInteger {
        const STORAGE_MODE_SHARED = 0;
        const STORAGE_MODE_MANAGED = 1;
        const STORAGE_MODE_PRIVATE = 2;
        const STORAGE_MODE_MEMORYLESS = 3;
    }
}

bitflags! {
    pub struct ResourceOptions: NSUInteger {
        const RESOURCE_CPU_CACHE_MODE_DEFAULT_CACHE =
            CPU_CACHE_MODE_DEFAULT_CACHE.bits << RESOURCE_CPU_CACHE_MODE_SHIFT;
        const RESOURCE_CPU_CACHE_MODE_WRITE_COMBINED =
            CPU_CACHE_MODE_WRITE_COMBINED.bits << RESOURCE_CPU_CACHE_MODE_SHIFT;
        const RESOURCE_STORAGE_MODE_SHARED =
            STORAGE_MODE_SHARED.bits << RESOURCE_STORAGE_MODE_SHIFT;
        const RESOURCE_STORAGE_MODE_MANAGED =
            STORAGE_MODE_MANAGED.bits << RESOURCE_STORAGE_MODE_SHIFT;
        const RESOURCE_STORAGE_MODE_PRIVATE =
            STORAGE_MODE_PRIVATE.bits << RESOURCE_STORAGE_MODE_SHIFT;
        const RESOURCE_STORAGE_MODE_MEMORYLESS =
            STORAGE_MODE_MEMORYLESS.bits << RESOURCE_STORAGE_MODE_SHIFT;
        const RESOURCE_HAZARD_TRACKING_MODE_UNTRACKED = 0x1 << RESOURCE_HAZARD_TRACKING_MODE_SHIFT;
        const RESOURCE_OPTION_CPU_CACHE_MODE_DEFAULT =
            RESOURCE_CPU_CACHE_MODE_DEFAULT_CACHE.bits;
        const RESOURCE_OPTION_CPU_CACHE_MODE_WRITE_COMBINED =
            RESOURCE_CPU_CACHE_MODE_WRITE_COMBINED.bits;
    }
}

unsafe impl objc::Encode for ResourceOptions {
    fn encode() -> objc::Encoding { NSUInteger::encode() }
}

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
        const PIXEL_FORMAT_BGR10_XR                             = 554;
        const PIXEL_FORMAT_BGR10_XR_SRGB                        = 555;
        const PIXEL_FORMAT_RG32_UINT                            = 103;
        const PIXEL_FORMAT_RG32_SINT                            = 104;
        const PIXEL_FORMAT_RG32_FLOAT                           = 105;
        const PIXEL_FORMAT_RGBA16_UNORM                         = 110;
        const PIXEL_FORMAT_RGBA16_SNORM                         = 112;
        const PIXEL_FORMAT_RGBA16_UINT                          = 113;
        const PIXEL_FORMAT_RGBA16_SINT                          = 114;
        const PIXEL_FORMAT_RGBA16_FLOAT                         = 115;
        const PIXEL_FORMAT_BGRA10_XR                            = 552;
        const PIXEL_FORMAT_BGRA10_XR_SRGB                       = 553;
        const PIXEL_FORMAT_RGBA32_UINT                          = 123;
        const PIXEL_FORMAT_RGBA32_SINT                          = 124;
        const PIXEL_FORMAT_RGBA32_FLOAT                         = 125;
        const PIXEL_FORMAT_BC1_RGBA                             = 130;
        const PIXEL_FORMAT_BC1_RGBA_SRGB                        = 131;
        const PIXEL_FORMAT_BC2_RGBA                             = 132;
        const PIXEL_FORMAT_BC2_RGBA_SRGB                        = 133;
        const PIXEL_FORMAT_BC3_RGBA                             = 134;
        const PIXEL_FORMAT_BC3_RGBA_SRGB                        = 135;
        const PIXEL_FORMAT_BC4_R_UNORM                          = 140;
        const PIXEL_FORMAT_BC4_R_SNORM                          = 141;
        const PIXEL_FORMAT_BC5_RG_UNORM                         = 142;
        const PIXEL_FORMAT_BC5_RG_SNORM                         = 143;
        const PIXEL_FORMAT_BC6H_RGB_FLOAT                       = 150;
        const PIXEL_FORMAT_BC6H_RGBU_FLOAT                      = 151;
        const PIXEL_FORMAT_BC7_RGBA_UNORM                       = 152;
        const PIXEL_FORMAT_BC7_RGBA_UNORM_SRGB                  = 153;
        const PIXEL_FORMAT_PVRTC_RGB_2BPP                       = 160;
        const PIXEL_FORMAT_PVRTC_RGB_2BPP_SRGB                  = 161;
        const PIXEL_FORMAT_PVRTC_RGB_4BPP                       = 162;
        const PIXEL_FORMAT_PVRTC_RGB_4BPP_SRGB                  = 163;
        const PIXEL_FORMAT_PVRTC_RGBA_2BPP                      = 164;
        const PIXEL_FORMAT_PVRTC_RGBA_2BPP_SRGB                 = 165;
        const PIXEL_FORMAT_PVRTC_RGBA_4BPP                      = 166;
        const PIXEL_FORMAT_PVRTC_RGBA_4BPP_SRGB                 = 167;
        const PIXEL_FORMAT_EAC_R11_UNORM                        = 170;
        const PIXEL_FORMAT_EAC_R11_SNORM                        = 172;
        const PIXEL_FORMAT_EAC_RG11_UNORM                       = 174;
        const PIXEL_FORMAT_EAC_RG11_SNORM                       = 176;
        const PIXEL_FORMAT_EAC_RGBA8                            = 178;
        const PIXEL_FORMAT_EAC_RGBA8_SRGB                       = 179;
        const PIXEL_FORMAT_ETC2_RGB8                            = 180;
        const PIXEL_FORMAT_ETC2_RGB8_SRGB                       = 181;
        const PIXEL_FORMAT_ETC2_RGB8A1                          = 182;
        const PIXEL_FORMAT_ETC2_RGB8A1_SRGB                     = 183;
        const PIXEL_FORMAT_ASTC_4X4_SRGB                        = 186;
        const PIXEL_FORMAT_ASTC_5X4_SRGB                        = 187;
        const PIXEL_FORMAT_ASTC_5X5_SRGB                        = 188;
        const PIXEL_FORMAT_ASTC_6X5_SRGB                        = 189;
        const PIXEL_FORMAT_ASTC_6X6_SRGB                        = 190;
        const PIXEL_FORMAT_ASTC_8X5_SRGB                        = 192;
        const PIXEL_FORMAT_ASTC_8X6_SRGB                        = 193;
        const PIXEL_FORMAT_ASTC_8X8_SRGB                        = 194;
        const PIXEL_FORMAT_ASTC_10X5_SRGB                       = 195;
        const PIXEL_FORMAT_ASTC_10X6_SRGB                       = 196;
        const PIXEL_FORMAT_ASTC_10X8_SRGB                       = 197;
        const PIXEL_FORMAT_ASTC_10X10_SRGB                      = 198;
        const PIXEL_FORMAT_ASTC_12X10_SRGB                      = 199;
        const PIXEL_FORMAT_ASTC_12X12_SRGB                      = 200;
        const PIXEL_FORMAT_ASTC_4X4_LDR                         = 204;
        const PIXEL_FORMAT_ASTC_5X4_LDR                         = 205;
        const PIXEL_FORMAT_ASTC_5X5_LDR                         = 206;
        const PIXEL_FORMAT_ASTC_6X5_LDR                         = 207;
        const PIXEL_FORMAT_ASTC_6X6_LDR                         = 208;
        const PIXEL_FORMAT_ASTC_8X5_LDR                         = 210;
        const PIXEL_FORMAT_ASTC_8X6_LDR                         = 211;
        const PIXEL_FORMAT_ASTC_8X8_LDR                         = 212;
        const PIXEL_FORMAT_ASTC_10X5_LDR                        = 213;
        const PIXEL_FORMAT_ASTC_10X6_LDR                        = 214;
        const PIXEL_FORMAT_ASTC_10X8_LDR                        = 215;
        const PIXEL_FORMAT_ASTC_10X10_LDR                       = 216;
        const PIXEL_FORMAT_ASTC_12X10_LDR                       = 217;
        const PIXEL_FORMAT_ASTC_12X12_LDR                       = 218;
        const PIXEL_FORMAT_GBGR422                              = 240;
        const PIXEL_FORMAT_BGRG422                              = 241;
        const PIXEL_FORMAT_DEPTH16_UNORM                        = 250;
        const PIXEL_FORMAT_DEPTH32_FLOAT                        = 252;
        const PIXEL_FORMAT_STENCIL8                             = 253;
        const PIXEL_FORMAT_DEPTH24_UNORM_STENCIL8               = 255;
        const PIXEL_FORMAT_DEPTH32_FLOAT_STENCIL8               = 260;
        const PIXEL_FORMAT_X32_STENCIL8                         = 261;
        const PIXEL_FORMAT_X24_STENCIL8                         = 262;
    }
}

unsafe impl objc::Encode for PixelFormat {
    fn encode() -> objc::Encoding { NSUInteger::encode() }
}

bitflags! {
    pub struct VertexFormat: NSUInteger {
        const VERTEX_FORMAT_INVALID = 0;
        const VERTEX_FORMAT_UCHAR2 = 1;
        const VERTEX_FORMAT_UCHAR3 = 2;
        const VERTEX_FORMAT_UCHAR4 = 3;
        const VERTEX_FORMAT_CHAR2 = 4;
        const VERTEX_FORMAT_CHAR3 = 5;
        const VERTEX_FORMAT_CHAR4 = 6;
        const VERTEX_FORMAT_UCHAR2_NORMALIZED = 7;
        const VERTEX_FORMAT_UCHAR3_NORMALIZED = 8;
        const VERTEX_FORMAT_UCHAR4_NORMALIZED = 9;
        const VERTEX_FORMAT_CHAR2_NORMALIZED = 10;
        const VERTEX_FORMAT_CHAR3_NORMALIZED = 11;
        const VERTEX_FORMAT_CHAR4_NORMALIZED = 12;
        const VERTEX_FORMAT_USHORT2 = 13;
        const VERTEX_FORMAT_USHORT3 = 14;
        const VERTEX_FORMAT_USHORT4 = 15;
        const VERTEX_FORMAT_SHORT2 = 16;
        const VERTEX_FORMAT_SHORT3 = 17;
        const VERTEX_FORMAT_SHORT4 = 18;
        const VERTEX_FORMAT_USHORT2_NORMALIZED = 19;
        const VERTEX_FORMAT_USHORT3_NORMALIZED = 20;
        const VERTEX_FORMAT_USHORT4_NORMALIZED = 21;
        const VERTEX_FORMAT_SHORT2_NORMALIZED = 22;
        const VERTEX_FORMAT_SHORT3_NORMALIZED = 23;
        const VERTEX_FORMAT_SHORT4_NORMALIZED = 24;
        const VERTEX_FORMAT_HALF2 = 25;
        const VERTEX_FORMAT_HALF3 = 26;
        const VERTEX_FORMAT_HALF4 = 27;
        const VERTEX_FORMAT_FLOAT = 28;
        const VERTEX_FORMAT_FLOAT2 = 29;
        const VERTEX_FORMAT_FLOAT3 = 30;
        const VERTEX_FORMAT_FLOAT4 = 31;
        const VERTEX_FORMAT_INT = 32;
        const VERTEX_FORMAT_INT2 = 33;
        const VERTEX_FORMAT_INT3 = 34;
        const VERTEX_FORMAT_INT4 = 35;
        const VERTEX_FORMAT_UINT = 36;
        const VERTEX_FORMAT_UINT2 = 37;
        const VERTEX_FORMAT_UINT3 = 38;
        const VERTEX_FORMAT_UINT4 = 39;
        const VERTEX_FORMAT_INT1010102_NORMALIZED = 40;
        const VERTEX_FORMAT_UINT1010102_NORMALIZED = 41;
    }
}

unsafe impl objc::Encode for VertexFormat {
    fn encode() -> objc::Encoding { NSUInteger::encode() }
}

bitflags! {
    pub struct VertexStepFunction: NSUInteger {
        const VERTEX_STEP_FUNCTION_CONSTANT                = 0;
        const VERTEX_STEP_FUNCTION_PER_VERTEX              = 1;
        const VERTEX_STEP_FUNCTION_PER_INSTANCE            = 2;
        const VERTEX_STEP_FUNCTION_PER_PATCH               = 3;
        const VERTEX_STEP_FUNCTION_PER_PATCH_CONTROL_POINT = 4;
    }
}

unsafe impl objc::Encode for VertexStepFunction {
    fn encode() -> objc::Encoding { NSUInteger::encode() }
}

bitflags! {
    pub struct NsWindowStyleMask: NSUInteger {
        const NS_BORDERLESS_WINDOW_MASK                 = 0;
        const NS_TITLED_WINDOW_MASK                     = 1 << 0;
        const NS_CLOSABLE_WINDOW_MASK                   = 1 << 1;
        const NS_MINIATURIZABLE_WINDOW_MASK             = 1 << 2;
        const NS_RESIZABLE_WINDOW_MASK                  = 1 << 3;
        const NS_TEXTURED_BACKGROUND_WINDOW_MASK        = 1 << 8;
        const NS_UNIFIED_TITLE_AND_TOOLBAR_WINDOW_MASK  = 1 << 12;
        const NS_FULLSCREEN_WINDOW_MASK                 = 1 << 14;
        const NS_FULLSIZE_CONTENT_VIEW_WINDOW_MASK      = 1 << 15;
    }
}

bitflags! {
    pub struct NsBackingStoreType: NSUInteger {
        const NS_BACKING_STORE_RETAINED    = 0;
        const NS_BACKING_STORE_NONRETAINED = 1;
        const NS_BACKING_STORE_BUFFERED    = 2;
    }
}

unsafe impl objc::Encode for NsBackingStoreType {
    fn encode() -> objc::Encoding { NSUInteger::encode() }
}

bitflags! {
    pub struct NsStringEncoding: NSInteger {
        const NS_ASCII_STRING_ENCODING = 1;
        const NS_NEXTSTEP_STRING_ENCODING = 2;
        const NS_JAPANESE_EUC_STRING_ENCODING = 3;
        const NS_UTF8_STRING_ENCODING = 4;
        const NS_ISO_LATIN1_STRING_ENCODING = 5;
        const NS_SYMBOL_STRING_ENCODING = 6;
        const NS_NON_LOSSY_ASCII_STRING_ENCODING = 7;
        const NS_SHIFT_JIS_STRING_ENCODING = 8;
        const NS_ISOLATIN2_STRING_ENCODING = 9;
        const NS_UNICODE_STRING_ENCODING = 10;
        const NS_WINDOWS_CP1251_STRING_ENCODING = 11;
        const NS_WINDOWS_CP1252_STRING_ENCODING = 12;
        const NS_WINDOWS_CP1253_STRING_ENCODING = 13;
        const NS_WINDOWS_CP1254_STRING_ENCODING = 14;
        const NS_WINDOWS_CP1250_STRING_ENCODING = 15;
        const NS_ISO2022JP_STRING_ENCODING = 21;
        const NS_MACOS_ROMAN_STRING_ENCODING = 30;
        const NS_UTF16_STRING_ENCODING = NS_UNICODE_STRING_ENCODING.bits;
        const NS_UTF16_BIG_ENDIAN_STRING_ENCODING = 0x90000100;
        const NS_UTF16_LITTLE_ENDIAN_STRING_ENCODING = 0x94000100;
        const NS_UTF32_STRING_ENCODING = 0x8c000100;
        const NS_UTF32_BIG_ENDIAN_STRING_ENCODING = 0x98000100;
        const NS_UTF32_LITTLE_ENDIAN_STRING_ENCODING = 0x9c000100;
    }
}

unsafe impl objc::Encode for NsStringEncoding {
    fn encode() -> objc::Encoding { NSUInteger::encode() }
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

#[link(name = "Foundation", kind = "framework")]
extern {
    // pub static NSDefaultRunLoopMode: mtl::Id;
}

#[link(name = "AppKit", kind = "framework")]
extern {
    // pub static NSImageHintCTM: Id;
}

// Rustified ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// functions --------------------------------------------------------------------------------------

pub fn get_class(s: &str) -> &Class {
    match Class::get(s) {
        Some(c) => { c },
        None => { logf!("Class: {:?} does not exist.", s); },
    }
}

pub fn get_instance(s: &str) -> Id {
    let c = match Class::get(s) {
        Some(c) => { c },
        None => { logf!("Class: {:?} does not exist.", s); },
    };
    let r: Id = unsafe { msg_send![c, alloc] };
    let r: Id = unsafe { msg_send![r, init] };
    return r;
}

pub fn dec_class(s: &str, c: &Class) -> ClassDecl {
    match ClassDecl::new(s, c) {
        Some(c) => { c },
        None => {
            logf!("Can not create class {} with super class {:?}.", s, c);
        },
    }
}

pub fn dec_class_s(s: &str, c: &str) -> ClassDecl {
    let c = match Class::get(c) {
        Some(c) => { c },
        None => { logf!("Class: {} does not exist.", c); },
    };
    match ClassDecl::new(s, c) {
        Some(c) => { c },
        None => {
            logf!("Can not create class {} with super class {:?}.", s, c);
        },
    }
}

pub fn set_ivar<T>(id: Id, name: &str, value: T) where T: objc::Encode {
    unsafe { (*id).set_ivar(name, value); }
}

pub fn alloc(s: &str) -> Id {
    let c = match Class::get(s) {
        Some(c) => { c },
        None => { logf!("Class: {:?} does not exist.", s); },
    };
    unsafe { msg_send![c, alloc] }
}

pub fn create_system_default_device() -> Id {
    unsafe {
        MTLCreateSystemDefaultDevice()
    }
}

// struct -----------------------------------------------------------------------------------------

pub struct NsAutoReleasePool {
    pool: Id
}

impl NsAutoReleasePool {
    pub fn new() -> Self {
        NsAutoReleasePool {
            pool: get_instance(NS_AUTO_RELEASE_POOL),
        }
    }
}

impl Drop for NsAutoReleasePool {
    fn drop(&mut self) {
        unsafe {
            msg_send![self.pool, drain];
        }
    }
}
