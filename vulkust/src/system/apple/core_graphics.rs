#[cfg(target_pointer_width = "32")]
pub type CGFloat = f32;
#[cfg(target_pointer_width = "64")]
pub type CGFloat = f64;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

unsafe impl objc::Encode for CGSize {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{CGSize={}{}}}",
            CGFloat::encode().as_str(),
            CGFloat::encode().as_str()
        );
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

pub fn make_size(w: CGFloat, h: CGFloat) -> CGSize {
    CGSize {
        width: w,
        height: h,
    }
}
