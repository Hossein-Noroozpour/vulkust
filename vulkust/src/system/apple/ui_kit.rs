use std::os::raw::{
    c_int,
    c_void,
    c_char,
};

use super::NSString;

#[link(name = "UIKit", kind = "framework")]
extern "C" {
    pub fn UIApplicationMain(argc: c_int, argv: *mut *mut c_char, pcn: NSString, dlg: NSString);
}