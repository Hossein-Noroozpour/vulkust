use std::os::raw::{
    c_char,
};

pub enum AConfiguration {}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Orientation {
    Any = 0x0000,
    Port = 0x0001,
    Land = 0x0002,
    Square = 0x0003,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Touchscreen {
    Any = 0x0000,
    NoTouch = 0x0001,
    Stylus = 0x0002,
    Finger = 0x0003,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Density {
    Default = 0,
    Low = 120,
    Medium = 160,
    High = 240,
    None = 0xffff,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Keyboard {
    Any = 0x0000,
    Nokeys = 0x0001,
    Qwerty = 0x0002,
    K12Key = 0x0003,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Navigation {
    Any = 0x0000,
    Nonav = 0x0001,
    Dpad = 0x0002,
    Trackball = 0x0003,
    Wheel = 0x0004,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum KeysHidden {
    Any = 0x0000,
    No = 0x0001,
    Yes = 0x0002,
    Soft = 0x0003,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum NavHidden {
    Any = 0x0000,
    No = 0x0001,
    Yes = 0x0002,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ScreenSize {
    Any = 0x00,
    Small = 0x01,
    Normal = 0x02,
    Large = 0x03,
    XLarge = 0x04,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ScreenLong {
    Any = 0x00,
    No = 0x1,
    Yes = 0x2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum UiModeType {
    Any = 0x00,
    Normal = 0x01,
    Desk = 0x02,
    Car = 0x03,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum UiModeNight {
    Any = 0x00,
    No = 0x1,
    Yes = 0x2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Id {
    Mcc = 0x0001,
    Mnc = 0x0002,
    Locale = 0x0004,
    Touchscreen = 0x0008,
    Keyboard = 0x0010,
    KeyboardHidden = 0x0020,
    Navigation = 0x0040,
    Orientation = 0x0080,
    Density = 0x0100,
    ScreenSize = 0x0200,
    Version = 0x0400,
    ScreenLayout = 0x0800,
    UiMode = 0x1000,
}

#[cfg_attr(target_os = "android", link(name = "android", kind = "dylib"))]
extern {
    pub fn AConfiguration_new() -> *mut AConfiguration;
    pub fn AConfiguration_delete(config: *mut AConfiguration);
    pub fn AConfiguration_fromAssetManager(out: *mut AConfiguration, am: *mut AAssetManager);
    pub fn AConfiguration_copy(dest: *mut AConfiguration, src: *mut AConfiguration);
    pub fn AConfiguration_getMcc(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setMcc(config: *mut AConfiguration, mcc: i32);
    pub fn AConfiguration_getMnc(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setMnc(config: *mut AConfiguration, mnc: i32);
    pub fn AConfiguration_getLanguage(config: *mut AConfiguration, out_language: *mut c_char);
    pub fn AConfiguration_setLanguage(config: *mut AConfiguration, language: *const c_char);
    pub fn AConfiguration_getCountry(config: *mut AConfiguration, out_country: *mut c_char);
    pub fn AConfiguration_setCountry(config: *mut AConfiguration, country: *const c_char);
    pub fn AConfiguration_getOrientation(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setOrientation(config: *mut AConfiguration, orientation: i32);
    pub fn AConfiguration_getTouchscreen(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setTouchscreen(config: *mut AConfiguration, touchscreen: i32);
    pub fn AConfiguration_getDensity(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setDensity(config: *mut AConfiguration, density: i32);
    pub fn AConfiguration_getKeyboard(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setKeyboard(config: *mut AConfiguration, keyboard: i32);
    pub fn AConfiguration_getNavigation(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setNavigation(config: *mut AConfiguration, navigation: i32);
    pub fn AConfiguration_getKeysHidden(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setKeysHidden(config: *mut AConfiguration, keys_hidden: i32);
    pub fn AConfiguration_getNavHidden(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setNavHidden(config: *mut AConfiguration, nav_hidden: i32);
    pub fn AConfiguration_getSdkVersion(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setSdkVersion(config: *mut AConfiguration, sdk_version: i32);
    pub fn AConfiguration_getScreenSize(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setScreenSize(config: *mut AConfiguration, screen_size: i32);
    pub fn AConfiguration_getScreenLong(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setScreenLong(config: *mut AConfiguration, screen_long: i32);
    pub fn AConfiguration_getUiModeType(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setUiModeType(config: *mut AConfiguration, ui_mode_type: i32);
    pub fn AConfiguration_getUiModeNight(config: *mut AConfiguration) -> i32;
    pub fn AConfiguration_setUiModeNight(config: *mut AConfiguration, ui_mode_night: i32);
    pub fn AConfiguration_diff(config1: *mut AConfiguration, config2: *mut AConfiguration) -> i32;
    pub fn AConfiguration_match(base: *mut AConfiguration, requested: *mut AConfiguration) -> i32;
    pub fn AConfiguration_isBetterThan(base: *mut AConfiguration, test: *mut AConfiguration, requested: *mut AConfiguration) -> i32;
}
