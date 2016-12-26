#![allow(non_camel_case_types, non_snake_case)]

use std::os::raw::{
    c_int,
    c_void,
    c_char,
    c_uchar,
    c_short,
    c_ushort,
    c_longlong,
};
use std::mem::{
    transmute,
    zeroed,
};
use std::default::Default;

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

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct jvalue {
    data: u64,
}

impl jvalue {
    pub unsafe fn z(&mut self) -> *mut jboolean {
        transmute(&mut self.data)
    }
    pub unsafe fn b(&mut self) -> *mut jbyte {
        transmute(&mut self.data)
    }
    pub unsafe fn c(&mut self) -> *mut jchar {
        transmute(&mut self.data)
    }
    pub unsafe fn s(&mut self) -> *mut jshort {
        transmute(&mut self.data)
    }
    pub unsafe fn i(&mut self) -> *mut jint {
        transmute(&mut self.data)
    }
    pub unsafe fn j(&mut self) -> *mut jlong {
        transmute(&mut self.data)
    }
    pub unsafe fn f(&mut self) -> *mut jfloat {
        transmute(&mut self.data)
    }
    pub unsafe fn d(&mut self) -> *mut jdouble {
        transmute(&mut self.data)
    }
    pub unsafe fn l(&mut self) -> *mut jobject {
        transmute(&mut self.data)
    }
}

impl Default for jvalue {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum jobjectRefType {
    JNIInvalidRefType = 0,
    JNILocalRefType = 1,
    JNIGlobalRefType = 2,
    JNIWeakGlobalRefType = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct JNINativeMethod {
    pub name: *const c_char,
    pub signature: *const c_char,
    pub fnPtr: *mut c_void,
}

impl Default for JNINativeMethod {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type C_JNIEnv = *const JNINativeInterface;
pub type JNIEnv = *const JNINativeInterface;
pub type JavaVM = *const JNIInvokeInterface;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct JNINativeInterface {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub reserved3: *mut c_void,
    pub GetVersion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv) -> jint,
    pub DefineClass: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: *const c_char, arg3: jobject, arg4: *const jbyte, arg5: jsize) -> jclass,
    pub FindClass: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: *const c_char) -> jclass,
    pub FromReflectedMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject) -> jmethodID,
    pub FromReflectedField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject) -> jfieldID,
    pub ToReflectedMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: jboolean) -> jobject,
    pub GetSuperclass: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass) -> jclass,
    pub IsAssignableFrom: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jclass) -> jboolean,
    pub ToReflectedField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID, arg4: jboolean) -> jobject,
    pub Throw: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jthrowable) -> jint,
    pub ThrowNew: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: *const c_char) -> jint,
    pub ExceptionOccurred: *mut unsafe extern "C" fn(arg1: *mut JNIEnv) -> jthrowable,
    pub ExceptionDescribe: *mut unsafe extern "C" fn(arg1: *mut JNIEnv),
    pub ExceptionClear: *mut unsafe extern "C" fn(arg1: *mut JNIEnv),
    pub FatalError: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: *const c_char),
    pub PushLocalFrame: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jint) -> jint,
    pub PopLocalFrame: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject) -> jobject,
    pub NewGlobalRef: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject) -> jobject,
    pub DeleteGlobalRef: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject),
    pub DeleteLocalRef: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject),
    pub IsSameObject: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jobject) -> jboolean,
    pub NewLocalRef: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject) -> jobject,
    pub EnsureLocalCapacity: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jint) -> jint,
    pub AllocObject: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass) -> jobject,
    pub NewObject: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, ...) -> jobject,
    pub NewObjectV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut c_void) -> jobject,
    pub NewObjectA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut jvalue) -> jobject,
    pub GetObjectClass: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject) -> jclass,
    pub IsInstanceOf: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass) -> jboolean,
    pub GetMethodID: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: *const c_char, arg4: *const c_char) -> jmethodID,
    pub CallObjectMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, ...) -> jobject,
    pub CallObjectMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut c_void) -> jobject,
    pub CallObjectMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut jvalue) -> jobject,
    pub CallBooleanMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, ...) -> jboolean,
    pub CallBooleanMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut c_void) -> jboolean,
    pub CallBooleanMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut jvalue) -> jboolean,
    pub CallByteMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, ...) -> jbyte,
    pub CallByteMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut c_void) -> jbyte,
    pub CallByteMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut jvalue) -> jbyte,
    pub CallCharMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, ...) -> jchar,
    pub CallCharMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut c_void) -> jchar,
    pub CallCharMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut jvalue) -> jchar,
    pub CallShortMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, ...) -> jshort,
    pub CallShortMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut c_void) -> jshort,
    pub CallShortMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut jvalue) -> jshort,
    pub CallIntMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, ...) -> jint,
    pub CallIntMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut c_void) -> jint,
    pub CallIntMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut jvalue) -> jint,
    pub CallLongMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, ...) -> jlong,
    pub CallLongMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut c_void) -> jlong,
    pub CallLongMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut jvalue) -> jlong,
    pub CallFloatMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, ...) -> jfloat,
    pub CallFloatMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut c_void) -> jfloat,
    pub CallFloatMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut jvalue) -> jfloat,
    pub CallDoubleMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, ...) -> jdouble,
    pub CallDoubleMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut c_void) -> jdouble,
    pub CallDoubleMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut jvalue) -> jdouble,
    pub CallVoidMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, ...),
    pub CallVoidMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut c_void),
    pub CallVoidMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jmethodID, arg4: *mut jvalue),
    pub CallNonvirtualObjectMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, ...) -> jobject,
    pub CallNonvirtualObjectMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut c_void) -> jobject,
    pub CallNonvirtualObjectMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut jvalue) -> jobject,
    pub CallNonvirtualBooleanMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, ...) -> jboolean,
    pub CallNonvirtualBooleanMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut c_void) -> jboolean,
    pub CallNonvirtualBooleanMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut jvalue) -> jboolean,
    pub CallNonvirtualByteMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, ...) -> jbyte,
    pub CallNonvirtualByteMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut c_void) -> jbyte,
    pub CallNonvirtualByteMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut jvalue) -> jbyte,
    pub CallNonvirtualCharMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, ...) -> jchar,
    pub CallNonvirtualCharMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut c_void) -> jchar,
    pub CallNonvirtualCharMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut jvalue) -> jchar,
    pub CallNonvirtualShortMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, ...) -> jshort,
    pub CallNonvirtualShortMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut c_void) -> jshort,
    pub CallNonvirtualShortMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut jvalue) -> jshort,
    pub CallNonvirtualIntMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, ...) -> jint,
    pub CallNonvirtualIntMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut c_void) -> jint,
    pub CallNonvirtualIntMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut jvalue) -> jint,
    pub CallNonvirtualLongMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, ...) -> jlong,
    pub CallNonvirtualLongMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut c_void) -> jlong,
    pub CallNonvirtualLongMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut jvalue) -> jlong,
    pub CallNonvirtualFloatMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, ...) -> jfloat,
    pub CallNonvirtualFloatMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut c_void) -> jfloat,
    pub CallNonvirtualFloatMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut jvalue) -> jfloat,
    pub CallNonvirtualDoubleMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, ...) -> jdouble,
    pub CallNonvirtualDoubleMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut c_void) -> jdouble,
    pub CallNonvirtualDoubleMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut jvalue) -> jdouble,
    pub CallNonvirtualVoidMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, ...),
    pub CallNonvirtualVoidMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut c_void),
    pub CallNonvirtualVoidMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jclass, arg4: jmethodID, arg5: *mut jvalue),
    pub GetFieldID: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: *const c_char, arg4: *const c_char) -> jfieldID,
    pub GetObjectField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jobject,
    pub GetBooleanField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jboolean,
    pub GetByteField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jbyte,
    pub GetCharField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jchar,
    pub GetShortField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jshort,
    pub GetIntField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jint,
    pub GetLongField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jlong,
    pub GetFloatField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jfloat,
    pub GetDoubleField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jdouble,
    pub SetObjectField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jobject),
    pub SetBooleanField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jboolean),
    pub SetByteField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jbyte),
    pub SetCharField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jchar),
    pub SetShortField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jshort),
    pub SetIntField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jint),
    pub SetLongField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jlong),
    pub SetFloatField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jfloat),
    pub SetDoubleField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jdouble),
    pub GetStaticMethodID: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: *const c_char, arg4: *const c_char) -> jmethodID,
    pub CallStaticObjectMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, ...) -> jobject,
    pub CallStaticObjectMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut c_void) -> jobject,
    pub CallStaticObjectMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut jvalue) -> jobject,
    pub CallStaticBooleanMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, ...) -> jboolean,
    pub CallStaticBooleanMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut c_void) -> jboolean,
    pub CallStaticBooleanMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut jvalue) -> jboolean,
    pub CallStaticByteMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, ...) -> jbyte,
    pub CallStaticByteMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut c_void) -> jbyte,
    pub CallStaticByteMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut jvalue) -> jbyte,
    pub CallStaticCharMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, ...) -> jchar,
    pub CallStaticCharMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut c_void) -> jchar,
    pub CallStaticCharMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut jvalue) -> jchar,
    pub CallStaticShortMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, ...) -> jshort,
    pub CallStaticShortMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut c_void) -> jshort,
    pub CallStaticShortMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut jvalue) -> jshort,
    pub CallStaticIntMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, ...) -> jint,
    pub CallStaticIntMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut c_void) -> jint,
    pub CallStaticIntMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut jvalue) -> jint,
    pub CallStaticLongMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, ...) -> jlong,
    pub CallStaticLongMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut c_void) -> jlong,
    pub CallStaticLongMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut jvalue) -> jlong,
    pub CallStaticFloatMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, ...) -> jfloat,
    pub CallStaticFloatMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut c_void) -> jfloat,
    pub CallStaticFloatMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut jvalue) -> jfloat,
    pub CallStaticDoubleMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, ...) -> jdouble,
    pub CallStaticDoubleMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut c_void) -> jdouble,
    pub CallStaticDoubleMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut jvalue) -> jdouble,
    pub CallStaticVoidMethod: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, ...),
    pub CallStaticVoidMethodV: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut c_void),
    pub CallStaticVoidMethodA: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jmethodID, arg4: *mut jvalue),
    pub GetStaticFieldID: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: *const c_char, arg4: *const c_char) -> jfieldID,
    pub GetStaticObjectField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jobject,
    pub GetStaticBooleanField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jboolean,
    pub GetStaticByteField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jbyte,
    pub GetStaticCharField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jchar,
    pub GetStaticShortField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jshort,
    pub GetStaticIntField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jint,
    pub GetStaticLongField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jlong,
    pub GetStaticFloatField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jfloat,
    pub GetStaticDoubleField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jdouble,
    pub SetStaticObjectField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID, arg4: jobject),
    pub SetStaticBooleanField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID, arg4: jboolean),
    pub SetStaticByteField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID, arg4: jbyte),
    pub SetStaticCharField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID, arg4: jchar),
    pub SetStaticShortField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID, arg4: jshort),
    pub SetStaticIntField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID, arg4: jint),
    pub SetStaticLongField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID, arg4: jlong),
    pub SetStaticFloatField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID, arg4: jfloat),
    pub SetStaticDoubleField: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: jfieldID, arg4: jdouble),
    pub NewString: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: *const jchar, arg3: jsize) -> jstring,
    pub GetStringLength: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jstring) -> jsize,
    pub GetStringChars: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jstring, arg3: *mut jboolean) -> *const jchar,
    pub ReleaseStringChars: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jstring, arg3: *const jchar),
    pub NewStringUTF: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: *const c_char) -> jstring,
    pub GetStringUTFLength: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jstring) -> jsize,
    pub GetStringUTFChars: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jstring, arg3: *mut jboolean) -> *const c_char,
    pub ReleaseStringUTFChars: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jstring, arg3: *const c_char),
    pub GetArrayLength: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jarray) -> jsize,
    pub NewObjectArray: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jsize, arg3: jclass, arg4: jobject) -> jobjectArray,
    pub GetObjectArrayElement: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobjectArray, arg3: jsize) -> jobject,
    pub SetObjectArrayElement: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobjectArray, arg3: jsize, arg4: jobject),
    pub NewBooleanArray: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jsize) -> jbooleanArray,
    pub NewByteArray: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jsize) -> jbyteArray,
    pub NewCharArray: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jsize) -> jcharArray,
    pub NewShortArray: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jsize) -> jshortArray,
    pub NewIntArray: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jsize) -> jintArray,
    pub NewLongArray: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jsize) -> jlongArray,
    pub NewFloatArray: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jsize) -> jfloatArray,
    pub NewDoubleArray: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jsize) -> jdoubleArray,
    pub GetBooleanArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jbooleanArray, arg3: *mut jboolean) -> *mut jboolean,
    pub GetByteArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jbyteArray, arg3: *mut jboolean) -> *mut jbyte,
    pub GetCharArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jcharArray, arg3: *mut jboolean) -> *mut jchar,
    pub GetShortArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jshortArray, arg3: *mut jboolean) -> *mut jshort,
    pub GetIntArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jintArray, arg3: *mut jboolean) -> *mut jint,
    pub GetLongArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jlongArray, arg3: *mut jboolean) -> *mut jlong,
    pub GetFloatArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jfloatArray, arg3: *mut jboolean) -> *mut jfloat,
    pub GetDoubleArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jdoubleArray, arg3: *mut jboolean) -> *mut jdouble,
    pub ReleaseBooleanArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jbooleanArray, arg3: *mut jboolean, arg4: jint),
    pub ReleaseByteArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jbyteArray, arg3: *mut jbyte, arg4: jint),
    pub ReleaseCharArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jcharArray, arg3: *mut jchar, arg4: jint),
    pub ReleaseShortArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jshortArray, arg3: *mut jshort, arg4: jint),
    pub ReleaseIntArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jintArray, arg3: *mut jint, arg4: jint),
    pub ReleaseLongArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jlongArray, arg3: *mut jlong, arg4: jint),
    pub ReleaseFloatArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jfloatArray, arg3: *mut jfloat, arg4: jint),
    pub ReleaseDoubleArrayElements: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jdoubleArray, arg3: *mut jdouble, arg4: jint),
    pub GetBooleanArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jbooleanArray, arg3: jsize, arg4: jsize, arg5: *mut jboolean),
    pub GetByteArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jbyteArray, arg3: jsize, arg4: jsize, arg5: *mut jbyte),
    pub GetCharArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jcharArray, arg3: jsize, arg4: jsize, arg5: *mut jchar),
    pub GetShortArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jshortArray, arg3: jsize, arg4: jsize, arg5: *mut jshort),
    pub GetIntArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jintArray, arg3: jsize, arg4: jsize, arg5: *mut jint),
    pub GetLongArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jlongArray, arg3: jsize, arg4: jsize, arg5: *mut jlong),
    pub GetFloatArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jfloatArray, arg3: jsize, arg4: jsize, arg5: *mut jfloat),
    pub GetDoubleArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jdoubleArray, arg3: jsize, arg4: jsize, arg5: *mut jdouble),
    pub SetBooleanArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jbooleanArray, arg3: jsize, arg4: jsize, arg5: *const jboolean),
    pub SetByteArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jbyteArray, arg3: jsize, arg4: jsize, arg5: *const jbyte),
    pub SetCharArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jcharArray, arg3: jsize, arg4: jsize, arg5: *const jchar),
    pub SetShortArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jshortArray, arg3: jsize, arg4: jsize, arg5: *const jshort),
    pub SetIntArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jintArray, arg3: jsize, arg4: jsize, arg5: *const jint),
    pub SetLongArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jlongArray, arg3: jsize, arg4: jsize, arg5: *const jlong),
    pub SetFloatArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jfloatArray, arg3: jsize, arg4: jsize, arg5: *const jfloat),
    pub SetDoubleArrayRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jdoubleArray, arg3: jsize, arg4: jsize, arg5: *const jdouble),
    pub RegisterNatives: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass, arg3: *const JNINativeMethod, arg4: jint) -> jint,
    pub UnregisterNatives: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jclass) -> jint,
    pub MonitorEnter: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject) -> jint,
    pub MonitorExit: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject) -> jint,
    pub GetJavaVM: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: *mut *mut JavaVM) -> jint,
    pub GetStringRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jstring, arg3: jsize, arg4: jsize, arg5: *mut jchar),
    pub GetStringUTFRegion: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jstring, arg3: jsize, arg4: jsize, arg5: *mut c_char),
    pub GetPrimitiveArrayCritical: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jarray, arg3: *mut jboolean) -> *mut c_void,
    pub ReleasePrimitiveArrayCritical: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jarray, arg3: *mut c_void, arg4: jint),
    pub GetStringCritical: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jstring, arg3: *mut jboolean) -> *const jchar,
    pub ReleaseStringCritical: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jstring, arg3: *const jchar),
    pub NewWeakGlobalRef: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject) -> jweak,
    pub DeleteWeakGlobalRef: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jweak),
    pub ExceptionCheck: *mut unsafe extern "C" fn(arg1: *mut JNIEnv) -> jboolean,
    pub NewDirectByteBuffer: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: *mut c_void, arg3: jlong) -> jobject,
    pub GetDirectBufferAddress: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject) -> *mut c_void,
    pub GetDirectBufferCapacity: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject) -> jlong,
    pub GetObjectRefType: *mut unsafe extern "C" fn(arg1: *mut JNIEnv, arg2: jobject) -> jobjectRefType,
}

impl Default for JNINativeInterface {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct _JNIEnv {
    pub functions: *const JNINativeInterface,
}

impl Default for _JNIEnv {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct JNIInvokeInterface {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub DestroyJavaVM: *mut unsafe extern "C" fn(arg1: *mut JavaVM) -> jint,
    pub AttachCurrentThread: *mut unsafe extern "C" fn(arg1: *mut JavaVM, arg2: *mut *mut JNIEnv, arg3: *mut c_void) -> jint,
    pub DetachCurrentThread: *mut unsafe extern "C" fn(arg1: *mut JavaVM) -> jint,
    pub GetEnv: *mut unsafe extern "C" fn(arg1: *mut JavaVM, arg2: *mut *mut c_void, arg3: jint) -> jint,
    pub AttachCurrentThreadAsDaemon: *mut unsafe extern "C" fn(arg1: *mut JavaVM, arg2: *mut *mut JNIEnv, arg3: *mut c_void) -> jint,
}

impl Default for JNIInvokeInterface {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct _JavaVM {
    pub functions: *const JNIInvokeInterface,
}

impl Default for _JavaVM {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct JavaVMAttachArgs {
    pub version: jint,
    pub name: *const c_char,
    pub group: jobject,
}

impl Default for JavaVMAttachArgs {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct JavaVMOption {
    pub optionString: *const c_char,
    pub extraInfo: *mut c_void,
}

impl Default for JavaVMOption {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct JavaVMInitArgs {
    pub version: jint,
    pub nOptions: jint,
    pub options: *mut JavaVMOption,
    pub ignoreUnrecognized: jboolean,
}

impl Default for JavaVMInitArgs {
    fn default() -> Self { unsafe { zeroed() } }
}

const JNI_VERSION_1_6: jint = 0x00010006;

#[no_mangle]
pub unsafe extern fn JNI_OnLoad(vm: *const JavaVM, reserved: *const c_void) -> jint {
    #![allow(unused_unsafe)]
    logdbg!(format!("Started with vm: {:?} and reserved: {:?}!", vm, reserved));
    JNI_VERSION_1_6
}

#[no_mangle]
pub unsafe extern fn JNI_OnUnload(vm: *const JavaVM, reserved: *const c_void) {
    #![allow(unused_unsafe)]
    logdbg!(format!("Ended with vm: {:?} and reserved: {:?}!", vm, reserved));
}