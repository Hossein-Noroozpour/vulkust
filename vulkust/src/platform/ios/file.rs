use super::super::apple;
use super::super::apple::NSString;
use std::ffi::CStr;
use std::fs::File as StdFile;
use std::io::{Read, Result, Seek, SeekFrom};
use std::os::raw::c_char;

pub struct File {
    pub file: StdFile,
}

pub static mut AASSET_MANAGER: isize = 0;

impl File {
    pub fn open(file_name: &str) -> Result<Self> {
        let file: Vec<_> = file_name.split('/').collect();
        let mut directory = String::new();
        if file.len() > 1 {
            directory += file[0];
        }
        for i in 1..file.len() - 1 {
            directory += "/";
            directory += file[i];
        }
        let file: Vec<_> = file[file.len() - 1].split('.').collect();
        let name = file[0];
        let mut file_type = String::new();
        if file.len() > 1 {
            file_type += file[1];
        }
        for i in 2..file.len() {
            file_type += ".";
            file_type += file[i];
        }
        let directory = NSString::new(&directory);
        let file_name = NSString::new(&name);
        let file_type = NSString::new(&file_type);
        let bundle = apple::get_class("NSBundle");
        let bundle: apple::Id = unsafe { msg_send![bundle, mainBundle] };
        let path: apple::Id = unsafe {
            msg_send![
                bundle,
                pathForResource:file_name
                ofType:file_type
                inDirectory:directory
            ]
        };
        let path = unsafe {
            let path: *const c_char = msg_send![path, fileSystemRepresentation];
            vx_result!(CStr::from_ptr(path).to_str())
        };
        let file = vx_result!(StdFile::open(path));
        Ok(File { file })
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.file.read(buf)
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.file.seek(pos)
    }
}
