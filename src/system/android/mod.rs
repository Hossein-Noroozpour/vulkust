#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

#[macro_use] pub mod log;
#[macro_use] pub mod activity;

use std::os::raw::{
    c_void,
};

use super::super::render;


pub type jobject = *const c_void;


const JNI_VERSION_1_6: i32 = 0x00010006;


#[no_mangle] pub unsafe extern fn JNI_OnLoad(vm: *const JavaVM, reserved: *const c_void) -> i32 {
    #![allow(unused_unsafe)]
    logdbg!(format!("Started with vm: {:?} and reserved: {:?}!", vm, reserved));
    JNI_VERSION_1_6
}

#[no_mangle] pub unsafe extern fn JNI_OnUnload(vm :*const JavaVM, reserved: *const c_void) {
    #![allow(unused_unsafe)]
    logdbg!(format!("Ended with vm: {:?} and reserved: {:?}!", vm, reserved));
}

#[no_mangle] pub unsafe extern fn Java_com_gearoenix_vulkust_GameActivity_initialize(env: *const JNIEnv, this: jobject) {
    #![allow(unused_unsafe)]
    logdbg!(format!("Initializing with env: {:?} and obj: {:?}!", env, this));
    render::initialize();
    logdbg!("Initialization is done.");
}