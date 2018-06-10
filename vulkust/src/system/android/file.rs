#[cfg(not(target_os = "android"))]
use std::fs::File as StdFile;
use std::io::{Read, Result, Seek, SeekFrom};
#[cfg(not(target_os = "android"))]
use std::io::BufReader;
use std::mem::{size_of, transmute};
#[cfg(target_os = "android")]
use super::android::asset as aas;

pub struct File {
    offset: isize,
    #[cfg(not(target_os = "android"))]
    pub reader: BufReader<StdFile>,
    #[cfg(target_os = "android")]
    pub asset: *mut aas::AAsset,
}

impl File {
    #[cfg(not(target_os = "android"))]
    pub fn new(file_name: &String) -> Self {
        match StdFile::open(file_name) {
            Ok(f) => {
                File {
                    offset: 0,
                    reader: BufReader::new(f),
                }
            },
            Err(e) => {
                vxlogf!("Error {:?} in file reading.", e);
            }
        }
    }

    #[cfg(target_os = "android")]
    pub fn new(file_name: &String, asset_manager: *mut aas::AAssetManager) -> Self {
        use std::ffi::CString;
        use std::ptr::null_mut;
        // let file_name: String = file_name.clone();
        let cstr_name = CString::new(file_name.clone().into_bytes()).unwrap();
        let asset = unsafe {
            aas::AAssetManager_open(asset_manager, cstr_name.as_ptr(), aas::O_RDONLY.bits())
        };
        if asset == null_mut() {
            logf!("File {} not found!", file_name);
        }
        let mut file = File {
            endian_compatible: true,
            offset: 0,
            asset: asset,
        };
        file.check_endian();
        return file;
    }

    pub fn read_bytes(&mut self, count: usize) -> Vec<u8> {
        let mut b = vec![0u8; count];
        let mut read_count = 0;
        while read_count < count {
            let tmp_count = match self.read(&mut b[read_count..count]) {
                Ok(c) => c,
                Err(_) => {
                    vxlogf!("Error in reading stream.");
                }
            };
            read_count += tmp_count;
            if tmp_count == 0 {
                vxlogf!(
                    "Expected bytes count is {} but the read bytes count is {}.",
                    count,
                    read_count
                );
            }
        }
        return b;
    }

    pub fn tell(&self) -> isize {
        return self.offset;
    }

    pub fn goto(&mut self, offset: isize) {
        if offset == self.offset {
            return;
        }
        let _ = self.seek(SeekFrom::Start(offset as u64));
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        #[cfg(not(target_os = "android"))]
        let result = self.reader.read(buf);
        #[cfg(target_os = "android")]
        let result = Ok(unsafe {
            aas::AAsset_read(self.asset, transmute(buf.as_mut_ptr()), buf.len()) as usize
        });
        match result {
            Ok(c) => self.offset += c as isize,
            _ => {
                vxlogf!("Error in reading stream.");
            }
        };
        return result;
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        #[cfg(not(target_os = "android"))]        
        let result = self.reader.seek(pos);
        #[cfg(target_os = "android")]
        {
            let result = Ok(match pos {
                SeekFrom::Start(pos) => unsafe {
                    aas::AAsset_seek(self.asset, pos as isize, aas::SEEK_SET.bits()) as u64
                },
                SeekFrom::End(pos) => unsafe {
                    aas::AAsset_seek(self.asset, pos as isize, aas::SEEK_END.bits()) as u64
                },
                SeekFrom::Current(pos) => unsafe {
                    aas::AAsset_seek(self.asset, pos as isize, aas::SEEK_CUR.bits()) as u64
                },
            });
        }
        match result {
            Ok(c) => {
                self.offset = c as isize;
            }
            _ => {
                vxlogf!("Error in file seeking!");
            }
        }
        return result;
    }
}
