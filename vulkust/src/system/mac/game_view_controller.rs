use super::super::super::objc;
use super::super::super::objc::runtime::{Object, Sel, YES, BOOL};
use super::super::metal as mtl;

pub const DEVICE_VAR_NAME: &str = "mtl_device";
pub const RENDERER_VAR_NAME: &str = "mtl_renderer";
pub const CLASS_NAME: &str = "GameViewController";
pub const SUPER_CLASS_NAME: &str = "NSViewController";


//- (void)metalViewDidLoad
extern fn metal_view_did_load(_this: &mut Object, _cmd: Sel) {
    // _view.delegate = self;
    // _view.device = _device;
    // _renderer = [[Renderer alloc] initWithMetalDevice:_device renderDestinationProvider:self];
    // [_renderer drawRectResized:_view.bounds.size];
}

// // Called whenever view changes orientation or layout is changed
// - (void)mtkView:(nonnull MTKView *)view drawableSizeWillChange:(CGSize)size
// {
//     [_renderer drawRectResized:view.bounds.size];
// }
//
// // Called whenever the view needs to render
// - (void)drawInMTKView:(nonnull MTKView *)view
// {
//     @autoreleasepool
//     {
//         [_renderer update];
//     }
// }
//
// // Methods to get and set state of the our ultimate render destination (i.e. the drawable)
// # pragma mark RenderDestinationProvider implementation
//
// - (MTLRenderPassDescriptor*) currentRenderPassDescriptor
// {
//     return _view.currentRenderPassDescriptor;
// }
//
// - (MTLPixelFormat) colorPixelFormat
// {
//     return _view.colorPixelFormat;
// }
//
// - (void) setColorPixelFormat: (MTLPixelFormat) pixelFormat
// {
//     _view.colorPixelFormat = pixelFormat;
// }
//
// - (MTLPixelFormat) depthStencilPixelFormat
// {
//     return _view.depthStencilPixelFormat;
// }
//
// - (void) setDepthStencilPixelFormat: (MTLPixelFormat) pixelFormat
// {
//     _view.depthStencilPixelFormat = pixelFormat;
// }
//
// - (NSUInteger) sampleCount
// {
//     return _view.sampleCount;
// }
//
// - (void) setSampleCount:(NSUInteger) sampleCount
// {
//     _view.sampleCount = sampleCount;
// }
//
// - (id<MTLDrawable>) currentDrawable
// {
//     return _view.currentDrawable;
// }
//
// @end



pub fn register() {
    let super_class = get_class!(SUPER_CLASS_NAME);
    let mut self_class = dec_class!(CLASS_NAME, super_class);
    self_class.add_ivar::<mtl::Id>(DEVICE_VAR_NAME);
    self_class.add_ivar::<mtl::Id>(RENDERER_VAR_NAME);
    unsafe {
        self_class.add_method(
            sel!(metalViewDidLoad),
            metal_view_did_load as extern fn(&mut Object, Sel));
        // self_class.add_method(
        //     sel!(applicationWillFinishLaunching:),
        //     application_will_finish_launching as extern fn(&Object, Sel, Id));
        // self_class.add_method(
        //     sel!(applicationDidFinishLaunching:),
        //     application_did_finish_launching as extern fn(&Object, Sel, Id));
        // self_class.add_method(
        //     sel!(applicationWillTerminate:),
        //     application_will_terminate as extern fn(&Object, Sel, Id));
        // self_class.add_method(
        //     sel!(applicationShouldTerminateAfterLastWindowClosed:),
        //     application_should_terminate_after_last_window_closed
        //     as extern fn(&Object, Sel, Id) -> BOOL);
    }
    self_class.register();
}
