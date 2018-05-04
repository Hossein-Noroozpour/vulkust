use std::mem::transmute;
use super::super::super::objc::runtime::{BOOL, Class, Object, Sel, YES};
use super::super::apple;

pub const CLASS_NAME: &str = "GameView";
pub const SUPER_CLASS_NAME: &str = "NSView";

//- (BOOL) wantsUpdateLayer
extern "C" fn wants_update_layer(this: &mut Object, _cmd: Sel) -> BOOL {
    YES
}

//+ (Class) layerClass
extern "C" fn layer_class(cls: &mut Class, _cmd: Sel) -> Class {
    unsafe {
        msg_send![apple::get_class("CAMetalLayer"), class]
    }
}

// -(CALayer*) makeBackingLayer
extern "C" fn make_backing_layer(this: &mut Object, _cmd: Sel) -> apple::Id {
    let layer: apple::Id = unsafe { msg_send![apple::get_class("CAMetalLayer"), layer] };
    let size = apple::core_graphics::make_size(1.0, 1.0);
    let view_scale: apple::core_graphics::CGSize = unsafe { 
        msg_send![this, convertSizeToBacking:size] 
    };
    let contents_scale = min(view_scale.width, view_scale.height);
    let _: () = unsafe { msg_send![layer, setContentsScale:contents_scale] };
    return layer;
}

// -(BOOL) acceptsFirstResponder { return YES; }
extern "C" fn accepts_first_responder(this: &mut Object, _cmd: Sel) -> BOOL {
    YES
}

pub fn register() {
    let mut self_class = mtl::dec_class_s(CLASS_NAME, SUPER_CLASS_NAME);
    unsafe {
        self_class.add_method(sel!(wantsUpdateLayer), wants_update_layer);
        self_class.add_class_method(sel!(layerClass), layer_class);
        self_class.add_method(sel!(makeBackingLayer), make_backing_layer);
        self_class.add_method(sel!(acceptsFirstResponder), accepts_first_responder);
    }
    self_class.register();
}
