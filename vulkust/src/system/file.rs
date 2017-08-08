#[cfg(not(target_os = "android"))]
use std::fs::File as StdFile;
use std::io::{Read, Result, Seek, SeekFrom};
#[cfg(not(target_os = "android"))]
use std::io::BufReader;
use std::mem::{size_of, transmute};
#[cfg(target_os = "android")]
use super::android::asset as aas;

#[derive(Debug)]
pub struct File {
    pub endian_compatible: bool,
    #[cfg(not(target_os = "android"))]
    pub reader: BufReader<StdFile>,
    #[cfg(target_os = "android")]
    pub asset: *mut aas::AAsset,
}

impl File {
    #[cfg(target_endian = "big")]
    fn check_endian(&mut self) {
        if self.read_bool() {
            self.endian_compatible = false;
        } else {
            self.endian_compatible = true;
        }
    }

    #[cfg(target_endian = "little")]
    fn check_endian(&mut self) {
        if self.read_bool() {
            self.endian_compatible = true;
        } else {
            self.endian_compatible = false;
        }
    }

    #[cfg(not(target_os = "android"))]
    pub fn new(file_name: &String) -> Self {
        match StdFile::open(file_name) {
            Ok(f) => {
                let mut s = File {
                    endian_compatible: true,
                    reader: BufReader::new(f),
                };
                s.check_endian();
                s
            }
            Err(e) => {
                logf!("Error {:?} in file reading.", e);
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
            asset: asset,
        };
        file.check_endian();
        return file;
    }

    pub fn read_typed_bytes(&mut self, des: *mut u8, count: usize) {
        let b = self.read_bytes(count);
        if self.endian_compatible {
            for i in 0..count {
                unsafe {
                    *des.offset(i as isize) = b[i];
                }
            }
        } else {
            let mut i = 0isize;
            let mut j = count - 1;
            let count = count as isize;
            while i < count {
                unsafe {
                    *des.offset(i) = b[j];
                }
                i += 1;
                j -= 1;
            }
        }
    }

    pub fn read_bytes(&mut self, count: usize) -> Vec<u8> {
        let mut b = vec![0u8; count];
        let mut read_count = 0;
        while read_count < count {
            let tmp_count = match self.read(&mut b[read_count..count]) {
                Ok(c) => c,
                Err(_) => {
                    logf!("Error in reading stream.");
                }
            };
            read_count += tmp_count;
            if tmp_count == 0 {
                logf!(
                    "Expected bytes count is {} but the read bytes count is {}.",
                    count,
                    read_count
                );
            }
        }
        return b;
    }

    pub fn read_bool(&mut self) -> bool {
        let b = self.read_bytes(1);
        if b[0] == 1 {
            return true;
        }
        return false;
    }

    pub fn read_type<T>(&mut self) -> T
    where
        T: Default,
    {
        let mut r = T::default();
        self.read_typed_bytes(unsafe { transmute(&mut r) }, size_of::<T>());
        r
    }

    pub fn read_id(&mut self) -> u64 {
        self.read_type()
    }

    pub fn read_count(&mut self) -> u64 {
        self.read_type()
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        #[cfg(not(target_os = "android"))] return self.reader.read(buf);
        #[cfg(target_os = "android")]
        return Ok(unsafe {
            aas::AAsset_read(self.asset, transmute(buf.as_mut_ptr()), buf.len()) as usize
        });
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        #[cfg(not(target_os = "android"))] return self.reader.seek(pos);
        #[cfg(target_os = "android")]
        {
            return Ok(match pos {
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
    }
}
