use super::super::super::core::application::Application as CoreAppTrait;
use super::super::super::core::constants;
use super::super::super::core::event;
use super::super::super::core::string::string_to_cwstring;
use super::super::super::core::types::Real;
use super::super::super::render::engine::Engine as RenderEngine;
use super::super::super::winapi;
#[cfg(debug_mode)]
use std::fmt;
use std::mem::{size_of, transmute, zeroed};
use std::ptr::{null, null_mut};
use std::sync::{Arc, RwLock};

struct AppData {
    width: Real,
    height: Real,
    mouse_x: Real,
    mouse_y: Real,
}

pub struct Application {
    pub instance: winapi::shared::minwindef::HINSTANCE,
    pub window: winapi::shared::windef::HWND,
    pub core_app: Option<Arc<RwLock<CoreAppTrait>>>,
    pub renderer: Option<Arc<RwLock<RenderEngine>>>,
    data: Arc<RwLock<AppData>>,
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
            data: Arc::new(RwLock::new(AppData {
                width: constants::DEFAULT_WINDOW_WIDTH as Real,
                height: constants::DEFAULT_WINDOW_HEIGHT as Real,
                mouse_x: 0.0,
                mouse_y: 0.0,
            })),
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
        #[cfg(debug_mode)]
        {
            window_rect.right = constants::DEFAULT_WINDOW_WIDTH as winapi::um::winnt::LONG;
            window_rect.bottom = constants::DEFAULT_WINDOW_HEIGHT as winapi::um::winnt::LONG;
        }
        #[cfg(debug_mode)]
        let dwex_style =
            winapi::um::winuser::WS_EX_APPWINDOW | winapi::um::winuser::WS_EX_WINDOWEDGE;
        #[cfg(debug_mode)]
        let dw_style = winapi::um::winuser::WS_OVERLAPPEDWINDOW
            | winapi::um::winuser::WS_CLIPSIBLINGS
            | winapi::um::winuser::WS_CLIPCHILDREN;
        #[cfg(not(debug_mode))]
        let dwex_style = winapi::um::winuser::WS_EX_APPWINDOW;
        #[cfg(not(debug_mode))]
        let dw_style = winapi::um::winuser::WS_POPUP
            | winapi::um::winuser::WS_CLIPSIBLINGS
            | winapi::um::winuser::WS_CLIPCHILDREN;
        #[cfg(not(debug_mode))]
        {
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
            if screen_width != constants::DEFAULT_WINDOW_WIDTH as i32
                && screen_height != constants::DEFAULT_WINDOW_HEIGHT as i32
            {
                let result = unsafe {
                    winapi::um::winuser::ChangeDisplaySettingsW(
                        &mut dm_screen_settings,
                        winapi::um::winuser::CDS_FULLSCREEN,
                    )
                };
                if result != winapi::um::winuser::DISP_CHANGE_SUCCESSFUL {
                    vxloge!("ChangeDisplaySettingsW result: {:?}", result);
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
        #[cfg(not(debug_mode))]
        {
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
        let mut itself = vxresult!(itself.write());
        itself.instance = instance;
        itself.window = window;
        {
            let mut data = vxresult!(itself.data.write());
            data.width = window_rect.right as Real;
            data.width = window_rect.bottom as Real;
        }
        itself.get_mouse_position();
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
                vxresult!(vxunwrap!(&self.core_app).write()).update();
                vxresult!(vxunwrap!(&self.renderer).read()).update();
                if msg.message == winapi::um::winuser::WM_QUIT {
                    return true;
                }
            }
        }
    }

    pub fn get_mouse_position(&self) -> (Real, Real) {
        let mut data = vxresult!(self.data.write());
        let mut p = winapi::shared::windef::POINT { x: 0, y: 0 };
        if unsafe { winapi::um::winuser::GetCursorPos(&mut p) } == 0 {
            vxloge!("GetCursorPos failed");
        }
        if unsafe { winapi::um::winuser::ScreenToClient(self.window, &mut p) } == 0 {
            vxloge!("ScreenToClient failed");
        }
        data.mouse_x = p.x as Real / data.width;
        data.mouse_y = p.y as Real / data.width;
        return (data.mouse_x, data.mouse_y);
    }

    pub fn get_window_aspect_ratio(&self) -> Real {
        let mut rect = winapi::shared::windef::RECT {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        };
        unsafe {
            winapi::um::winuser::GetWindowRect(self.window, &mut rect);
        }
        let w = (rect.left - rect.right) as Real;
        let h = (rect.bottom - rect.top) as Real;
        let a = (w / h).abs();
        return a;
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
            we @ winapi::um::winuser::WM_KEYDOWN | we @ winapi::um::winuser::WM_KEYUP => {
                let button = match w_param {
                    0x41 => event::Button::Keyboard(event::Keyboard::A),
                    0x42 => event::Button::Keyboard(event::Keyboard::B),
                    0x43 => event::Button::Keyboard(event::Keyboard::C),
                    0x44 => event::Button::Keyboard(event::Keyboard::D),
                    0x45 => event::Button::Keyboard(event::Keyboard::E),
                    0x46 => event::Button::Keyboard(event::Keyboard::F),
                    0x47 => event::Button::Keyboard(event::Keyboard::G),
                    0x48 => event::Button::Keyboard(event::Keyboard::H),
                    0x49 => event::Button::Keyboard(event::Keyboard::I),
                    0x4A => event::Button::Keyboard(event::Keyboard::J),
                    0x4B => event::Button::Keyboard(event::Keyboard::K),
                    0x4C => event::Button::Keyboard(event::Keyboard::L),
                    0x4D => event::Button::Keyboard(event::Keyboard::M),
                    0x4E => event::Button::Keyboard(event::Keyboard::N),
                    0x4F => event::Button::Keyboard(event::Keyboard::O),
                    0x50 => event::Button::Keyboard(event::Keyboard::P),
                    0x51 => event::Button::Keyboard(event::Keyboard::Q),
                    0x52 => event::Button::Keyboard(event::Keyboard::R),
                    0x53 => event::Button::Keyboard(event::Keyboard::S),
                    0x54 => event::Button::Keyboard(event::Keyboard::T),
                    0x55 => event::Button::Keyboard(event::Keyboard::U),
                    0x56 => event::Button::Keyboard(event::Keyboard::V),
                    0x57 => event::Button::Keyboard(event::Keyboard::W),
                    0x58 => event::Button::Keyboard(event::Keyboard::X),
                    0x59 => event::Button::Keyboard(event::Keyboard::Y),
                    0x5A => event::Button::Keyboard(event::Keyboard::Z),
                    _ => event::Button::Keyboard(event::Keyboard::Unknown),
                };
                let action = match we {
                    winapi::um::winuser::WM_KEYDOWN => event::ButtonAction::Press,
                    winapi::um::winuser::WM_KEYUP => event::ButtonAction::Release,
                    _ => vxunexpected!(),
                };
                let e = event::Event::new(event::Type::Button { button, action });
                let core_app = vxunwrap!(&self.core_app);
                let core_app = vxresult!(core_app.read());
                core_app.on_event(e);
            }
            winapi::um::winuser::WM_RBUTTONDOWN => {
                let action = event::ButtonAction::Press;
                let button = event::Button::Mouse(event::Mouse::Right);
                let e = event::Event::new(event::Type::Button { button, action });
                vxresult!(vxunwrap!(&self.core_app).read()).on_event(e);
            }
            winapi::um::winuser::WM_LBUTTONDOWN => {
                let action = event::ButtonAction::Press;
                let button = event::Button::Mouse(event::Mouse::Left);
                let e = event::Event::new(event::Type::Button { button, action });
                vxresult!(vxunwrap!(&self.core_app).read()).on_event(e);
            }
            winapi::um::winuser::WM_MBUTTONDOWN => {
                let action = event::ButtonAction::Press;
                let button = event::Button::Mouse(event::Mouse::Middle);
                let e = event::Event::new(event::Type::Button { button, action });
                vxresult!(vxunwrap!(&self.core_app).read()).on_event(e);
            }
            winapi::um::winuser::WM_RBUTTONUP => {
                let action = event::ButtonAction::Release;
                let button = event::Button::Mouse(event::Mouse::Right);
                let e = event::Event::new(event::Type::Button { button, action });
                vxresult!(vxunwrap!(&self.core_app).read()).on_event(e);
            }
            winapi::um::winuser::WM_LBUTTONUP => {
                let action = event::ButtonAction::Release;
                let button = event::Button::Mouse(event::Mouse::Left);
                let e = event::Event::new(event::Type::Button { button, action });
                vxresult!(vxunwrap!(&self.core_app).read()).on_event(e);
            }
            winapi::um::winuser::WM_MBUTTONUP => {
                let action = event::ButtonAction::Release;
                let button = event::Button::Mouse(event::Mouse::Middle);
                let e = event::Event::new(event::Type::Button { button, action });
                vxresult!(vxunwrap!(&self.core_app).read()).on_event(e);
            }
            winapi::um::winuser::WM_MOUSEWHEEL => {
                //short wheelDelta = GET_WHEEL_DELTA_WPARAM(wParam);
                //zoom += (float)wheelDelta * 0.005f * zoomSpeed;
                //camera.translate(glm::vec3(0.0f, 0.0f, (float)wheelDelta * 0.005f * zoomSpeed));
                //viewUpdated = true;
            }
            winapi::um::winuser::WM_MOUSEMOVE => {
                let previous = {
                    let data = vxresult!(self.data.read());
                    (data.mouse_x, data.mouse_y)
                };
                let current = self.get_mouse_position();
                let delta = (current.0 - previous.0, current.1 - previous.1);
                let m = event::Move::Mouse {
                    previous,
                    current,
                    delta,
                };
                let e = event::Event::new(event::Type::Move(m));
                vxresult!(vxunwrap!(&self.core_app).read()).on_event(e);
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
