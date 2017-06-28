use std::os::raw::c_char;
use super::asset::AAssetManager;

pub enum AConfiguration {}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Orientation {
    Any = 0x0000,
    Port = 0x0001,
    Land = 0x0002,
    Square = 0x0003,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Touchscreen {
    Any = 0x0000,
    NoTouch = 0x0001,
    Stylus = 0x0002,
    Finger = 0x0003,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Density {
    Default = 0,
    Low = 120,
    Medium = 160,
    High = 240,
    None = 0xffff,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Keyboard {
    Any = 0x0000,
    Nokeys = 0x0001,
    Qwerty = 0x0002,
    K12Key = 0x0003,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Navigation {
    Any = 0x0000,
    Nonav = 0x0001,
    Dpad = 0x0002,
    Trackball = 0x0003,
    Wheel = 0x0004,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum KeysHidden {
    Any = 0x0000,
    No = 0x0001,
    Yes = 0x0002,
    Soft = 0x0003,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum NavHidden {
    Any = 0x0000,
    No = 0x0001,
    Yes = 0x0002,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum ScreenSize {
    Any = 0x00,
    Small = 0x01,
    Normal = 0x02,
    Large = 0x03,
    XLarge = 0x04,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum ScreenLong {
    Any = 0x00,
    No = 0x1,
    Yes = 0x2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum UiModeType {
    Any = 0x00,
    Normal = 0x01,
    Desk = 0x02,
    Car = 0x03,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum UiModeNight {
    Any = 0x00,
    No = 0x1,
    Yes = 0x2,
}

#[repr(i32)]
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
extern "C" {
    pub fn AConfiguration_new() -> *mut AConfiguration;
    pub fn AConfiguration_delete(config: *mut AConfiguration);
    pub fn AConfiguration_fromAssetManager(out: *mut AConfiguration, am: *mut AAssetManager);
    pub fn AConfiguration_copy(dest: *mut AConfiguration, src: *mut AConfiguration);
    pub fn AConfiguration_getMcc(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setMcc(config: *mut AConfiguration, mcc: i32);
    pub fn AConfiguration_getMnc(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setMnc(config: *mut AConfiguration, mnc: i32);
    pub fn AConfiguration_getLanguage(config: *const AConfiguration, out_language: *mut c_char);
    pub fn AConfiguration_setLanguage(config: *mut AConfiguration, language: *const c_char);
    pub fn AConfiguration_getCountry(config: *const AConfiguration, out_country: *mut c_char);
    pub fn AConfiguration_setCountry(config: *mut AConfiguration, country: *const c_char);
    pub fn AConfiguration_getOrientation(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setOrientation(config: *mut AConfiguration, orientation: i32);
    pub fn AConfiguration_getTouchscreen(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setTouchscreen(config: *mut AConfiguration, touchscreen: i32);
    pub fn AConfiguration_getDensity(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setDensity(config: *mut AConfiguration, density: i32);
    pub fn AConfiguration_getKeyboard(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setKeyboard(config: *mut AConfiguration, keyboard: i32);
    pub fn AConfiguration_getNavigation(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setNavigation(config: *mut AConfiguration, navigation: i32);
    pub fn AConfiguration_getKeysHidden(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setKeysHidden(config: *mut AConfiguration, keys_hidden: i32);
    pub fn AConfiguration_getNavHidden(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setNavHidden(config: *mut AConfiguration, nav_hidden: i32);
    pub fn AConfiguration_getSdkVersion(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setSdkVersion(config: *mut AConfiguration, sdk_version: i32);
    pub fn AConfiguration_getScreenSize(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setScreenSize(config: *mut AConfiguration, screen_size: i32);
    pub fn AConfiguration_getScreenLong(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setScreenLong(config: *mut AConfiguration, screen_long: i32);
    pub fn AConfiguration_getUiModeType(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setUiModeType(config: *mut AConfiguration, ui_mode_type: i32);
    pub fn AConfiguration_getUiModeNight(config: *const AConfiguration) -> i32;
    pub fn AConfiguration_setUiModeNight(config: *mut AConfiguration, ui_mode_night: i32);
    pub fn AConfiguration_diff(config1: *mut AConfiguration, config2: *mut AConfiguration) -> i32;
    pub fn AConfiguration_match(base: *mut AConfiguration, requested: *mut AConfiguration) -> i32;
    pub fn AConfiguration_isBetterThan(
        base: *mut AConfiguration,
        test: *mut AConfiguration,
        requested: *mut AConfiguration,
    ) -> i32;
}

use std::fmt::{Debug, Result, Formatter};

impl Debug for AConfiguration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut lang = [0 as c_char; 2];
        let mut country = [0 as c_char; 2];
        unsafe {
            AConfiguration_getLanguage(self, lang.as_mut_ptr());
        }
        unsafe {
            AConfiguration_getCountry(self, country.as_mut_ptr());
        }
        write!(
            f, "\nAConfiguration {{\n    mcc : {:?}\n    mnc : {:?}\n    lang : {}{}\
            \n    cnt : {}{}\n    orientation : {:?}\n    touch : {:?}\n    dens : {:?}\
            \n    keys : {:?}\n    nav : {:?}\n    keysHid : {:?}\n    navHid : {:?}\
            \n    sdk : {:?}\n    size : {:?}\n    long : {:?}\n    modeType : {:?}\
            \n    modeNight : {:?}\n}}",
            unsafe {AConfiguration_getMcc(self) },
            unsafe {AConfiguration_getMnc(self) },
            lang[0] as char, lang[1] as char, country[0] as char, country[1] as char,
            unsafe { AConfiguration_getOrientation(self) },
            unsafe { AConfiguration_getTouchscreen(self) },
            unsafe { AConfiguration_getDensity(self) },
            unsafe { AConfiguration_getKeyboard(self) },
            unsafe { AConfiguration_getNavigation(self) },
            unsafe { AConfiguration_getKeysHidden(self) },
            unsafe { AConfiguration_getNavHidden(self) },
            unsafe { AConfiguration_getSdkVersion(self) },
            unsafe { AConfiguration_getScreenSize(self) },
            unsafe { AConfiguration_getScreenLong(self) },
            unsafe { AConfiguration_getUiModeType(self) },
            unsafe { AConfiguration_getUiModeNight(self) }
        )
    }
}
