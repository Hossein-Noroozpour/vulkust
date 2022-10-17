#[cfg(target_os = "android")]
use super::os::file::File as OsFile;
#[cfg(target_os = "ios")]
use super::os::file::File as OsFile;

#[cfg(desktop_os)]
use std::fs::File as OsFile;

pub type File = OsFile;
