use super::super::super::objc::runtime::{Class, Object, Sel, BOOL, YES};
use super::super::apple;

pub const CLASS_NAME: &str = "GameView";
pub const SUPER_CLASS_NAME: &str = "UIWindow";

//+ (Class) layerClass
extern "C" fn layer_class(_cls: &mut Class, _cmd: Sel) -> &'static Class {
    unsafe { msg_send![apple::get_class("CAMetalLayer"), class] }
}

pub fn register() {
    let mut self_class = apple::dec_class_s(CLASS_NAME, SUPER_CLASS_NAME);
    unsafe {
        self_class.add_class_method(
            sel!(layerClass),
            layer_class as extern "C" fn(&mut Class, Sel) -> &'static Class,
        );
    }
    self_class.register();
}

pub fn create_instance(frame: apple::NSRect) -> apple::Id {
    let cls = apple::get_class(CLASS_NAME);
    let obj: apple::Id = unsafe { msg_send![cls, alloc] };
    unsafe { msg_send![obj, initWithFrame: frame] }
}
