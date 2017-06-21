use std::ptr::null_mut;
use std::os::raw::c_void;
use std::mem::transmute;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::core::asset::manager::Manager as AssetManager;
// use super::super::super::io::read::Read;
use super::super::super::render::engine::RenderEngine;
use super::super::os::ApplicationTrait as OsApplicationTrait;
use super::super::file::File;
use super::super::metal as mtl;
use super::app_delegate;
use super::game_view_controller;
// use super::foundation::*;

pub struct Application<CoreApp> where CoreApp: ApplicationTrait  {
    pub core_app: *mut CoreApp,
    pub render_engine: *mut RenderEngine<CoreApp>,
    pub ns_application: mtl::Id,
    pub app_delegate: mtl::Id,
    pub game_view_controller: mtl::Id,
    pub metal_device: mtl::Id,
    pub metal_view: mtl::Id,
    pub asset_manager: AssetManager,
    pub ns_autorelease_pool: mtl::NsAutoReleasePool, // TODO: remove it I don't like it
}

impl<CoreApp> OsApplicationTrait<CoreApp> for Application<CoreApp>
    where CoreApp: ApplicationTrait {
    fn new(_: *const c_void) -> Self {
        let asset_file_name = "data.gx3d".to_string();
        Application {
            core_app: null_mut(),
            render_engine: null_mut(),
            ns_application: null_mut(),
            app_delegate: null_mut(),
            game_view_controller: null_mut(),
            metal_device: null_mut(),
            metal_view: null_mut(),
            asset_manager: AssetManager::new(File::new(&asset_file_name)),
            ns_autorelease_pool: mtl::NsAutoReleasePool::new(),
        }
    }
    fn initialize(&mut self) -> bool {
        self.asset_manager.initialize();
        let app: mtl::NSUInteger = unsafe { transmute((*self.render_engine).os_app) };
        app_delegate::register::<CoreApp>();
        game_view_controller::register::<CoreApp>();
        let ns_application = mtl::get_class("NSApplication");
        let ns_application: mtl::Id = unsafe { msg_send![ns_application, sharedApplication] };
        self.ns_application = ns_application;
        self.app_delegate = mtl::get_instance(app_delegate::CLASS_NAME);
        unsafe { (*self.app_delegate).set_ivar(app_delegate::APP_VAR_NAME, app); }
        unsafe { let _: () = msg_send![self.app_delegate, initialize];}
        unsafe { let _: () = msg_send![ns_application, setDelegate:self.app_delegate]; }
        logi!("Reached.");




        // // Create a window:
        // // Style flags:
        // let window_style =
        //     (
        //         NS_TITLED_WINDOW_MASK |
        //         NS_CLOSABLE_WINDOW_MASK |
        //         NS_RESIZABLE_WINDOW_MASK
        //     ).bits() as NSUInteger;
        // // Window bounds (x, y, width, height).
        // let window_rect = ns_make_rect(100., 100., 400., 400.);
        // let window = get_class!("NSWindow");
        // let window: Id = unsafe { msg_send![window, alloc] };
        // let window: Id = unsafe { msg_send![window,
        //     initWithContentRect:window_rect
        //     styleMask:window_style
        //     backing:NS_BACKING_STORE_BUFFERED
        //     defer:objc::runtime::NO] };
        // unsafe { msg_send![window, autorelease] };
        // // Window controller:
        // let window_controller = get_class!("NSWindowController");
        // let window_controller: Id = unsafe { msg_send![window_controller, alloc] };
        // let window_controller: Id = unsafe { msg_send![window_controller, initWithWindow:window] };
        // unsafe { msg_send![window_controller, autorelease] };
        // // Text
        // let text = ns_string_new_with_pool("Hello I'm Hossein.");
        // // This will add a simple text view to the window,
        // // so we can write a test string on it.
        // let text_view = get_class!("NSTextView");
        // let text_view: Id = unsafe { msg_send![text_view, alloc] };
        // let text_view: Id = unsafe { msg_send![text_view, initWithFrame:window_rect] };
        // unsafe { msg_send![text_view, autorelease] };
        // unsafe { msg_send![window, setContentView:text_view] };
        // unsafe { msg_send![text_view, insertText:text] };
        // // TODO: Create app delegate to handle system events.
        // // TODO: Create menus (especially Quit!)
        // // Show window and run event loop.
        // unsafe { msg_send![window, orderFrontRegardless] };
        true
    }
    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }
    fn set_rnd_eng(&mut self, r: *mut RenderEngine<CoreApp>) {
        self.render_engine = r;
    }
    fn execute(&mut self) -> bool {
        unsafe { let _: () = msg_send![self.ns_application, run]; }
        true
    }
}
