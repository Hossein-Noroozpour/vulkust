extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate gdi32;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::render::engine::RenderEngine;
use super::super::super::util::string::string_to_cwstring;
use super::super::os::{OsApplication, ApplicationTrait as OsApplicationTrait};

use std::ptr::{null, null_mut};
use std::mem::{zeroed, size_of, transmute};
use std::os::raw::c_void;

pub struct Application<CoreApp>
    where CoreApp: ApplicationTrait
{
    pub h_instance: winapi::minwindef::HINSTANCE,
    pub h_window: winapi::windef::HWND,
    pub core_app: *mut CoreApp,
    pub render_engine: *mut RenderEngine<CoreApp>,
}

unsafe extern "system" fn process_callback(
        hwnd: winapi::windef::HWND,
        msg: winapi::minwindef::UINT,
        wParam: winapi::minwindef::WPARAM,
        lParam: winapi::minwindef::LPARAM) -> winapi::minwindef::LRESULT {
    0
}

impl<CoreApp> OsApplicationTrait<CoreApp> for Application<CoreApp>
    where CoreApp: ApplicationTrait
{
    fn new() -> Self {
        let application_name = string_to_cwstring("Gearoenix Nu-Frag Game Engine");
        let mut this = Application {
            h_instance: unsafe { kernel32::GetModuleHandleA(null()) },
            h_window: null_mut(),
            core_app: null_mut(),
            render_engine: null_mut(),
        };
        let mut wnd_class: winapi::winuser::WNDCLASSEXW = unsafe { zeroed() };
        wnd_class.cbSize = size_of::<winapi::winuser::WNDCLASSEXW>() as u32;
        wnd_class.style = winapi::winuser::CS_HREDRAW | winapi::winuser::CS_VREDRAW;
        wnd_class.lpfnWndProc = Some(process_callback);
        wnd_class.hInstance = this.h_instance;
        wnd_class.hIcon = unsafe { user32::LoadIconW(
            0 as winapi::minwindef::HINSTANCE, winapi::winuser::IDI_APPLICATION) };
        wnd_class.hCursor = unsafe { user32::LoadCursorW(
            0 as winapi::minwindef::HINSTANCE, winapi::winuser::IDC_ARROW) };
        wnd_class.hbrBackground =
            unsafe { gdi32::GetStockObject(winapi::wingdi::BLACK_BRUSH) }
            as winapi::windef::HBRUSH;
        wnd_class.lpszClassName = application_name.as_ptr();
        wnd_class.hIconSm = unsafe { user32::LoadIconW(
            0 as winapi::minwindef::HINSTANCE, winapi::winuser::IDI_WINLOGO) };
        if unsafe { user32::RegisterClassExW(&wnd_class) } == 0 as _ {
            logf!("Could not register window class!");
        }
        let screen_width = unsafe { user32::GetSystemMetrics(winapi::winuser::SM_CXSCREEN) };
        let screen_height = unsafe { user32::GetSystemMetrics(winapi::winuser::SM_CYSCREEN) };
        let mut dwex_style = 0 as winapi::minwindef::DWORD;
        let mut dw_style = 0;
        let mut window_rect: winapi::windef::RECT = unsafe { zeroed() };
        #[cfg(feature = "fullscreen")]
        {
            let mut dm_screen_settings: winapi::wingdi::DEVMODEW = unsafe { zeroed() };
            dm_screen_settings.dmSize = size_of::<winapi::wingdi::DEVMODEW>();
            dm_screen_settings.dmPelsWidth = screen_width;
            dm_screen_settings.dmPelsHeight = screen_height;
            dm_screen_settings.dmBitsPerPel = 32;
            dm_screen_settings.dmFields =
                winapi::wingdi::DM_BITSPERPEL |
                winapi::wingdi::DM_PELSWIDTH |
                winapi::wingdi::DM_PELSHEIGHT;
            if screen_width != default_window_width!() &&
               screen_height != default_window_height!() {
                if user32::ChangeDisplaySettingsW(
                    &dm_screen_settings, winapi::winuser::CDS_FULLSCREEN) !=
                        winapi::winuser::DISP_CHANGE_SUCCESSFUL {
                    logf!("Fullscreen Mode not supported!");
                }
            }
            dwex_style = winapi::winuser::WS_EX_APPWINDOW;
            dw_style =
                winapi::winuser::WS_POPUP |
                winapi::winuser::WS_CLIPSIBLINGS |
                winapi::winuser::WS_CLIPCHILDREN;
            window_rect.right = screen_width;
            window_rect.bottom = screen_height;
        }
        #[cfg(not(feature = "fullscreen"))]
        {
            dwex_style = winapi::winuser::WS_EX_APPWINDOW | winapi::winuser::WS_EX_WINDOWEDGE;
            dw_style = winapi::winuser::WS_OVERLAPPEDWINDOW | winapi::winuser::WS_CLIPSIBLINGS |
                       winapi::winuser::WS_CLIPCHILDREN;
            window_rect.right = default_window_width!();
            window_rect.bottom = default_window_height!();
        }
        unsafe { user32::AdjustWindowRectEx(
            &mut window_rect, dw_style, winapi::minwindef::FALSE, dwex_style); }
        this.h_window = unsafe { user32::CreateWindowExW(
            0, application_name.as_ptr(), application_name.as_ptr(),
            dw_style | winapi::winuser::WS_CLIPSIBLINGS | winapi::winuser::WS_CLIPCHILDREN, 0, 0,
            window_rect.right - window_rect.left, window_rect.bottom - window_rect.top, null_mut(),
            null_mut(), this.h_instance, transmute(&mut this)) };
        #[cfg(feature = "fullscreen")]
        {
            let x = (user32::GetSystemMetrics(winapi::winuser::SM_CXSCREEN) -
                window_rect.right) / 2;
            let y = (user32::GetSystemMetrics(winapi::winuser::SM_CYSCREEN) -
                window_rect.bottom) / 2;
            user32::SetWindowPos(
                window, 0, x, y, 0, 0,
                winapi::winuser::SWP_NOZORDER | winapi::winuser::SWP_NOSIZE);
        }
        if this.h_window == null_mut() {
            logf!("Could not create window!");
        }
        unsafe { user32::ShowWindow(this.h_window, winapi::winuser::SW_SHOW); }
        unsafe { user32::SetForegroundWindow(this.h_window); }
        unsafe { user32::SetFocus(this.h_window); }
        unsafe { user32::UpdateWindow(this.h_window); }
        this
    }
    fn start(&mut self) -> bool {
        return true;
    }
    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }
    fn set_rnd_eng(&mut self, r: *mut RenderEngine<CoreApp>) {
        self.render_engine = r;
    }
    fn execute() -> bool {
        return true;
    }
}
