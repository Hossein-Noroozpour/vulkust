use super::{get_class, objc, Id, IdPtr, NSString, NSUInteger};
use std::mem::transmute;
use std::os::raw::c_void;

pub struct NSDictionaryBuilder {
    pub keys: Vec<NSString>,
    pub values: Vec<Id>,
}

impl NSDictionaryBuilder {
    pub fn new() -> Self {
        NSDictionaryBuilder {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn add_with_str(mut self, key: &str, value: Id) -> Self {
        self.keys.push(NSString::new(key));
        self.values.push(value);
        self
    }

    pub fn add(mut self, key: NSString, value: Id) -> Self {
        self.keys.push(key);
        self.values.push(value);
        self
    }

    pub fn build(self) -> NSDictionary {
        let values_ptr: IdPtr = unsafe { transmute(self.values.as_ptr()) };
        let keys_ptr: IdPtr = unsafe { transmute(self.keys.as_ptr()) };
        let count = self.keys.len() as NSUInteger;
        NSDictionary {
            id: unsafe {
                msg_send![
                    get_class("NSDictionary"),
                    dictionaryWithObjects:values_ptr
                    forKeys:keys_ptr
                    count:count
                ]
            },
        }
    }
}

#[repr(C)]
pub struct NSDictionary {
    id: Id,
}

unsafe impl objc::Encode for NSDictionary {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("@") }
    }
}

#[repr(C)]
pub struct NSNumber {
    pub id: Id,
}

unsafe impl objc::Encode for NSNumber {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("@") }
    }
}

const NSNUMBER_CLASS_NAME: &'static str = "NSNumber";

impl NSNumber {
    pub fn new_uint(v: NSUInteger) -> Self {
        NSNumber {
            id: unsafe { msg_send![get_class(NSNUMBER_CLASS_NAME), numberWithUnsignedInteger: v] },
        }
    }
}

#[repr(C)]
pub struct NSData {
    pub id: Id,
}

unsafe impl objc::Encode for NSData {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("@") }
    }
}

pub const NSDATA_CLASS_NAME: &'static str = "NSData";

impl NSData {
    pub fn new<T>(data: *const T, len: usize) -> Self {
        let data: *const c_void = unsafe { transmute(data) };
        let len = len as NSUInteger;
        NSData {
            id: unsafe { msg_send![get_class(NSDATA_CLASS_NAME), dataWithBytes:data length:len] },
        }
    }
}
