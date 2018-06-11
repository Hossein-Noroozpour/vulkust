#[cfg(target_os = "android")]
use super::os::file::File as OsFile;
#[cfg(not(target_os = "android"))]
use std::fs::File as OsFile;

pub type File = OsFile;
