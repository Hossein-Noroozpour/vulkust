use super::super::super::objc;

pub type Id = *mut objc::runtime::Object;

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
