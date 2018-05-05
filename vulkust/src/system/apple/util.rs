use super::objc::runtime::{Class, Object, Sel};
use super::objc::MessageArguments;
use super::Id;
use std::any::Any;
use std::mem;

type Imp = unsafe extern "C" fn();

#[cfg(all(any(target_os = "macos", target_os = "ios"), target_arch = "x86_64"))]
fn msg_send_fn<R>(obj: Id, _: Sel) -> (Imp, Id) {
    extern "C" {
        fn objc_msgSend();
        fn objc_msgSend_stret();
    }
    let msg_fn = if mem::size_of::<R>() <= 16 {
        objc_msgSend
    } else {
        objc_msgSend_stret
    };
    (msg_fn, obj)
}

#[cfg(all(any(target_os = "macos", target_os = "ios"), target_arch = "aarch64"))]
fn msg_send_fn<R>(obj: Id, _: Sel) -> (Imp, Id) {
    extern "C" {
        fn objc_msgSend();
    }
    (objc_msgSend, obj)
}

#[cfg(all(any(target_os = "macos", target_os = "ios"), target_arch = "arm"))]
fn msg_send_fn<R: Any>(obj: Id, _: Sel) -> (Imp, Id) {
    extern "C" {
        fn objc_msgSend();
        fn objc_msgSend_stret();
    }
    let type_id = TypeId::of::<R>();
    let msg_fn = if mem::size_of::<R>() <= 4 || type_id == TypeId::of::<i64>()
        || type_id == TypeId::of::<u64>() || type_id == TypeId::of::<f64>()
    {
        objc_msgSend
    } else {
        objc_msgSend_stret
    };
    (msg_fn, obj)
}

pub trait Receiver {}

impl Receiver for Object {}
impl Receiver for Class {}

pub fn send_unverified<T, A, R>(obj: *const T, sel: Sel, args: A) -> R
where
    T: Receiver,
    A: MessageArguments,
    R: Any,
{
    let (msg_send_fn, receiver) = msg_send_fn::<R>(unsafe { mem::transmute(obj) }, sel);
    unsafe { A::invoke(msg_send_fn, receiver, sel, args) }
}
