use super::super::super::core::application::ApplicationTrait;
use super::super::super::objc::runtime::{Object, Sel};
use super::super::super::render::engine::RenderEngine;
use super::super::apple;
use std::mem::transmute;
use std::os::raw::c_void;

pub const CLASS_NAME: &str = "GameViewController";
pub const SUPER_CLASS_NAME: &str = "NSViewController";
pub const DISPLAY_LINK_VAR_NAME: &str = "display_link";
pub const CORE_APP_VAR_NAME: &str = "core_app";

extern "C" fn display_link_callback(
    _display_link: apple::core_video::CVDisplayLinkRef,
    _in_now: *const apple::core_video::CVTimeStamp,
    _in_output_time: *const apple::core_video::CVTimeStamp,
    _flags_in: apple::core_video::CVOptionFlags,
    _flags_out: *mut apple::core_video::CVOptionFlags,
    _display_link_context: *mut libc::c_void,
) -> apple::core_video::CVReturn {
    vxtodo!(); // render frame in here
    apple::core_video::KCVReturnSuccess
}

//- (void)gameViewDidLoad
extern "C" fn game_view_did_load(this: &mut Object, _cmd: Sel) {
    let _: () = unsafe { msg_send![this, viewDidLoad] };
    let view: apple::Id = unsafe { msg_send![this, view] };
    let _: () = unsafe { msg_send![this, setWantsLayer: YES] };
    vxtodo!(); // initialize sys app in here
    let mut display_link = apple::core_video::CVDisplayLinkRef;
    unsafe {
        apple::core_video::CVDisplayLinkCreateWithActiveCGDisplays(&mut display_link);
        vxtodo!(); // pass sys app persistant pointer in here
        apple::core_video::CVDisplayLinkSetOutputCallback(display_link, &display_link_callback, 0);
        apple::core_video::CVDisplayLinkStart(display_link);
    }
}

// -(void) dealloc
extern "C" fn dealloc(this: &mut Object, _cmd: Sel) {
    apple::core_video::CVDisplayLinkRelease(display_link);
    let bounds: apple::NSRect = unsafe { msg_send![view, bounds] };
    let app: *mut App<CoreApp> =
        unsafe { transmute(*(*this).get_ivar::<apple::NSUInteger>(APP_VAR_NAME)) };
    unsafe {
        (*(*app).render_engine).draw_rect_resized(&bounds.size);
    }
}

//-(void) keyDown:(NSEvent*) theEvent
extern "C" fn key_down(this: &mut Object, _cmd: Sel, _event: apple::Id) {
    let release_pool = apple::NsAutoReleasePool::new();
    let app: &mut App<CoreApp> =
        unsafe { transmute(*this.get_ivar::<apple::NSUInteger>(APP_VAR_NAME)) };
    let renderer: &mut RenderEngine<CoreApp> = unsafe { transmute(app.render_engine) };
    renderer.render();
    let _ = release_pool;
}

// Methods to get and set state of the our ultimate render destination (i.e. the drawable)
// # pragma mark RenderDestinationProvider implementation
// - (MTLRenderPassDescriptor*) currentRenderPassDescriptor
extern "C" fn get_current_render_pass_descriptor(this: &mut Object, _cmd: Sel) -> apple::Id {
    let view: apple::Id = unsafe { msg_send![this, view] };
    let desc: apple::Id = unsafe { msg_send![view, currentRenderPassDescriptor] };
    return desc;
}

// - (MTLPixelFormat) colorPixelFormat
extern "C" fn get_color_pixel_format(this: &mut Object, _cmd: Sel) -> apple::NSUInteger {
    let view: apple::Id = unsafe { msg_send![this, view] };
    unsafe { msg_send![view, colorPixelFormat] }
}

// - (void) setColorPixelFormat: (MTLPixelFormat) pixelFormat
extern "C" fn set_color_pixel_format(this: &mut Object, _cmd: Sel, frmt: apple::NSUInteger) {
    let view: apple::Id = unsafe { msg_send![this, view] };
    unsafe {
        let _: () = msg_send![view, setColorPixelFormat: frmt];
    }
}

// - (MTLPixelFormat) depthStencilPixelFormat
extern "C" fn get_depth_stencil_pixel_format(this: &mut Object, _cmd: Sel) -> apple::NSUInteger {
    let view: apple::Id = unsafe { msg_send![this, view] };
    unsafe { msg_send![view, depthStencilPixelFormat] }
}

// - (void) setDepthStencilPixelFormat: (MTLPixelFormat) pixelFormat
extern "C" fn set_depth_stencil_pixel_format(
    this: &mut Object,
    _cmd: Sel,
    frmt: apple::NSUInteger,
) {
    let view: apple::Id = unsafe { msg_send![this, view] };
    unsafe {
        let _: () = msg_send![view, setDepthStencilPixelFormat: frmt];
    }
}

// - (NSUInteger) sampleCount
extern "C" fn get_sample_count(this: &mut Object, _cmd: Sel) -> apple::NSUInteger {
    let view: apple::Id = unsafe { msg_send![this, view] };
    return unsafe { msg_send![view, sampleCount] };
}

// - (void) setSampleCount:(NSUInteger) sampleCount
extern "C" fn set_sample_count(this: &mut Object, _cmd: Sel, sample_count: apple::NSUInteger) {
    let view: apple::Id = unsafe { msg_send![this, view] };
    unsafe {
        let _: () = msg_send![view, setSampleCount: sample_count];
    }
}

// - (id<MTLDrawable>) currentDrawable
extern "C" fn get_current_drawable(this: &mut Object, _cmd: Sel) -> apple::Id {
    let view: apple::Id = unsafe { msg_send![this, view] };
    return unsafe { msg_send![view, currentDrawable] };
}

pub fn register<CoreApp>()
where
    CoreApp: ApplicationTrait,
{
    let mut self_class = apple::dec_class_s(CLASS_NAME, SUPER_CLASS_NAME);
    self_class.add_ivar::<c_void>(DISPLAY_LINK_VAR_NAME);
    self_class.add_ivar::<c_void>(CORE_APP_VAR_NAME);
    self_class.add_ivar::<apple::Id>(DEVICE_VAR_NAME);
    self_class.add_ivar::<apple::NSUInteger>(APP_VAR_NAME);

    unsafe {
        self_class.add_method(sel!(gameViewDidLoad), game_view_did_load);
        self_class.add_method(
            sel!(mtkView:drawableSizeWillChange:),
            mtl_view::<CoreApp> as extern "C" fn(&mut Object, Sel, apple::Id, apple::NSSize),
        );
        self_class.add_method(
            sel!(drawInMTKView:),
            draw_in_mtk_view::<CoreApp> as extern "C" fn(&mut Object, Sel, apple::Id),
        );
        self_class.add_method(
            sel!(currentRenderPassDescriptor),
            get_current_render_pass_descriptor as extern "C" fn(&mut Object, Sel) -> apple::Id,
        );
        self_class.add_method(
            sel!(colorPixelFormat),
            get_color_pixel_format as extern "C" fn(&mut Object, Sel) -> apple::NSUInteger,
        );
        self_class.add_method(
            sel!(setColorPixelFormat:),
            set_color_pixel_format as extern "C" fn(&mut Object, Sel, apple::NSUInteger),
        );
        self_class.add_method(
            sel!(depthStencilPixelFormat),
            get_depth_stencil_pixel_format as extern "C" fn(&mut Object, Sel) -> apple::NSUInteger,
        );
        self_class.add_method(
            sel!(setDepthStencilPixelFormat:),
            set_depth_stencil_pixel_format as extern "C" fn(&mut Object, Sel, apple::NSUInteger),
        );
        self_class.add_method(
            sel!(sampleCount),
            get_sample_count as extern "C" fn(&mut Object, Sel) -> apple::NSUInteger,
        );
        self_class.add_method(
            sel!(setSampleCount:),
            set_sample_count as extern "C" fn(&mut Object, Sel, apple::NSUInteger),
        );
        self_class.add_method(
            sel!(currentDrawable),
            get_current_drawable as extern "C" fn(&mut Object, Sel) -> apple::Id,
        );
    }
    self_class.register();
}
