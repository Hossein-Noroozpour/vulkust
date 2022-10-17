use std::os::raw::{c_char, c_int, c_void};

pub type AAssetManager = c_void;
pub type AAssetDir = c_void;
pub type AAsset = c_void;

bitflags! {
    pub struct AccesMode: c_int {
        const O_RDONLY	    = 00;
        const O_WRONLY	    = 01;
        const O_RDWR		= 02;
    }
}

bitflags! {
    pub struct SeekDir: c_int {
        const SEEK_SET = 0;
        const SEEK_CUR = 1;
        const SEEK_END = 2;
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AAssetMode {
    UNKNOWN = 0,
    RANDOM = 1,
    STREAMING = 2,
    BUFFER = 3,
}

#[cfg_attr(target_os = "android", link(name = "android", kind = "dylib"))]
extern "C" {
    pub fn AAssetManager_openDir(
        mgr: *mut AAssetManager,
        dir_name: *const c_char,
    ) -> *mut AAssetDir;
    pub fn AAssetManager_open(
        mgr: *mut AAssetManager,
        filename: *const c_char,
        mode: c_int,
    ) -> *mut AAsset;
    pub fn AAssetDir_getNextFileName(asset_dir: *mut AAssetDir) -> *const c_char;
    pub fn AAssetDir_rewind(asset_dir: *mut AAssetDir);
    pub fn AAssetDir_close(asset_dir: *mut AAssetDir);
    pub fn AAsset_read(asset: *mut AAsset, buf: *mut c_void, count: usize) -> c_int;
    pub fn AAsset_seek(asset: *mut AAsset, offset: isize, whence: c_int) -> isize;
    pub fn AAsset_close(asset: *mut AAsset);
    pub fn AAsset_getBuffer(asset: *mut AAsset) -> *const c_void;
    pub fn AAsset_getLength(asset: *mut AAsset) -> isize;
    pub fn AAsset_getRemainingLength(asset: *mut AAsset) -> isize;
    pub fn AAsset_openFileDescriptor(
        asset: *mut AAsset,
        out_start: *mut isize,
        out_length: *mut isize,
    ) -> c_int;
    pub fn AAsset_isAllocated(asset: *mut AAsset) -> c_int;
}
