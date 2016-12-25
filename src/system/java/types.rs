#![allow(non_camel_case_types)]

use std::os::raw::{
    c_int,
    c_void,
    c_char,
    c_uchar,
    c_short,
    c_ushort,
    c_longlong,
};

pub type jboolean = c_uchar;
pub type jbyte = c_char;
pub type jchar = c_ushort;
pub type jshort = c_short;
pub type jint = c_int;
pub type jlong = c_longlong;
pub type jfloat = f32;
pub type jdouble = f64;
pub type jsize = jint;
pub type jobject = *mut c_void;
pub type jclass = jobject;
pub type jstring = jobject;
pub type jarray = jobject;
pub type jobjectArray = jarray;
pub type jbooleanArray = jarray;
pub type jbyteArray = jarray;
pub type jcharArray = jarray;
pub type jshortArray = jarray;
pub type jintArray = jarray;
pub type jlongArray = jarray;
pub type jfloatArray = jarray;
pub type jdoubleArray = jarray;
pub type jthrowable = jobject;
pub type jweak = jobject;
pub type jfieldID = *mut c_void;
pub type jmethodID = *mut c_void;