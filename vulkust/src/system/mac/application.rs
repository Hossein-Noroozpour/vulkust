use std::ptr::null_mut;
use std::os::raw::c_void;
use super::super::super::objc;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::render::engine::RenderEngine;
use super::super::os::ApplicationTrait as OsApplicationTrait;
use super::types::*;
use super::foundation::*;

pub struct Application<CoreApp> where CoreApp: ApplicationTrait  {
    pub core_app: *mut CoreApp,
    pub render_engine: *mut RenderEngine<CoreApp>,
}

impl<CoreApp> OsApplicationTrait<CoreApp> for Application<CoreApp>
    where CoreApp: ApplicationTrait {
    fn new(_: *const c_void) -> Self {
        Application {
            core_app: null_mut(),
            render_engine: null_mut(),
        }
    }
    fn initialize(&mut self) -> bool {
        // Autorelease Pool:
        // Objects declared in this scope will be automatically
        // released at the end of it, when the pool is "drained".
        let pool = get_class!("NSAutoreleasePool");
        let pool: Id = unsafe { msg_send![pool, alloc] };
        let pool: Id = unsafe { msg_send![pool, init] };
        // Create a shared app instance.
        // This will initialize the global variable
        // 'NSApp' with the application instance.
        let ns_application = get_class!("NSApplication");
        let ns_application: Id = unsafe { msg_send![ns_application, sharedApplication] };
        // Create a window:
        // Style flags:
        let window_style =
            (
                NS_TITLED_WINDOW_MASK |
                NS_CLOSABLE_WINDOW_MASK |
                NS_RESIZABLE_WINDOW_MASK
            ).bits() as NSUInteger;
        // Window bounds (x, y, width, height).
        let window_rect = ns_make_rect(100., 100., 400., 400.);
        let window = get_class!("NSWindow");
        let window: Id = unsafe { msg_send![window, alloc] };
        let window: Id = unsafe { msg_send![window,
            initWithContentRect:window_rect
            styleMask:window_style
            backing:NS_BACKING_STORE_BUFFERED
            defer:objc::runtime::NO] };
        unsafe { msg_send![window, autorelease] };
        // Window controller:
        let window_controller = get_class!("NSWindowController");
        let window_controller: Id = unsafe { msg_send![window_controller, alloc] };
        let window_controller: Id = unsafe { msg_send![window_controller, initWithWindow:window] };
        unsafe { msg_send![window_controller, autorelease] };
        // Text
        let text = ns_string_new_with_pool("Hello I'm Hossein.");
        // This will add a simple text view to the window,
        // so we can write a test string on it.
        let text_view = get_class!("NSTextView");
        let text_view: Id = unsafe { msg_send![text_view, alloc] };
        let text_view: Id = unsafe { msg_send![text_view, initWithFrame:window_rect] };
        unsafe { msg_send![text_view, autorelease] };
        unsafe { msg_send![window, setContentView:text_view] };
        unsafe { msg_send![text_view, insertText:text] };
        // TODO: Create app delegate to handle system events.
        // TODO: Create menus (especially Quit!)
        // Show window and run event loop.
        unsafe { msg_send![window, orderFrontRegardless] };
        unsafe { msg_send![ns_application, run] };
        unsafe { msg_send![pool, drain]; }
        true
    }
    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }
    fn set_rnd_eng(&mut self, r: *mut RenderEngine<CoreApp>) {
        self.render_engine = r;
    }
    fn execute(&mut self) -> bool {
        true
    }
}
