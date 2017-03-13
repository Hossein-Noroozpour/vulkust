use std::ffi::CString;
use std::os::raw::c_char;

pub fn slice_to_string(s: &[u8]) -> String {
    let mut r = String::new();
    for c in s {
        if *c == 0 {
            break;
        }
        r.push(*c as char);
    }
    r
}

pub fn strings_to_cstrings(ss: Vec<String>) -> Vec<CString> {
    let mut r = Vec::new();
    for s in ss {
        r.push(CString::new(s.into_bytes()).unwrap());
    }
    r
}

pub fn cstrings_to_ptrs(cs: &Vec<CString>) -> Vec<*const c_char> {
    let mut r = Vec::new();
    for c in cs {
        r.push(c.as_ptr());
    }
    r
}