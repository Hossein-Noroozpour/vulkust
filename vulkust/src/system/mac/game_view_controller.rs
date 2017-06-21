use std::mem::transmute;
use super::super::super::objc::runtime::{Object, Sel};
use super::super::super::render::engine::{RenderEngine, EngineTrait};
use super::super::super::core::application::ApplicationTrait;
use super::super::metal as mtl;
use super::application::Application as App;

pub const DEVICE_VAR_NAME: &str = "mtl_device";
pub const RENDERER_VAR_NAME: &str = "mtl_renderer";
pub const APP_VAR_NAME: &str = "vukust_os_app";
pub const CLASS_NAME: &str = "GameViewController";
pub const SUPER_CLASS_NAME: &str = "NSViewController";

//- (void)metalViewDidLoad
extern fn metal_view_did_load(_this: &mut Object, _cmd: Sel) {
    // _view.delegate = self;
    // _view.device = _device;
    // _renderer = [[Renderer alloc] initWithMetalDevice:_device renderDestinationProvider:self];
    // [_renderer drawRectResized:_view.bounds.size];
}

// Called whenever view changes orientation or layout is changed
// - (void)mtkView:(nonnull MTKView *)view drawableSizeWillChange:(CGSize)size
extern fn mtl_view(this: &mut Object, _cmd: Sel, view: mtl::Id, _size: mtl::NSSize) {
    let bounds: mtl::NSRect = unsafe { msg_send![view, bounds] };
    let renderer: &mtl::Id = unsafe { this.get_ivar(RENDERER_VAR_NAME) };
    unsafe { let _: () = msg_send![*renderer, drawRectResized:bounds.size]; }
}

// Called whenever the view needs to render
// - (void)drawInMTKView:(nonnull MTKView *)view
extern fn draw_in_mtk_view<CoreApp>(this: &mut Object, _cmd: Sel, _view: mtl::Id)
        where CoreApp: ApplicationTrait {
    let app: &App<CoreApp> = unsafe { transmute(this.get_ivar::<mtl::NSUInteger>(APP_VAR_NAME)) };
    let renderer: &mut RenderEngine<CoreApp> = unsafe { transmute(app.render_engine) };
    renderer.update();
}

// Methods to get and set state of the our ultimate render destination (i.e. the drawable)
// # pragma mark RenderDestinationProvider implementation
// - (MTLRenderPassDescriptor*) currentRenderPassDescriptor
extern fn get_current_render_pass_descriptor(this: &mut Object, _cmd: Sel) -> mtl::Id {
    let view: mtl::Id = unsafe { msg_send![this, view] };
    let desc: mtl::Id = unsafe { msg_send![view, currentRenderPassDescriptor] };
    return desc;
}

// - (MTLPixelFormat) colorPixelFormat
extern fn get_color_pixel_format(this: &mut Object, _cmd: Sel) -> mtl::NSUInteger {
    let view: mtl::Id = unsafe { msg_send![this, view] };
    unsafe { msg_send![view, colorPixelFormat] }
}

// - (void) setColorPixelFormat: (MTLPixelFormat) pixelFormat
extern fn set_color_pixel_format(this: &mut Object, _cmd: Sel, frmt: mtl::NSUInteger) {
    let view: mtl::Id = unsafe { msg_send![this, view] };
    unsafe { let _: () = msg_send![view, setColorPixelFormat:frmt]; }
}

// - (MTLPixelFormat) depthStencilPixelFormat
extern fn get_depth_stencil_pixel_format(this: &mut Object, _cmd: Sel) -> mtl::NSUInteger {
    let view: mtl::Id = unsafe { msg_send![this, view] };
    unsafe { msg_send![view, depthStencilPixelFormat] }
}

// - (void) setDepthStencilPixelFormat: (MTLPixelFormat) pixelFormat
extern fn set_depth_stencil_pixel_format(this: &mut Object, _cmd: Sel, frmt: mtl::NSUInteger) {
    let view: mtl::Id = unsafe { msg_send![this, view] };
    unsafe { let _: () = msg_send![view, setDepthStencilPixelFormat:frmt]; }
}

// - (NSUInteger) sampleCount
extern fn get_sample_count(this: &mut Object, _cmd: Sel) -> mtl::NSUInteger {
    let view: mtl::Id = unsafe { msg_send![this, view] };
    return unsafe { msg_send![view, sampleCount] };
}

// - (void) setSampleCount:(NSUInteger) sampleCount
extern fn set_sample_count(this: &mut Object, _cmd: Sel, sample_count: mtl::NSUInteger) {
    let view: mtl::Id = unsafe { msg_send![this, view] };
    unsafe { let _: () = msg_send![view, setSampleCount:sample_count]; }
}

// - (id<MTLDrawable>) currentDrawable
extern fn get_current_drawable(this: &mut Object, _cmd: Sel) -> mtl::Id {
    let view: mtl::Id = unsafe { msg_send![this, view] };
    return unsafe { msg_send![view, currentDrawable] };
}

pub fn register<CoreApp>() where CoreApp: ApplicationTrait {
    let mut self_class = mtl::dec_class_s(CLASS_NAME, SUPER_CLASS_NAME);
    self_class.add_ivar::<mtl::Id>(DEVICE_VAR_NAME);
    self_class.add_ivar::<mtl::Id>(RENDERER_VAR_NAME);
    self_class.add_ivar::<mtl::NSUInteger>(APP_VAR_NAME);

    unsafe {
        self_class.add_method(
            sel!(metalViewDidLoad),
            metal_view_did_load as extern fn(&mut Object, Sel));
        self_class.add_method(
            sel!(mtkView:v:),
            mtl_view as extern fn(&mut Object, Sel, mtl::Id, mtl::NSSize));
        self_class.add_method(
            sel!(drawInMTKView:),
            draw_in_mtk_view::<CoreApp> as extern fn(&mut Object, Sel, mtl::Id));
        self_class.add_method(
            sel!(currentRenderPassDescriptor),
            get_current_render_pass_descriptor as extern fn(&mut Object, Sel) -> mtl::Id);
        self_class.add_method(
            sel!(colorPixelFormat),
            get_color_pixel_format as extern fn(&mut Object, Sel) -> mtl::NSUInteger);
        self_class.add_method(
            sel!(setColorPixelFormat:),
            set_color_pixel_format as extern fn(&mut Object, Sel, mtl::NSUInteger));
        self_class.add_method(
            sel!(depthStencilPixelFormat),
            get_depth_stencil_pixel_format as extern fn(&mut Object, Sel) -> mtl::NSUInteger);
        self_class.add_method(
            sel!(setDepthStencilPixelFormat:),
            set_depth_stencil_pixel_format as extern fn(&mut Object, Sel, mtl::NSUInteger));
        self_class.add_method(
            sel!(sampleCount),
            get_sample_count as extern fn(&mut Object, Sel) -> mtl::NSUInteger);
        self_class.add_method(
            sel!(setSampleCount:),
            set_sample_count as extern fn(&mut Object, Sel, mtl::NSUInteger));
        self_class.add_method(
            sel!(currentDrawable),
            get_current_drawable as extern fn(&mut Object, Sel) -> mtl::Id);
    }
    self_class.register();
}
