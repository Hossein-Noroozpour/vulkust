use std::os::raw::c_uint;
pub(super) const KEY_PRESS: c_uint = 2;
pub(super) const KEY_RELEASE: c_uint = 3;
pub(super) const BUTTON_PRESS: c_uint = 4;
pub(super) const BUTTON_RELEASE: c_uint = 5;
pub(super) const MOTION_NOTIFY: c_uint = 6;
pub(super) const DESTROY_NOTIFY: c_uint = 17;
pub(super) const CONFIGURE_NOTIFY: c_uint = 22;
pub(super) const CLIENT_MESSAGE: c_uint = 33;
pub(super) const KEY_A: u8 = 38;
// pub(super) const KEY_B: u8 = 48;
// pub(super) const KEY_C: u8 = 46;
pub(super) const KEY_D: u8 = 40;
// pub(super) const KEY_E: u8 = 26;
// pub(super) const KEY_F: u8 = 33;
// pub(super) const KEY_G: u8 = 34;
// pub(super) const KEY_H: u8 = 35;
// pub(super) const KEY_I: u8 = 23;
// pub(super) const KEY_J: u8 = 36;
// pub(super) const KEY_K: u8 = 37;
// pub(super) const KEY_L: u8 = 38;
// pub(super) const KEY_M: u8 = 50;
// pub(super) const KEY_N: u8 = 49;
// pub(super) const KEY_O: u8 = 24;
// pub(super) const KEY_P: u8 = 25;
// pub(super) const KEY_Q: u8 = 24;
// pub(super) const KEY_R: u8 = 27;
pub(super) const KEY_S: u8 = 39;
// pub(super) const KEY_T: u8 = 28;
// pub(super) const KEY_U: u8 = 30;
// pub(super) const KEY_V: u8 = 47;
pub(super) const KEY_W: u8 = 25;
// pub(super) const KEY_X: u8 = 45;
// pub(super) const KEY_Y: u8 = 29;
// pub(super) const KEY_Z: u8 = 44;
pub(super) const KEY_F1: u8 = 67;
// pub(super) const KEY_F2: u8 = 60;
// pub(super) const KEY_F3: u8 = 61;
// pub(super) const KEY_F4: u8 = 62;
// pub(super) const KEY_F5: u8 = 63;
// pub(super) const KEY_F6: u8 = 64;
// pub(super) const KEY_F7: u8 = 65;
// pub(super) const KEY_F8: u8 = 66;
// pub(super) const KEY_F9: u8 = 67;
// pub(super) const KEY_F10: u8 = 68;
// pub(super) const KEY_F11: u8 = 87;
// pub(super) const KEY_F12: u8 = 88;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(super) enum PropMode {
    Replace = 0,
    // Prepend = 1,
    // Append = 2,
}