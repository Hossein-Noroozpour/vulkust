use super::asset as aas;
use std::ffi::CString;
use std::io::{Read, Result, Seek, SeekFrom, Error, ErrorKind};
use std::mem::transmute;
use std::ptr::null_mut;
use std::fmt;

pub struct File {
    pub asset: *mut aas::AAsset,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Android-Asset-File")
    }
}

pub static mut AASSET_MANAGER: isize = 0;

impl File {
    pub fn open(file_name: &str) -> Result<Self> {
        let cstr_name = CString::new(file_name).unwrap();
        let asset = unsafe {
            aas::AAssetManager_open(
                transmute(AASSET_MANAGER),
                cstr_name.as_ptr(),
                aas::AccesMode::O_RDONLY.bits(),
            )
        };
        if asset == null_mut() {
            return Err(Error::new(ErrorKind::NotFound, format!("File {} not found!", file_name)));
        }
        Ok(File { asset: asset })
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(
            unsafe {
                aas::AAsset_read(self.asset, transmute(buf.as_mut_ptr()), buf.len()) as usize
            },
        )
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(match pos {
            SeekFrom::Start(pos) => unsafe {
                aas::AAsset_seek(self.asset, pos as isize, aas::SeekDir::SEEK_SET.bits()) as u64
            },
            SeekFrom::End(pos) => unsafe {
                aas::AAsset_seek(self.asset, pos as isize, aas::SeekDir::SEEK_END.bits()) as u64
            },
            SeekFrom::Current(pos) => unsafe {
                aas::AAsset_seek(self.asset, pos as isize, aas::SeekDir::SEEK_CUR.bits()) as u64
            },
        })
    }
}
