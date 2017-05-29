use super::super::super::objc;
use super::types::*;

pub fn ns_make_rect(x: CGFloat, y: CGFloat, w: CGFloat, h: CGFloat) -> NSRect {
    let mut r = NSRect::default();
    r.origin.x = x;
    r.origin.y = y;
    r.size.width = w;
    r.size.height = h;
    return r;
}

#[link(name = "Foundation", kind = "framework")]
extern {
    pub static NSDefaultRunLoopMode: Id;
}

pub fn ns_string_new_with_pool(string: &str) -> Id {
    let s = get_class!("NSString");
    let s: Id = unsafe { msg_send![s, alloc] };
    let s: Id = unsafe { msg_send![s,
        initWithBytes:string.as_ptr()
        length:string.len()
        encoding:NS_UTF8_STRING_ENCODING]};
    return s;
}
