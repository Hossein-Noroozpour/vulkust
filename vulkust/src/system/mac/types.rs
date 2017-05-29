use super::super::super::objc;

pub type Id = *mut objc::runtime::Object;

#[cfg(target_pointer_width = "32")]
pub type NSInteger = i32;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = u32;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = i64;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = u64;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NSSize {
    pub width: f64,
    pub height: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NSPoint {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
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
