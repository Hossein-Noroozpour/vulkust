#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
extern crate libc;
pub mod log;

use libc::{
    c_void,
};

pub type JavaVM = *const c_void;
pub type JNIEnv = *const c_void;
pub type jobject = *const c_void;


const JNI_VERSION_1_6: i32 = 0x00010006;


#[no_mangle] pub unsafe extern fn JNI_OnLoad(vm: *const JavaVM, reserved: *const c_void) -> i32 {
    println!("Started with vm: {:?} and reserved: {:?}!", vm, reserved);
    JNI_VERSION_1_6
}

#[no_mangle] pub unsafe extern fn JNI_OnUnload(vm :*const JavaVM, reserved: *const c_void) {
    println!("Ended with vm: {:?} and reserved: {:?}!", vm, reserved);
}

#[no_mangle] pub unsafe extern fn Java_com_gearoenix_vulkust_GameActivity_initialize(env: *const JNIEnv, this: jobject) {
    println!("Initializing with env: {:?} and obj: {:?}!", env, this);
    let l = log::Log::new();
    l.write("Hello");
}