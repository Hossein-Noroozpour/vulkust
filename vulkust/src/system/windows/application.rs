extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate gdi32;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::core::asset::manager::Manager as AssetManager;
use super::super::super::render::engine::{EngineTrait, RenderEngine};
use super::super::super::util::string::string_to_cwstring;
use super::super::os::{ApplicationTrait as OsApplicationTrait};
use super::super::file::File;

use std::ptr::{null, null_mut};
use std::mem::{size_of, transmute, zeroed};
use std::os::raw::c_void;

pub struct Application<CoreApp>
where
    CoreApp: 'static + ApplicationTrait,
{
    pub asset_manager: AssetManager,
    pub h_instance: winapi::minwindef::HINSTANCE,
    pub h_window: winapi::windef::HWND,
    pub core_app: &'static mut CoreApp,
    pub render_engine: &'static mut RenderEngine<CoreApp>,
}

unsafe extern "system" fn process_callback<CoreApp>(
    hwnd: winapi::windef::HWND,
    msg: winapi::minwindef::UINT,
    w_param: winapi::minwindef::WPARAM,
    l_param: winapi::minwindef::LPARAM,
) -> winapi::minwindef::LRESULT
where
    CoreApp: 'static + ApplicationTrait,
{
    let mut os_app = user32::GetWindowLongPtrW(hwnd, winapi::winuser::GWLP_USERDATA);
    if winapi::winuser::WM_CREATE == msg {
        let ref create_structure: &mut winapi::winuser::CREATESTRUCTW = transmute(l_param);
        os_app = transmute(create_structure.lpCreateParams);
        user32::SetWindowLongPtrW(hwnd, winapi::winuser::GWLP_USERDATA, os_app);
    }
    if os_app == 0 {
        loge!("Unexpected message for nullptr sys app uMsg is: {}", msg);
        return user32::DefWindowProcW(hwnd, msg, w_param, l_param);
    }
    let ref mut os_app: &mut Application<CoreApp> = transmute(os_app);
    return os_app.handle_message(hwnd, msg, w_param, l_param);
}

impl<CoreApp> OsApplicationTrait<CoreApp> for Application<CoreApp>
where
    CoreApp: 'static + ApplicationTrait,
{
    fn new(_args: *const c_void) -> Self {
        let application_name = string_to_cwstring("Gearoenix Nu-Frag Game Engine");
        let file = File::new(&"data.gx3d".to_string());
        let mut this = Application {
            asset_manager: AssetManager::new(file),
            h_instance: unsafe { kernel32::GetModuleHandleA(null()) },
            h_window: null_mut(),
            core_app: unsafe { transmute(0usize) },
            render_engine: unsafe { transmute(0usize) },
        };
        let mut wnd_class: winapi::winuser::WNDCLASSEXW = unsafe { zeroed() };
        wnd_class.cbSize = size_of::<winapi::winuser::WNDCLASSEXW>() as u32;
        wnd_class.style = winapi::winuser::CS_HREDRAW | winapi::winuser::CS_VREDRAW;
        wnd_class.lpfnWndProc = Some(process_callback::<CoreApp>);
        wnd_class.hInstance = this.h_instance;
        wnd_class.hIcon = unsafe {
            user32::LoadIconW(
                0 as winapi::minwindef::HINSTANCE,
                winapi::winuser::IDI_APPLICATION,
            )
        };
        wnd_class.hCursor = unsafe {
            user32::LoadCursorW(
                0 as winapi::minwindef::HINSTANCE,
                winapi::winuser::IDC_ARROW,
            )
        };
        wnd_class.hbrBackground =
            unsafe { gdi32::GetStockObject(winapi::wingdi::BLACK_BRUSH) } as winapi::windef::HBRUSH;
        wnd_class.lpszClassName = application_name.as_ptr();
        wnd_class.hIconSm = unsafe {
            user32::LoadIconW(
                0 as winapi::minwindef::HINSTANCE,
                winapi::winuser::IDI_WINLOGO,
            )
        };
        if unsafe { user32::RegisterClassExW(&wnd_class) } == 0 as _ {
            logf!("Could not register window class!");
        }
        let mut window_rect: winapi::windef::RECT = unsafe { zeroed() };
        #[cfg(not(feature = "fullscreen"))]
        {
            window_rect.right = 1000;
            window_rect.bottom = 700;
        }
        #[cfg(not(feature = "fullscreen"))]
        let dwex_style = winapi::winuser::WS_EX_APPWINDOW | winapi::winuser::WS_EX_WINDOWEDGE;
        #[cfg(not(feature = "fullscreen"))]
        let dw_style =
            winapi::winuser::WS_OVERLAPPEDWINDOW |
            winapi::winuser::WS_CLIPSIBLINGS |
            winapi::winuser::WS_CLIPCHILDREN;
        #[cfg(feature = "fullscreen")]
        let dwex_style = winapi::winuser::WS_EX_APPWINDOW;
        #[cfg(feature = "fullscreen")]
        let dw_style = winapi::winuser::WS_POPUP | winapi::winuser::WS_CLIPSIBLINGS |
            winapi::winuser::WS_CLIPCHILDREN;
        #[cfg(feature = "fullscreen")]
        {
            let screen_width = unsafe { user32::GetSystemMetrics(winapi::winuser::SM_CXSCREEN) };
            let screen_height = unsafe { user32::GetSystemMetrics(winapi::winuser::SM_CYSCREEN) };
            let mut dm_screen_settings: winapi::wingdi::DEVMODEW = unsafe { zeroed() };
            dm_screen_settings.dmSize = size_of::<winapi::wingdi::DEVMODEW>();
            dm_screen_settings.dmPelsWidth = screen_width;
            dm_screen_settings.dmPelsHeight = screen_height;
            dm_screen_settings.dmBitsPerPel = 32;
            dm_screen_settings.dmFields = winapi::wingdi::DM_BITSPERPEL |
                winapi::wingdi::DM_PELSWIDTH |
                winapi::wingdi::DM_PELSHEIGHT;
            if screen_width != default_window_width!() &&
                screen_height != default_window_height!()
            {
                if user32::ChangeDisplaySettingsW(
                    &dm_screen_settings,
                    winapi::winuser::CDS_FULLSCREEN,
                ) != winapi::winuser::DISP_CHANGE_SUCCESSFUL
                {
                    logf!("Fullscreen Mode not supported!");
                }
            }
            window_rect.right = screen_width;
            window_rect.bottom = screen_height;
        }
        unsafe {
            user32::AdjustWindowRectEx(
                &mut window_rect,
                dw_style,
                winapi::minwindef::FALSE,
                dwex_style,
            );
        }
        this.h_window = unsafe {
            user32::CreateWindowExW(
                0,
                application_name.as_ptr(),
                application_name.as_ptr(),
                dw_style | winapi::winuser::WS_CLIPSIBLINGS | winapi::winuser::WS_CLIPCHILDREN,
                0,
                0,
                window_rect.right - window_rect.left,
                window_rect.bottom - window_rect.top,
                null_mut(),
                null_mut(),
                this.h_instance,
                transmute(&mut this),
            )
        };
        if this.h_window == null_mut() {
            logf!("Could not create window!");
        }
        #[cfg(feature = "fullscreen")]
        {
            let x =
                (user32::GetSystemMetrics(winapi::winuser::SM_CXSCREEN) - window_rect.right) / 2;
            let y =
                (user32::GetSystemMetrics(winapi::winuser::SM_CYSCREEN) - window_rect.bottom) / 2;
            user32::SetWindowPos(
                window,
                0,
                x,
                y,
                0,
                0,
                winapi::winuser::SWP_NOZORDER | winapi::winuser::SWP_NOSIZE,
            );
        }
        unsafe {
            user32::ShowWindow(this.h_window, winapi::winuser::SW_SHOW);
        }
        unsafe {
            user32::SetForegroundWindow(this.h_window);
        }
        unsafe {
            user32::SetFocus(this.h_window);
        }
        unsafe {
            user32::UpdateWindow(this.h_window);
        }
        this
    }

    fn set_core_app(&mut self, c: &'static mut CoreApp) {
        self.core_app = c;
    }

    fn set_rnd_eng(&mut self, r: &'static mut RenderEngine<CoreApp>) {
        self.render_engine = r;
    }

    fn initialize(&mut self) -> bool {
        self.asset_manager.initialize();
        true
    }

    fn execute(&mut self) -> bool {
        loop {
            let mut msg: winapi::winuser::MSG = unsafe { zeroed() };
            while unsafe {
                user32::PeekMessageW(
                    &mut msg,
                    0 as winapi::windef::HWND,
                    0,
                    0,
                    winapi::winuser::PM_REMOVE,
                )
            } != 0
            {
                unsafe {
                    user32::TranslateMessage(&msg);
                    user32::DispatchMessageW(&msg);
                }
                self.render_engine.update();
                if msg.message == winapi::winuser::WM_QUIT {
                    return true;
                }
            }
        }
    }

    fn get_mouse_position(&self) -> (f64, f64) {
        loge!("TODO");
        // TODO
        (0.0, 0.0)
    }

    fn get_window_ratio(&self) -> f64 {
        loge!("TODO");
        // TODO
        1.7
    }
}

impl<CoreApp> Application<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    fn handle_message(
        &mut self,
        hwnd: winapi::windef::HWND,
        msg: winapi::minwindef::UINT,
        w_param: winapi::minwindef::WPARAM,
        l_param: winapi::minwindef::LPARAM,
    ) -> winapi::minwindef::LRESULT {
        match msg {
            winapi::winuser::WM_CLOSE => {
                // TODO: proper termination
                // running = false;
                // DestroyWindow(hWnd);
                // PostQuitMessage(0);
            }
            winapi::winuser::WM_PAINT => unsafe {
                user32::ValidateRect(hwnd, null());
            },
            winapi::winuser::WM_KEYDOWN => {
                let vk_f1 = winapi::winuser::VK_F1 as winapi::minwindef::WPARAM;
                let vk_escape = winapi::winuser::VK_ESCAPE as winapi::minwindef::WPARAM;
                if w_param == 0x50 {
                    // p
                    // TODO pause
                } else if w_param == vk_f1 {
                    /*if (enableTextOverlay) {
        				textOverlay->visible = !textOverlay->visible;
        			}*/
                } else if w_param == vk_escape {
                    unsafe {
                        user32::PostQuitMessage(0);
                    }
                } else {

                }
                /*if (camera.firstperson) {
        			switch (wParam)
        			{
        			case KEY_W:
        				camera.keys.up = true;
        				break;
        			case KEY_S:
        				camera.keys.down = true;
        				break;
        			case KEY_A:
        				camera.keys.left = true;
        				break;
        			case KEY_D:
        				camera.keys.right = true;
        				break;
        			}
        		}*/
            }
            winapi::winuser::WM_KEYUP => {
                //if (camera.firstperson) {
                //	switch (wParam)
                //	{
                //	case 0x57: // W
                //		camera.keys.up = false;
                //		break;
                //	case 0x53: // S
                //		camera.keys.down = false;
                //		break;
                //	case 0x41: // A
                //		camera.keys.left = false;
                //		break;
                //	case 0x44: // D
                //		camera.keys.right = false;
                //		break;
                //	}
                //}
            }
            winapi::winuser::WM_RBUTTONDOWN => {}
            winapi::winuser::WM_LBUTTONDOWN => {}
            winapi::winuser::WM_MBUTTONDOWN => {
                //mousePos.x = (float)LOWORD(lParam);
                //mousePos.y = (float)HIWORD(lParam);
            }
            winapi::winuser::WM_MOUSEWHEEL => {
                //short wheelDelta = GET_WHEEL_DELTA_WPARAM(wParam);
                //zoom += (float)wheelDelta * 0.005f * zoomSpeed;
                //camera.translate(glm::vec3(0.0f, 0.0f, (float)wheelDelta * 0.005f * zoomSpeed));
                //viewUpdated = true;
            }
            winapi::winuser::WM_MOUSEMOVE => {
                if w_param & winapi::winuser::MK_RBUTTON != 0 {
                    //int32_t posx = LOWORD(lParam);
                    //int32_t posy = HIWORD(lParam);
                    //zoom += (mousePos.y - (float)posy) * .005f * zoomSpeed;
                    //camera.translate(glm::vec3(-0.0f, 0.0f, (mousePos.y - (float)posy) * .005f * zoomSpeed));
                    //mousePos = glm::vec2((float)posx, (float)posy);
                    //viewUpdated = true;
                }
                if w_param & winapi::winuser::MK_LBUTTON != 0 {
                    //int32_t posx = LOWORD(lParam);
                    //int32_t posy = HIWORD(lParam);
                    //rotation.x += (mousePos.y - (float)posy) * 1.25f * rotationSpeed;
                    //rotation.y -= (mousePos.x - (float)posx) * 1.25f * rotationSpeed;
                    //camera.rotate(glm::vec3((mousePos.y - (float)posy) * camera.rotationSpeed, -(mousePos.x - (float)posx) * camera.rotationSpeed, 0.0f));
                    //mousePos = glm::vec2((float)posx, (float)posy);
                    //viewUpdated = true;
                }
                if w_param & winapi::winuser::MK_MBUTTON != 0 {
                    //int32_t posx = LOWORD(lParam);
                    //int32_t posy = HIWORD(lParam);
                    //cameraPos.x -= (mousePos.x - (float)posx) * 0.01f;
                    //cameraPos.y -= (mousePos.y - (float)posy) * 0.01f;
                    //camera.translate(glm::vec3(-(mousePos.x - (float)posx) * 0.01f, -(mousePos.y - (float)posy) * 0.01f, 0.0f));
                    //viewUpdated = true;
                    //mousePos.x = (float)posx;
                    //mousePos.y = (float)posy;
                }
            }
            winapi::winuser::WM_SIZE => {
                //if ((prepared) && (wParam != SIZE_MINIMIZED))
                //{
                //	if ((resizing) || ((wParam == SIZE_MAXIMIZED) || (wParam == SIZE_RESTORED)))
                //	{
                //		destWidth = LOWORD(lParam);
                //		destHeight = HIWORD(lParam);
                //		windowResize();
                //	}
                //}
            }
            winapi::winuser::WM_SHOWWINDOW => {}
            winapi::winuser::WM_ENTERSIZEMOVE => {
                //resizing = true;
            }
            winapi::winuser::WM_EXITSIZEMOVE => {
                //resizing = false;
            }
            _ => {
                return unsafe { user32::DefWindowProcW(hwnd, msg, w_param, l_param) };
            }
        }
        return unsafe { user32::DefWindowProcW(hwnd, msg, w_param, l_param) };
    }
}
