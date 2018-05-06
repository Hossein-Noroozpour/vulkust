use super::super::super::objc::runtime::{Class, Object, Sel, BOOL, YES};
use super::super::apple;

pub const CLASS_NAME: &str = "GameView";
pub const SUPER_CLASS_NAME: &str = "NSView";

//- (BOOL) wantsUpdateLayer
extern "C" fn wants_update_layer(this: &mut Object, _cmd: Sel) -> BOOL {
    YES
}

//+ (Class) layerClass
extern "C" fn layer_class(_cls: &mut Class, _cmd: Sel) -> &'static Class {
    unsafe { msg_send![apple::get_class("CAMetalLayer"), class] }
}

// -(CALayer*) makeBackingLayer
extern "C" fn make_backing_layer(this: &mut Object, _cmd: Sel) -> apple::Id {
    let layer: apple::Id = unsafe { msg_send![apple::get_class("CAMetalLayer"), layer] };
    let size = apple::core_graphics::make_size(1.0, 1.0);
    let view_scale: apple::core_graphics::CGSize =
        unsafe { msg_send![this, convertSizeToBacking: size] };
    let contents_scale = if view_scale.width < view_scale.height {
        view_scale.width
    } else {
        view_scale.height
    };
    let _: () = unsafe { msg_send![layer, setContentsScale: contents_scale] };
    return layer;
}

// -(BOOL) acceptsFirstResponder { return YES; }
extern "C" fn accepts_first_responder(this: &mut Object, _cmd: Sel) -> BOOL {
    YES
}

pub fn register() {
    let mut self_class = apple::dec_class_s(CLASS_NAME, SUPER_CLASS_NAME);
    unsafe {
        self_class.add_method(
            sel!(wantsUpdateLayer),
            wants_update_layer as extern "C" fn(&mut Object, Sel) -> BOOL,
        );
        self_class.add_class_method(
            sel!(layerClass),
            layer_class as extern "C" fn(&mut Class, Sel) -> &'static Class,
        );
        self_class.add_method(
            sel!(makeBackingLayer),
            make_backing_layer as extern "C" fn(&mut Object, Sel) -> apple::Id,
        );
        self_class.add_method(
            sel!(acceptsFirstResponder),
            accepts_first_responder as extern "C" fn(&mut Object, Sel) -> BOOL,
        );
    }
    self_class.register();
}

pub fn create_instance(frame: apple::NSRect) -> apple::Id {
    let cls = apple::get_class(CLASS_NAME);
    unsafe { msg_send![cls, initWithFrame: frame] }
}
