use super::super::super::core::application::Application as CoreAppTrait;
use super::super::super::core::constants;
use super::super::super::core::string::string_to_cwstring;
use super::super::super::core::types::Real;
use super::super::super::render::engine::Engine as RenderEngine;
use super::super::super::winapi;
use std::fmt;
use std::mem::{size_of, transmute, zeroed};
use std::ptr::{null, null_mut};
use std::sync::{Arc, RwLock, Weak};

pub struct Application {
    pub instance: winapi::shared::minwindef::HINSTANCE,
    pub window: winapi::shared::windef::HWND,
    pub core_app: Option<Arc<RwLock<CoreAppTrait>>>,
    pub renderer: Option<Arc<RwLock<RenderEngine>>>,
}

extern "system" fn process_callback(
    hwnd: winapi::shared::windef::HWND,
    msg: winapi::shared::minwindef::UINT,
    w_param: winapi::shared::minwindef::WPARAM,
    l_param: winapi::shared::minwindef::LPARAM,
) -> winapi::shared::minwindef::LRESULT {
    let mut os_app =
        unsafe { winapi::um::winuser::GetWindowLongPtrW(hwnd, winapi::um::winuser::GWLP_USERDATA) };
    if winapi::um::winuser::WM_CREATE == msg {
        let ref create_structure: &mut winapi::um::winuser::CREATESTRUCTW =
            unsafe { transmute(l_param) };
        os_app = unsafe { transmute(create_structure.lpCreateParams) };
        unsafe {
            winapi::um::winuser::SetWindowLongPtrW(
                hwnd,
                winapi::um::winuser::GWLP_USERDATA,
                os_app,
            );
        }
    }
    if os_app == 0 {
        vxloge!("Unexpected message for nullptr sys app uMsg is: {}", msg);
        return unsafe { winapi::um::winuser::DefWindowProcW(hwnd, msg, w_param, l_param) };
    }
    let os_app: &'static Arc<RwLock<Application>> = unsafe { transmute(os_app) };
    let os_app = vxresult!(os_app.read());
    let result = os_app.handle_message(hwnd, msg, w_param, l_param);
    return result;
}

impl Application {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>) -> Self {
        Application {
            instance: 0 as winapi::shared::minwindef::HINSTANCE,
            window: 0 as winapi::shared::windef::HWND,
            core_app: Some(core_app),
            renderer: None,
        }
    }

    pub fn initialize(itself: &Arc<RwLock<Application>>) {
        let application_name = string_to_cwstring("Gearoenix Vulkust Game Engine");
        let instance = unsafe { winapi::um::libloaderapi::GetModuleHandleA(null()) };
        let mut wnd_class: winapi::um::winuser::WNDCLASSEXW = unsafe { zeroed() };
        wnd_class.cbSize = size_of::<winapi::um::winuser::WNDCLASSEXW>() as u32;
        wnd_class.style = winapi::um::winuser::CS_HREDRAW | winapi::um::winuser::CS_VREDRAW;
        wnd_class.lpfnWndProc = Some(process_callback);
        wnd_class.hInstance = instance;
        wnd_class.hIcon = unsafe {
            winapi::um::winuser::LoadIconW(
                0 as winapi::shared::minwindef::HINSTANCE,
                winapi::um::winuser::IDI_APPLICATION,
            )
        };
        wnd_class.hCursor = unsafe {
            winapi::um::winuser::LoadCursorW(
                0 as winapi::shared::minwindef::HINSTANCE,
                winapi::um::winuser::IDC_ARROW,
            )
        };
        wnd_class.hbrBackground = unsafe {
            winapi::um::wingdi::GetStockObject(
                winapi::um::wingdi::BLACK_BRUSH as winapi::ctypes::c_int,
            )
        } as winapi::shared::windef::HBRUSH;
        wnd_class.lpszClassName = application_name.as_ptr();
        wnd_class.hIconSm = unsafe {
            winapi::um::winuser::LoadIconW(
                0 as winapi::shared::minwindef::HINSTANCE,
                winapi::um::winuser::IDI_WINLOGO,
            )
        };
        if unsafe { winapi::um::winuser::RegisterClassExW(&wnd_class) }
            == 0 as winapi::shared::minwindef::ATOM
        {
            vxlogf!("Could not register window class!");
        }
        let mut window_rect: winapi::shared::windef::RECT = unsafe { zeroed() };
        #[cfg(debug_assertions)]
        {
            window_rect.right = constants::DEFAULT_WINDOW_WIDTH as winapi::um::winnt::LONG;
            window_rect.bottom = constants::DEFAULT_WINDOW_HEIGHT as winapi::um::winnt::LONG;
        }
        #[cfg(debug_assertions)]
        let dwex_style =
            winapi::um::winuser::WS_EX_APPWINDOW | winapi::um::winuser::WS_EX_WINDOWEDGE;
        #[cfg(debug_assertions)]
        let dw_style = winapi::um::winuser::WS_OVERLAPPEDWINDOW
            | winapi::um::winuser::WS_CLIPSIBLINGS
            | winapi::um::winuser::WS_CLIPCHILDREN;
        #[cfg(not(debug_assertions))]
        let dwex_style = winapi::um::winuser::WS_EX_APPWINDOW;
        #[cfg(not(debug_assertions))]
        let dw_style = winapi::um::winuser::WS_POPUP
            | winapi::um::winuser::WS_CLIPSIBLINGS
            | winapi::um::winuser::WS_CLIPCHILDREN;
        #[cfg(not(debug_assertions))]
        {
            let screen_width =
                unsafe { winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN) };
            let screen_height =
                unsafe { winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN) };
            let mut dm_screen_settings: winapi::um::wingdi::DEVMODEW = unsafe { zeroed() };
            dm_screen_settings.dmSize = size_of::<winapi::um::wingdi::DEVMODEW>();
            dm_screen_settings.dmPelsWidth = screen_width;
            dm_screen_settings.dmPelsHeight = screen_height;
            dm_screen_settings.dmBitsPerPel = 32;
            dm_screen_settings.dmFields = winapi::um::wingdi::DM_BITSPERPEL
                | winapi::um::wingdi::DM_PELSWIDTH
                | winapi::um::wingdi::DM_PELSHEIGHT;
            if screen_width != constants::DEFAULT_WINDOW_WIDTH
                && screen_height != constants::DEFAULT_WINDOW_HEIGHT
            {
                if winapi::um::winuser::ChangeDisplaySettingsW(
                    &dm_screen_settings,
                    winapi::um::winuser::CDS_FULLSCREEN,
                ) != winapi::um::winuser::DISP_CHANGE_SUCCESSFUL
                {
                    vxlogf!("Fullscreen Mode not supported!");
                }
            }
            window_rect.right = screen_width;
            window_rect.bottom = screen_height;
        }
        unsafe {
            winapi::um::winuser::AdjustWindowRectEx(
                &mut window_rect,
                dw_style,
                winapi::shared::minwindef::FALSE,
                dwex_style,
            );
        }
        let window = unsafe {
            winapi::um::winuser::CreateWindowExW(
                0,
                application_name.as_ptr(),
                application_name.as_ptr(),
                dw_style
                    | winapi::um::winuser::WS_CLIPSIBLINGS
                    | winapi::um::winuser::WS_CLIPCHILDREN,
                0,
                0,
                window_rect.right - window_rect.left,
                window_rect.bottom - window_rect.top,
                null_mut(),
                null_mut(),
                instance,
                transmute(Box::into_raw(Box::new(itself.clone()))),
            )
        };
        if window == null_mut() {
            vxlogf!("Could not create window!");
        }
        #[cfg(not(debug_assertions))]
        {
            let x = (winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN)
                - window_rect.right)
                / 2;
            let y = (winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN)
                - window_rect.bottom)
                / 2;
            winapi::um::winuser::SetWindowPos(
                window,
                0,
                x,
                y,
                0,
                0,
                winapi::um::winuser::SWP_NOZORDER | winapi::um::winuser::SWP_NOSIZE,
            );
        }
        unsafe {
            winapi::um::winuser::ShowWindow(window, winapi::um::winuser::SW_SHOW);
        }
        unsafe {
            winapi::um::winuser::SetForegroundWindow(window);
        }
        unsafe {
            winapi::um::winuser::SetFocus(window);
        }
        unsafe {
            winapi::um::winuser::UpdateWindow(window);
        }
        let mut itself = vxresult!(itself.write());
        itself.instance = instance;
        itself.window = window;
    }

    pub fn set_renderer(&mut self, r: Arc<RwLock<RenderEngine>>) {
        self.renderer = Some(r);
    }

    pub fn run(&self) -> bool {
        loop {
            let mut msg: winapi::um::winuser::MSG = unsafe { zeroed() };
            while unsafe {
                winapi::um::winuser::PeekMessageW(
                    &mut msg,
                    0 as winapi::shared::windef::HWND,
                    0,
                    0,
                    winapi::um::winuser::PM_REMOVE,
                )
            } != 0
            {
                unsafe {
                    winapi::um::winuser::TranslateMessage(&msg);
                    winapi::um::winuser::DispatchMessageW(&msg);
                }
                vxresult!(vxunwrap!(&self.renderer).read()).update();
                if msg.message == winapi::um::winuser::WM_QUIT {
                    return true;
                }
            }
        }
    }

    fn get_mouse_position(&self) -> (f64, f64) {
        vxloge!("TODO");
        // TODO
        (0.0, 0.0)
    }

    pub fn get_window_aspect_ratio(&self) -> Real {
        vxloge!("TODO");
        1.7
    }

    fn handle_message(
        &self,
        hwnd: winapi::shared::windef::HWND,
        msg: winapi::shared::minwindef::UINT,
        w_param: winapi::shared::minwindef::WPARAM,
        l_param: winapi::shared::minwindef::LPARAM,
    ) -> winapi::shared::minwindef::LRESULT {
        match msg {
            winapi::um::winuser::WM_CLOSE => {
                // TODO: proper termination
                // running = false;
                // DestroyWindow(hWnd);
                // PostQuitMessage(0);
            }
            winapi::um::winuser::WM_PAINT => unsafe {
                winapi::um::winuser::ValidateRect(hwnd, null());
            },
            winapi::um::winuser::WM_KEYDOWN => {
                let vk_f1 = winapi::um::winuser::VK_F1 as winapi::shared::minwindef::WPARAM;
                let vk_escape = winapi::um::winuser::VK_ESCAPE as winapi::shared::minwindef::WPARAM;
                if w_param == 0x50 {
                    // p
                    // TODO pause
                } else if w_param == vk_f1 {
                    /*if (enableTextOverlay) {
        				textOverlay->visible = !textOverlay->visible;
        			}*/
                } else if w_param == vk_escape {
                    unsafe {
                        winapi::um::winuser::PostQuitMessage(0);
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
            winapi::um::winuser::WM_KEYUP => {
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
            winapi::um::winuser::WM_RBUTTONDOWN => {}
            winapi::um::winuser::WM_LBUTTONDOWN => {}
            winapi::um::winuser::WM_MBUTTONDOWN => {
                //mousePos.x = (float)LOWORD(lParam);
                //mousePos.y = (float)HIWORD(lParam);
            }
            winapi::um::winuser::WM_MOUSEWHEEL => {
                //short wheelDelta = GET_WHEEL_DELTA_WPARAM(wParam);
                //zoom += (float)wheelDelta * 0.005f * zoomSpeed;
                //camera.translate(glm::vec3(0.0f, 0.0f, (float)wheelDelta * 0.005f * zoomSpeed));
                //viewUpdated = true;
            }
            winapi::um::winuser::WM_MOUSEMOVE => {
                if w_param & winapi::um::winuser::MK_RBUTTON != 0 {
                    //int32_t posx = LOWORD(lParam);
                    //int32_t posy = HIWORD(lParam);
                    //zoom += (mousePos.y - (float)posy) * .005f * zoomSpeed;
                    //camera.translate(glm::vec3(-0.0f, 0.0f, (mousePos.y - (float)posy) * .005f * zoomSpeed));
                    //mousePos = glm::vec2((float)posx, (float)posy);
                    //viewUpdated = true;
                }
                if w_param & winapi::um::winuser::MK_LBUTTON != 0 {
                    //int32_t posx = LOWORD(lParam);
                    //int32_t posy = HIWORD(lParam);
                    //rotation.x += (mousePos.y - (float)posy) * 1.25f * rotationSpeed;
                    //rotation.y -= (mousePos.x - (float)posx) * 1.25f * rotationSpeed;
                    //camera.rotate(glm::vec3((mousePos.y - (float)posy) * camera.rotationSpeed, -(mousePos.x - (float)posx) * camera.rotationSpeed, 0.0f));
                    //mousePos = glm::vec2((float)posx, (float)posy);
                    //viewUpdated = true;
                }
                if w_param & winapi::um::winuser::MK_MBUTTON != 0 {
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
            winapi::um::winuser::WM_SIZE => {
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
            winapi::um::winuser::WM_SHOWWINDOW => {}
            winapi::um::winuser::WM_ENTERSIZEMOVE => {
                //resizing = true;
            }
            winapi::um::winuser::WM_EXITSIZEMOVE => {
                //resizing = false;
            }
            _ => {
                return unsafe { winapi::um::winuser::DefWindowProcW(hwnd, msg, w_param, l_param) };
            }
        }
        return unsafe { winapi::um::winuser::DefWindowProcW(hwnd, msg, w_param, l_param) };
    }
}

impl fmt::Debug for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OS-Application-Windows")
    }
}
