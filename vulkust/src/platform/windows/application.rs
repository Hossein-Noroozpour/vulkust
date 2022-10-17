use super::super::{
    super::core::string::string_to_cwstring,
    base::Base,
    config::{Config, ScreenState},
};
#[cfg(debug_mode)]
use std::fmt;
use std::{
    mem::{size_of, transmute, transmute_copy, zeroed},
    ptr::{null, null_mut},
};

pub struct Application {
    pub base: Base,
    pub instance: winapi::shared::minwindef::HINSTANCE,
    pub window: winapi::shared::windef::HWND,
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
        vx_log_e!(
            "Unexpected message for nullptr platform application uMsg is: {}",
            msg
        );
        return unsafe { winapi::um::winuser::DefWindowProcW(hwnd, msg, w_param, l_param) };
    }
    let os_app: &mut Application = unsafe { transmute(os_app) };
    os_app.handle_message(hwnd, msg, w_param, l_param)
}

impl Application {
    pub fn new() -> Self {
        let config = Config::default();
        let mut myself = Self {
            base: Base::new(config),
            instance: 0 as winapi::shared::minwindef::HINSTANCE,
            window: 0 as winapi::shared::windef::HWND,
        };
        let instance = unsafe { winapi::um::libloaderapi::GetModuleHandleW(null()) };
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
        let application_name = string_to_cwstring(&myself.base.config.application_name);
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
            vx_log_f!("Could not register window class!");
        }
        let mut window_rect: winapi::shared::windef::RECT = unsafe { zeroed() };
        if let ScreenState::Windowed(windowed) = &myself.base.config.screen_state {
            window_rect.right = windowed.width as winapi::um::winnt::LONG;
            window_rect.bottom = windowed.height as winapi::um::winnt::LONG;
        }
        let dwex_style = if let ScreenState::Fullscreen = myself.base.config.screen_state {
            winapi::um::winuser::WS_EX_APPWINDOW
        } else {
            winapi::um::winuser::WS_EX_APPWINDOW | winapi::um::winuser::WS_EX_WINDOWEDGE
        };
        let dw_style = if let ScreenState::Fullscreen = myself.base.config.screen_state {
            winapi::um::winuser::WS_POPUP
                | winapi::um::winuser::WS_CLIPSIBLINGS
                | winapi::um::winuser::WS_CLIPCHILDREN
        } else {
            winapi::um::winuser::WS_OVERLAPPEDWINDOW
                | winapi::um::winuser::WS_CLIPSIBLINGS
                | winapi::um::winuser::WS_CLIPCHILDREN
        };
        if let ScreenState::Fullscreen = myself.base.config.screen_state {
            let screen_width =
                unsafe { winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN) };
            let screen_height =
                unsafe { winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN) };
            let mut dm_screen_settings: winapi::um::wingdi::DEVMODEW = unsafe { zeroed() };
            dm_screen_settings.dmSize = size_of::<winapi::um::wingdi::DEVMODEW>() as u16;
            dm_screen_settings.dmPelsWidth = screen_width as u32;
            dm_screen_settings.dmPelsHeight = screen_height as u32;
            dm_screen_settings.dmBitsPerPel = 32;
            dm_screen_settings.dmFields = winapi::um::wingdi::DM_BITSPERPEL
                | winapi::um::wingdi::DM_PELSWIDTH
                | winapi::um::wingdi::DM_PELSHEIGHT;
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
                transmute(&myself),
            )
        };
        if window == null_mut() {
            vx_log_f!("Could not create window!");
        }
        if let ScreenState::Fullscreen = myself.base.config.screen_state {
        } else {
            let x = (unsafe {
                winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN)
            } - window_rect.right)
                / 2;
            let y = (unsafe {
                winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN)
            } - window_rect.bottom)
                / 2;
            unsafe {
                winapi::um::winuser::SetWindowPos(
                    window,
                    null_mut(),
                    x,
                    y,
                    0,
                    0,
                    winapi::um::winuser::SWP_NOZORDER | winapi::um::winuser::SWP_NOSIZE,
                );
            }
        }
        unsafe {
            winapi::um::winuser::ShowWindow(window, winapi::um::winuser::SW_SHOW);
            winapi::um::winuser::SetForegroundWindow(window);
            winapi::um::winuser::SetFocus(window);
            winapi::um::winuser::UpdateWindow(window);
        }
        while !myself.base.is_running {
            let mut msg = winapi::um::winuser::MSG::default();
            unsafe {
                winapi::um::winuser::GetMessageW(&mut msg, null_mut(), 0, 0);
                winapi::um::winuser::TranslateMessage(&mut msg);
                winapi::um::winuser::DispatchMessageW(&mut msg);
            }
        }
        myself.instance = instance;
        myself.window = window;
        myself
    }

    pub fn run(&mut self) {
        unsafe {
            winapi::um::winuser::SetWindowLongPtrW(
                self.window,
                winapi::um::winuser::GWLP_USERDATA,
                transmute_copy(&self),
            );
        }
        while self.base.is_running {
            let mut msg = winapi::um::winuser::MSG::default();
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
            }
        }
    }

    // pub fn get_mouse_position(&self) -> (Real, Real) {
    //     let mut data = vx_result!(self.data.write());
    //     let mut p = winapi::shared::windef::POINT { x: 0, y: 0 };
    //     if unsafe { winapi::um::winuser::GetCursorPos(&mut p) } == 0 {
    //         vx_log_e!("GetCursorPos failed");
    //     }
    //     if unsafe { winapi::um::winuser::ScreenToClient(self.window, &mut p) } == 0 {
    //         vx_log_e!("ScreenToClient failed");
    //     }
    //     data.mouse_x = p.x as Real / data.height;
    //     data.mouse_y = p.y as Real / data.height;
    //     return (data.mouse_x, data.mouse_y);
    // }

    // pub fn get_window_aspect_ratio(&self) -> Real {
    //     let mut rect = winapi::shared::windef::RECT {
    //         left: 0,
    //         right: 0,
    //         top: 0,
    //         bottom: 0,
    //     };
    //     unsafe {
    //         winapi::um::winuser::GetWindowRect(self.window, &mut rect);
    //     }
    //     let w = (rect.left - rect.right) as Real;
    //     let h = (rect.bottom - rect.top) as Real;
    //     let a = (w / h).abs();
    //     return a;
    // }

    fn handle_message(
        &mut self,
        hwnd: winapi::shared::windef::HWND,
        msg: winapi::shared::minwindef::UINT,
        w_param: winapi::shared::minwindef::WPARAM,
        l_param: winapi::shared::minwindef::LPARAM,
    ) -> winapi::shared::minwindef::LRESULT {
        match msg {
            winapi::um::winuser::WM_QUIT | winapi::um::winuser::WM_CLOSE => {
                self.base.is_running = false;
                unsafe {
                    winapi::um::winuser::DestroyWindow(self.window);
                    winapi::um::winuser::PostQuitMessage(0);
                }
            }
            winapi::um::winuser::WM_PAINT => unsafe {
                winapi::um::winuser::ValidateRect(hwnd, null());
            },
            we @ winapi::um::winuser::WM_KEYDOWN | we @ winapi::um::winuser::WM_KEYUP => {
                // let button = match w_param {
                //     0x41 => event::Button::Keyboard(event::Keyboard::A),
                //     0x42 => event::Button::Keyboard(event::Keyboard::B),
                //     0x43 => event::Button::Keyboard(event::Keyboard::C),
                //     0x44 => event::Button::Keyboard(event::Keyboard::D),
                //     0x45 => event::Button::Keyboard(event::Keyboard::E),
                //     0x46 => event::Button::Keyboard(event::Keyboard::F),
                //     0x47 => event::Button::Keyboard(event::Keyboard::G),
                //     0x48 => event::Button::Keyboard(event::Keyboard::H),
                //     0x49 => event::Button::Keyboard(event::Keyboard::I),
                //     0x4A => event::Button::Keyboard(event::Keyboard::J),
                //     0x4B => event::Button::Keyboard(event::Keyboard::K),
                //     0x4C => event::Button::Keyboard(event::Keyboard::L),
                //     0x4D => event::Button::Keyboard(event::Keyboard::M),
                //     0x4E => event::Button::Keyboard(event::Keyboard::N),
                //     0x4F => event::Button::Keyboard(event::Keyboard::O),
                //     0x50 => event::Button::Keyboard(event::Keyboard::P),
                //     0x51 => event::Button::Keyboard(event::Keyboard::Q),
                //     0x52 => event::Button::Keyboard(event::Keyboard::R),
                //     0x53 => event::Button::Keyboard(event::Keyboard::S),
                //     0x54 => event::Button::Keyboard(event::Keyboard::T),
                //     0x55 => event::Button::Keyboard(event::Keyboard::U),
                //     0x56 => event::Button::Keyboard(event::Keyboard::V),
                //     0x57 => event::Button::Keyboard(event::Keyboard::W),
                //     0x58 => event::Button::Keyboard(event::Keyboard::X),
                //     0x59 => event::Button::Keyboard(event::Keyboard::Y),
                //     0x5A => event::Button::Keyboard(event::Keyboard::Z),
                //     _ => event::Button::Keyboard(event::Keyboard::Unknown),
                // };
                // let action = match we {
                //     winapi::um::winuser::WM_KEYDOWN => event::ButtonAction::Press,
                //     winapi::um::winuser::WM_KEYUP => event::ButtonAction::Release,
                //     _ => vx_unexpected!(),
                // };
                // let e = event::Event::new(event::Type::Button { button, action });
                // let core_app = vx_unwrap!(&self.core_app);
                // let core_app = vx_result!(core_app.read());
                // core_app.on_event(e);
            }
            winapi::um::winuser::WM_RBUTTONDOWN => {
                // let action = event::ButtonAction::Press;
                // let button = event::Button::Mouse(event::Mouse::Right);
                // let e = event::Event::new(event::Type::Button { button, action });
                // vx_result!(vx_unwrap!(&self.core_app).read()).on_event(e);
            }
            winapi::um::winuser::WM_LBUTTONDOWN => {
                // let action = event::ButtonAction::Press;
                // let button = event::Button::Mouse(event::Mouse::Left);
                // let e = event::Event::new(event::Type::Button { button, action });
                // vx_result!(vx_unwrap!(&self.core_app).read()).on_event(e);
            }
            winapi::um::winuser::WM_MBUTTONDOWN => {
                // let action = event::ButtonAction::Press;
                // let button = event::Button::Mouse(event::Mouse::Middle);
                // let e = event::Event::new(event::Type::Button { button, action });
                // vx_result!(vx_unwrap!(&self.core_app).read()).on_event(e);
            }
            winapi::um::winuser::WM_RBUTTONUP => {
                // let action = event::ButtonAction::Release;
                // let button = event::Button::Mouse(event::Mouse::Right);
                // let e = event::Event::new(event::Type::Button { button, action });
                // vx_result!(vx_unwrap!(&self.core_app).read()).on_event(e);
            }
            winapi::um::winuser::WM_LBUTTONUP => {
                // let action = event::ButtonAction::Release;
                // let button = event::Button::Mouse(event::Mouse::Left);
                // let e = event::Event::new(event::Type::Button { button, action });
                // vx_result!(vx_unwrap!(&self.core_app).read()).on_event(e);
            }
            winapi::um::winuser::WM_MBUTTONUP => {
                // let action = event::ButtonAction::Release;
                // let button = event::Button::Mouse(event::Mouse::Middle);
                // let e = event::Event::new(event::Type::Button { button, action });
                // vx_result!(vx_unwrap!(&self.core_app).read()).on_event(e);
            }
            winapi::um::winuser::WM_MOUSEWHEEL => {
                //short wheelDelta = GET_WHEEL_DELTA_WPARAM(wParam);
                //zoom += (float)wheelDelta * 0.005f * zoomSpeed;
                //camera.translate(glm::vec3(0.0f, 0.0f, (float)wheelDelta * 0.005f * zoomSpeed));
                //viewUpdated = true;
            }
            winapi::um::winuser::WM_MOUSEMOVE => {
                // let previous = {
                //     let data = vx_result!(self.data.read());
                //     (data.mouse_x, data.mouse_y)
                // };
                // let current = self.get_mouse_position();
                // let delta = (current.0 - previous.0, current.1 - previous.1);
                // let m = event::Move::Mouse {
                //     previous,
                //     current,
                //     delta,
                // };
                // let e = event::Event::new(event::Type::Move(m));
                // vx_result!(vx_unwrap!(&self.core_app).read()).on_event(e);
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
            winapi::um::winuser::WM_SHOWWINDOW => {
                self.base.is_running = true;
            }
            winapi::um::winuser::WM_ENTERSIZEMOVE => {
                //resizing = true;
            }
            winapi::um::winuser::WM_EXITSIZEMOVE => {
                //resizing = false;
            }
            _ => {}
        }
        return unsafe { winapi::um::winuser::DefWindowProcW(hwnd, msg, w_param, l_param) };
    }
}

#[cfg(debug_mode)]
impl fmt::Debug for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OS-Application-Windows")
    }
}
