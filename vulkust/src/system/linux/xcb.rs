#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_char;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
impl ::std::default::Default for __fsid_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __qaddr_t = *mut __quad_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type ssize_t = isize;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type time_t = __time_t;
pub type clockid_t = __clockid_t;
pub type timer_t = __timer_t;
pub type size_t = usize;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint_ = ::std::os::raw::c_uint;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulong;
pub type register_t = ::std::os::raw::c_long;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub const XCB_COPY_FROM_PARENT: u64 = 0;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
impl ::std::default::Default for __sigset_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
impl ::std::default::Default for timespec {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
impl ::std::default::Default for timeval {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
impl ::std::default::Default for fd_set {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type fd_mask = __fd_mask;
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy)]
pub struct pthread_attr_t {
    pub _bindgen_data_: [u64; 7usize],
}
impl pthread_attr_t {
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 56usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for pthread_attr_t {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for pthread_attr_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
impl ::std::default::Default for __pthread_internal_list {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Copy)]
pub struct pthread_mutex_t {
    pub _bindgen_data_: [u64; 5usize],
}
impl pthread_mutex_t {
    pub unsafe fn __data(&mut self) -> *mut __pthread_mutex_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 40usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for pthread_mutex_t {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for pthread_mutex_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
impl ::std::default::Default for __pthread_mutex_s {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct pthread_mutexattr_t {
    pub _bindgen_data_: [u32; 1usize],
}
impl pthread_mutexattr_t {
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for pthread_mutexattr_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct pthread_cond_t {
    pub _bindgen_data_: [u64; 6usize],
}
impl pthread_cond_t {
    pub unsafe fn __data(&mut self) -> *mut Struct_Unnamed1 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 48usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_longlong {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for pthread_cond_t {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for pthread_cond_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_Unnamed1 {
    pub __lock: ::std::os::raw::c_int,
    pub __futex: ::std::os::raw::c_uint,
    pub __total_seq: ::std::os::raw::c_ulonglong,
    pub __wakeup_seq: ::std::os::raw::c_ulonglong,
    pub __woken_seq: ::std::os::raw::c_ulonglong,
    pub __mutex: *mut ::std::os::raw::c_void,
    pub __nwaiters: ::std::os::raw::c_uint,
    pub __broadcast_seq: ::std::os::raw::c_uint,
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct pthread_condattr_t {
    pub _bindgen_data_: [u32; 1usize],
}
impl pthread_condattr_t {
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for pthread_condattr_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct pthread_rwlock_t {
    pub _bindgen_data_: [u64; 7usize],
}
impl pthread_rwlock_t {
    pub unsafe fn __data(&mut self) -> *mut Struct_Unnamed2 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 56usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for pthread_rwlock_t {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for pthread_rwlock_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_Unnamed2 {
    pub __lock: ::std::os::raw::c_int,
    pub __nr_readers: ::std::os::raw::c_uint,
    pub __readers_wakeup: ::std::os::raw::c_uint,
    pub __writer_wakeup: ::std::os::raw::c_uint,
    pub __nr_readers_queued: ::std::os::raw::c_uint,
    pub __nr_writers_queued: ::std::os::raw::c_uint,
    pub __writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_char,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct pthread_rwlockattr_t {
    pub _bindgen_data_: [u64; 1usize],
}
impl pthread_rwlockattr_t {
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 8usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for pthread_rwlockattr_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct pthread_barrier_t {
    pub _bindgen_data_: [u64; 4usize],
}
impl pthread_barrier_t {
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 32usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for pthread_barrier_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct pthread_barrierattr_t {
    pub _bindgen_data_: [u32; 1usize],
}
impl pthread_barrierattr_t {
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for pthread_barrierattr_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type int_least8_t = ::std::os::raw::c_char;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_long;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulong;
pub type int_fast8_t = ::std::os::raw::c_char;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct iovec {
    pub iov_base: *mut ::std::os::raw::c_void,
    pub iov_len: size_t,
}
impl ::std::default::Default for iovec {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct sched_param {
    pub __sched_priority: ::std::os::raw::c_int,
}
impl ::std::default::Default for sched_param {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct __sched_param {
    pub __sched_priority: ::std::os::raw::c_int,
}
impl ::std::default::Default for __sched_param {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type __cpu_mask = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16usize],
}
impl ::std::default::Default for cpu_set_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct tm {
    pub tm_sec: ::std::os::raw::c_int,
    pub tm_min: ::std::os::raw::c_int,
    pub tm_hour: ::std::os::raw::c_int,
    pub tm_mday: ::std::os::raw::c_int,
    pub tm_mon: ::std::os::raw::c_int,
    pub tm_year: ::std::os::raw::c_int,
    pub tm_wday: ::std::os::raw::c_int,
    pub tm_yday: ::std::os::raw::c_int,
    pub tm_isdst: ::std::os::raw::c_int,
    pub tm_gmtoff: ::std::os::raw::c_long,
    pub tm_zone: *const ::std::os::raw::c_char,
}
impl ::std::default::Default for tm {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
impl ::std::default::Default for itimerspec {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub enum sigevent {
}
pub enum __locale_data {
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const ::std::os::raw::c_ushort,
    pub __ctype_tolower: *const ::std::os::raw::c_int,
    pub __ctype_toupper: *const ::std::os::raw::c_int,
    pub __names: [*const ::std::os::raw::c_char; 13usize],
}
impl ::std::default::Default for __locale_struct {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
pub type __jmp_buf = [::std::os::raw::c_long; 8usize];
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed3 {
    PTHREAD_CREATE_JOINABLE = 0,
    PTHREAD_CREATE_DETACHED = 1,
}
pub const PTHREAD_MUTEX_NORMAL: Enum_Unnamed4 = Enum_Unnamed4::PTHREAD_MUTEX_TIMED_NP;
pub const PTHREAD_MUTEX_RECURSIVE: Enum_Unnamed4 = Enum_Unnamed4::PTHREAD_MUTEX_RECURSIVE_NP;
pub const PTHREAD_MUTEX_ERRORCHECK: Enum_Unnamed4 = Enum_Unnamed4::PTHREAD_MUTEX_ERRORCHECK_NP;
pub const PTHREAD_MUTEX_DEFAULT: Enum_Unnamed4 = Enum_Unnamed4::PTHREAD_MUTEX_TIMED_NP;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed4 {
    PTHREAD_MUTEX_TIMED_NP = 0,
    PTHREAD_MUTEX_RECURSIVE_NP = 1,
    PTHREAD_MUTEX_ERRORCHECK_NP = 2,
    PTHREAD_MUTEX_ADAPTIVE_NP = 3,
}
pub const PTHREAD_MUTEX_STALLED_NP: Enum_Unnamed5 = Enum_Unnamed5::PTHREAD_MUTEX_STALLED;
pub const PTHREAD_MUTEX_ROBUST_NP: Enum_Unnamed5 = Enum_Unnamed5::PTHREAD_MUTEX_ROBUST;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed5 {
    PTHREAD_MUTEX_STALLED = 0,
    PTHREAD_MUTEX_ROBUST = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed6 {
    PTHREAD_PRIO_NONE = 0,
    PTHREAD_PRIO_INHERIT = 1,
    PTHREAD_PRIO_PROTECT = 2,
}
pub const PTHREAD_RWLOCK_DEFAULT_NP: Enum_Unnamed7 = Enum_Unnamed7::PTHREAD_RWLOCK_PREFER_READER_NP;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed7 {
    PTHREAD_RWLOCK_PREFER_READER_NP = 0,
    PTHREAD_RWLOCK_PREFER_WRITER_NP = 1,
    PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed8 {
    PTHREAD_INHERIT_SCHED = 0,
    PTHREAD_EXPLICIT_SCHED = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed9 {
    PTHREAD_SCOPE_SYSTEM = 0,
    PTHREAD_SCOPE_PROCESS = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed10 {
    PTHREAD_PROCESS_PRIVATE = 0,
    PTHREAD_PROCESS_SHARED = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct _pthread_cleanup_buffer {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __canceltype: ::std::os::raw::c_int,
    pub __prev: *mut _pthread_cleanup_buffer,
}
impl ::std::default::Default for _pthread_cleanup_buffer {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed11 {
    PTHREAD_CANCEL_ENABLE = 0,
    PTHREAD_CANCEL_DISABLE = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed12 {
    PTHREAD_CANCEL_DEFERRED = 0,
    PTHREAD_CANCEL_ASYNCHRONOUS = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [Struct_Unnamed13; 1usize],
    pub __pad: [*mut ::std::os::raw::c_void; 4usize],
}
impl ::std::default::Default for __pthread_unwind_buf_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_Unnamed13 {
    pub __cancel_jmp_buf: __jmp_buf,
    pub __mask_was_saved: ::std::os::raw::c_int,
}
impl ::std::default::Default for Struct_Unnamed13 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct __pthread_cleanup_frame {
    pub __cancel_routine:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __cancel_arg: *mut ::std::os::raw::c_void,
    pub __do_it: ::std::os::raw::c_int,
    pub __cancel_type: ::std::os::raw::c_int,
}
impl ::std::default::Default for __pthread_cleanup_frame {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub enum __jmp_buf_tag {
}
pub enum xcb_connection_t {
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_generic_iterator_t {
    pub data: *mut ::std::os::raw::c_void,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_generic_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_generic_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
}
impl ::std::default::Default for xcb_generic_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_generic_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub pad: [uint32_t; 7usize],
    pub full_sequence: uint32_t,
}
impl ::std::default::Default for xcb_generic_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_ge_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub event_type: uint16_t,
    pub pad1: uint16_t,
    pub pad: [uint32_t; 5usize],
    pub full_sequence: uint32_t,
}
impl ::std::default::Default for xcb_ge_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_generic_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub resource_id: uint32_t,
    pub minor_code: uint16_t,
    pub major_code: uint8_t,
    pub pad0: uint8_t,
    pub pad: [uint32_t; 5usize],
    pub full_sequence: uint32_t,
}
impl ::std::default::Default for xcb_generic_error_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_void_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_void_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_char2b_t {
    pub byte1: uint8_t,
    pub byte2: uint8_t,
}
impl ::std::default::Default for xcb_char2b_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_char2b_iterator_t {
    pub data: *mut xcb_char2b_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_char2b_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_window_t = uint32_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_window_iterator_t {
    pub data: *mut xcb_window_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_window_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_pixmap_t = uint32_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_pixmap_iterator_t {
    pub data: *mut xcb_pixmap_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_pixmap_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_cursor_t = uint32_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_cursor_iterator_t {
    pub data: *mut xcb_cursor_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_cursor_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_font_t = uint32_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_font_iterator_t {
    pub data: *mut xcb_font_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_font_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_gcontext_t = uint32_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_gcontext_iterator_t {
    pub data: *mut xcb_gcontext_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_gcontext_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_colormap_t = uint32_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_colormap_iterator_t {
    pub data: *mut xcb_colormap_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_colormap_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_atom_t = uint32_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_atom_iterator_t {
    pub data: *mut xcb_atom_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_atom_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_drawable_t = uint32_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_drawable_iterator_t {
    pub data: *mut xcb_drawable_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_drawable_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_fontable_t = uint32_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_fontable_iterator_t {
    pub data: *mut xcb_fontable_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_fontable_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_visualid_t = uint32_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_visualid_iterator_t {
    pub data: *mut xcb_visualid_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_visualid_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_timestamp_t = uint32_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_timestamp_iterator_t {
    pub data: *mut xcb_timestamp_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_timestamp_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_keysym_t = uint32_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_keysym_iterator_t {
    pub data: *mut xcb_keysym_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_keysym_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_keycode_t = uint8_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_keycode_iterator_t {
    pub data: *mut xcb_keycode_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_keycode_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_button_t = uint8_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_button_iterator_t {
    pub data: *mut xcb_button_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_button_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_point_t {
    pub x: int16_t,
    pub y: int16_t,
}
impl ::std::default::Default for xcb_point_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_point_iterator_t {
    pub data: *mut xcb_point_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_point_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_rectangle_t {
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
}
impl ::std::default::Default for xcb_rectangle_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_rectangle_iterator_t {
    pub data: *mut xcb_rectangle_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_rectangle_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_arc_t {
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub angle1: int16_t,
    pub angle2: int16_t,
}
impl ::std::default::Default for xcb_arc_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_arc_iterator_t {
    pub data: *mut xcb_arc_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_arc_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_format_t {
    pub depth: uint8_t,
    pub bits_per_pixel: uint8_t,
    pub scanline_pad: uint8_t,
    pub pad0: [uint8_t; 5usize],
}
impl ::std::default::Default for xcb_format_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_format_iterator_t {
    pub data: *mut xcb_format_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_format_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_visual_class_t {
    XCB_VISUAL_CLASS_STATIC_GRAY = 0,
    XCB_VISUAL_CLASS_GRAY_SCALE = 1,
    XCB_VISUAL_CLASS_STATIC_COLOR = 2,
    XCB_VISUAL_CLASS_PSEUDO_COLOR = 3,
    XCB_VISUAL_CLASS_TRUE_COLOR = 4,
    XCB_VISUAL_CLASS_DIRECT_COLOR = 5,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_visualtype_t {
    pub visual_id: xcb_visualid_t,
    pub _class: uint8_t,
    pub bits_per_rgb_value: uint8_t,
    pub colormap_entries: uint16_t,
    pub red_mask: uint32_t,
    pub green_mask: uint32_t,
    pub blue_mask: uint32_t,
    pub pad0: [uint8_t; 4usize],
}
impl ::std::default::Default for xcb_visualtype_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_visualtype_iterator_t {
    pub data: *mut xcb_visualtype_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_visualtype_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_depth_t {
    pub depth: uint8_t,
    pub pad0: uint8_t,
    pub visuals_len: uint16_t,
    pub pad1: [uint8_t; 4usize],
}
impl ::std::default::Default for xcb_depth_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_depth_iterator_t {
    pub data: *mut xcb_depth_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_depth_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_event_mask_t {
    XCB_EVENT_MASK_NO_EVENT = 0,
    XCB_EVENT_MASK_KEY_PRESS = 1,
    XCB_EVENT_MASK_KEY_RELEASE = 2,
    XCB_EVENT_MASK_BUTTON_PRESS = 4,
    XCB_EVENT_MASK_BUTTON_RELEASE = 8,
    XCB_EVENT_MASK_ENTER_WINDOW = 16,
    XCB_EVENT_MASK_LEAVE_WINDOW = 32,
    XCB_EVENT_MASK_POINTER_MOTION = 64,
    XCB_EVENT_MASK_POINTER_MOTION_HINT = 128,
    XCB_EVENT_MASK_BUTTON_1_MOTION = 256,
    XCB_EVENT_MASK_BUTTON_2_MOTION = 512,
    XCB_EVENT_MASK_BUTTON_3_MOTION = 1024,
    XCB_EVENT_MASK_BUTTON_4_MOTION = 2048,
    XCB_EVENT_MASK_BUTTON_5_MOTION = 4096,
    XCB_EVENT_MASK_BUTTON_MOTION = 8192,
    XCB_EVENT_MASK_KEYMAP_STATE = 16384,
    XCB_EVENT_MASK_EXPOSURE = 32768,
    XCB_EVENT_MASK_VISIBILITY_CHANGE = 65536,
    XCB_EVENT_MASK_STRUCTURE_NOTIFY = 131072,
    XCB_EVENT_MASK_RESIZE_REDIRECT = 262144,
    XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY = 524288,
    XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT = 1048576,
    XCB_EVENT_MASK_FOCUS_CHANGE = 2097152,
    XCB_EVENT_MASK_PROPERTY_CHANGE = 4194304,
    XCB_EVENT_MASK_COLOR_MAP_CHANGE = 8388608,
    XCB_EVENT_MASK_OWNER_GRAB_BUTTON = 16777216,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_backing_store_t {
    XCB_BACKING_STORE_NOT_USEFUL = 0,
    XCB_BACKING_STORE_WHEN_MAPPED = 1,
    XCB_BACKING_STORE_ALWAYS = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: uint32_t,
    pub black_pixel: uint32_t,
    pub current_input_masks: uint32_t,
    pub width_in_pixels: uint16_t,
    pub height_in_pixels: uint16_t,
    pub width_in_millimeters: uint16_t,
    pub height_in_millimeters: uint16_t,
    pub min_installed_maps: uint16_t,
    pub max_installed_maps: uint16_t,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: uint8_t,
    pub save_unders: uint8_t,
    pub root_depth: uint8_t,
    pub allowed_depths_len: uint8_t,
}
impl ::std::default::Default for xcb_screen_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_screen_iterator_t {
    pub data: *mut xcb_screen_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_screen_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_setup_request_t {
    pub byte_order: uint8_t,
    pub pad0: uint8_t,
    pub protocol_major_version: uint16_t,
    pub protocol_minor_version: uint16_t,
    pub authorization_protocol_name_len: uint16_t,
    pub authorization_protocol_data_len: uint16_t,
    pub pad1: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_setup_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_setup_request_iterator_t {
    pub data: *mut xcb_setup_request_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_setup_request_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_setup_failed_t {
    pub status: uint8_t,
    pub reason_len: uint8_t,
    pub protocol_major_version: uint16_t,
    pub protocol_minor_version: uint16_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_setup_failed_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_setup_failed_iterator_t {
    pub data: *mut xcb_setup_failed_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_setup_failed_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_setup_authenticate_t {
    pub status: uint8_t,
    pub pad0: [uint8_t; 5usize],
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_setup_authenticate_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_setup_authenticate_iterator_t {
    pub data: *mut xcb_setup_authenticate_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_setup_authenticate_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_image_order_t {
    XCB_IMAGE_ORDER_LSB_FIRST = 0,
    XCB_IMAGE_ORDER_MSB_FIRST = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_setup_t {
    pub status: uint8_t,
    pub pad0: uint8_t,
    pub protocol_major_version: uint16_t,
    pub protocol_minor_version: uint16_t,
    pub length: uint16_t,
    pub release_number: uint32_t,
    pub resource_id_base: uint32_t,
    pub resource_id_mask: uint32_t,
    pub motion_buffer_size: uint32_t,
    pub vendor_len: uint16_t,
    pub maximum_request_length: uint16_t,
    pub roots_len: uint8_t,
    pub pixmap_formats_len: uint8_t,
    pub image_byte_order: uint8_t,
    pub bitmap_format_bit_order: uint8_t,
    pub bitmap_format_scanline_unit: uint8_t,
    pub bitmap_format_scanline_pad: uint8_t,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [uint8_t; 4usize],
}
impl ::std::default::Default for xcb_setup_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_setup_iterator_t {
    pub data: *mut xcb_setup_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_setup_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_mod_mask_t {
    XCB_MOD_MASK_SHIFT = 1,
    XCB_MOD_MASK_LOCK = 2,
    XCB_MOD_MASK_CONTROL = 4,
    XCB_MOD_MASK_1 = 8,
    XCB_MOD_MASK_2 = 16,
    XCB_MOD_MASK_3 = 32,
    XCB_MOD_MASK_4 = 64,
    XCB_MOD_MASK_5 = 128,
    XCB_MOD_MASK_ANY = 32768,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_key_but_mask_t {
    XCB_KEY_BUT_MASK_SHIFT = 1,
    XCB_KEY_BUT_MASK_LOCK = 2,
    XCB_KEY_BUT_MASK_CONTROL = 4,
    XCB_KEY_BUT_MASK_MOD_1 = 8,
    XCB_KEY_BUT_MASK_MOD_2 = 16,
    XCB_KEY_BUT_MASK_MOD_3 = 32,
    XCB_KEY_BUT_MASK_MOD_4 = 64,
    XCB_KEY_BUT_MASK_MOD_5 = 128,
    XCB_KEY_BUT_MASK_BUTTON_1 = 256,
    XCB_KEY_BUT_MASK_BUTTON_2 = 512,
    XCB_KEY_BUT_MASK_BUTTON_3 = 1024,
    XCB_KEY_BUT_MASK_BUTTON_4 = 2048,
    XCB_KEY_BUT_MASK_BUTTON_5 = 4096,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_window_enum_t {
    XCB_WINDOW_NONE = 0,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_key_press_event_t {
    pub response_type: uint8_t,
    pub detail: xcb_keycode_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: int16_t,
    pub root_y: int16_t,
    pub event_x: int16_t,
    pub event_y: int16_t,
    pub state: uint16_t,
    pub same_screen: uint8_t,
    pub pad0: uint8_t,
}
impl ::std::default::Default for xcb_key_press_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_key_release_event_t = xcb_key_press_event_t;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_button_mask_t {
    XCB_BUTTON_MASK_1 = 256,
    XCB_BUTTON_MASK_2 = 512,
    XCB_BUTTON_MASK_3 = 1024,
    XCB_BUTTON_MASK_4 = 2048,
    XCB_BUTTON_MASK_5 = 4096,
    XCB_BUTTON_MASK_ANY = 32768,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_button_press_event_t {
    pub response_type: uint8_t,
    pub detail: xcb_button_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: int16_t,
    pub root_y: int16_t,
    pub event_x: int16_t,
    pub event_y: int16_t,
    pub state: uint16_t,
    pub same_screen: uint8_t,
    pub pad0: uint8_t,
}
impl ::std::default::Default for xcb_button_press_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_button_release_event_t = xcb_button_press_event_t;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_motion_t {
    XCB_MOTION_NORMAL = 0,
    XCB_MOTION_HINT = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_motion_notify_event_t {
    pub response_type: uint8_t,
    pub detail: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: int16_t,
    pub root_y: int16_t,
    pub event_x: int16_t,
    pub event_y: int16_t,
    pub state: uint16_t,
    pub same_screen: uint8_t,
    pub pad0: uint8_t,
}
impl ::std::default::Default for xcb_motion_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_notify_detail_t {
    XCB_NOTIFY_DETAIL_ANCESTOR = 0,
    XCB_NOTIFY_DETAIL_VIRTUAL = 1,
    XCB_NOTIFY_DETAIL_INFERIOR = 2,
    XCB_NOTIFY_DETAIL_NONLINEAR = 3,
    XCB_NOTIFY_DETAIL_NONLINEAR_VIRTUAL = 4,
    XCB_NOTIFY_DETAIL_POINTER = 5,
    XCB_NOTIFY_DETAIL_POINTER_ROOT = 6,
    XCB_NOTIFY_DETAIL_NONE = 7,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_notify_mode_t {
    XCB_NOTIFY_MODE_NORMAL = 0,
    XCB_NOTIFY_MODE_GRAB = 1,
    XCB_NOTIFY_MODE_UNGRAB = 2,
    XCB_NOTIFY_MODE_WHILE_GRABBED = 3,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_enter_notify_event_t {
    pub response_type: uint8_t,
    pub detail: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: int16_t,
    pub root_y: int16_t,
    pub event_x: int16_t,
    pub event_y: int16_t,
    pub state: uint16_t,
    pub mode: uint8_t,
    pub same_screen_focus: uint8_t,
}
impl ::std::default::Default for xcb_enter_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_leave_notify_event_t = xcb_enter_notify_event_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_focus_in_event_t {
    pub response_type: uint8_t,
    pub detail: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub mode: uint8_t,
    pub pad0: [uint8_t; 3usize],
}
impl ::std::default::Default for xcb_focus_in_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_focus_out_event_t = xcb_focus_in_event_t;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_keymap_notify_event_t {
    pub response_type: uint8_t,
    pub keys: [uint8_t; 31usize],
}
impl ::std::default::Default for xcb_keymap_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_expose_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub x: uint16_t,
    pub y: uint16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub count: uint16_t,
    pub pad1: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_expose_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_graphics_exposure_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub drawable: xcb_drawable_t,
    pub x: uint16_t,
    pub y: uint16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub minor_opcode: uint16_t,
    pub count: uint16_t,
    pub major_opcode: uint8_t,
    pub pad1: [uint8_t; 3usize],
}
impl ::std::default::Default for xcb_graphics_exposure_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_no_exposure_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub drawable: xcb_drawable_t,
    pub minor_opcode: uint16_t,
    pub major_opcode: uint8_t,
    pub pad1: uint8_t,
}
impl ::std::default::Default for xcb_no_exposure_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_visibility_t {
    XCB_VISIBILITY_UNOBSCURED = 0,
    XCB_VISIBILITY_PARTIALLY_OBSCURED = 1,
    XCB_VISIBILITY_FULLY_OBSCURED = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_visibility_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub state: uint8_t,
    pub pad1: [uint8_t; 3usize],
}
impl ::std::default::Default for xcb_visibility_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_create_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub override_redirect: uint8_t,
    pub pad1: uint8_t,
}
impl ::std::default::Default for xcb_create_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_destroy_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_destroy_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_unmap_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub from_configure: uint8_t,
    pub pad1: [uint8_t; 3usize],
}
impl ::std::default::Default for xcb_unmap_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_map_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub override_redirect: uint8_t,
    pub pad1: [uint8_t; 3usize],
}
impl ::std::default::Default for xcb_map_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_map_request_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_map_request_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_reparent_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub parent: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub override_redirect: uint8_t,
    pub pad1: [uint8_t; 3usize],
}
impl ::std::default::Default for xcb_reparent_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_configure_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub above_sibling: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub override_redirect: uint8_t,
    pub pad1: uint8_t,
}
impl ::std::default::Default for xcb_configure_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_configure_request_event_t {
    pub response_type: uint8_t,
    pub stack_mode: uint8_t,
    pub sequence: uint16_t,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub sibling: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub value_mask: uint16_t,
}
impl ::std::default::Default for xcb_configure_request_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_gravity_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
}
impl ::std::default::Default for xcb_gravity_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_resize_request_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub width: uint16_t,
    pub height: uint16_t,
}
impl ::std::default::Default for xcb_resize_request_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_place_t {
    XCB_PLACE_ON_TOP = 0,
    XCB_PLACE_ON_BOTTOM = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_circulate_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub pad1: [uint8_t; 4usize],
    pub place: uint8_t,
    pub pad2: [uint8_t; 3usize],
}
impl ::std::default::Default for xcb_circulate_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_circulate_request_event_t = xcb_circulate_notify_event_t;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_property_t {
    XCB_PROPERTY_NEW_VALUE = 0,
    XCB_PROPERTY_DELETE = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_property_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub atom: xcb_atom_t,
    pub time: xcb_timestamp_t,
    pub state: uint8_t,
    pub pad1: [uint8_t; 3usize],
}
impl ::std::default::Default for xcb_property_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_selection_clear_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
}
impl ::std::default::Default for xcb_selection_clear_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_time_t {
    XCB_TIME_CURRENT_TIME = 0,
}
pub const XCB_ATOM_ANY: xcb_atom_enum_t = xcb_atom_enum_t::XCB_ATOM_NONE;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_atom_enum_t {
    XCB_ATOM_NONE = 0,
    XCB_ATOM_PRIMARY = 1,
    XCB_ATOM_SECONDARY = 2,
    XCB_ATOM_ARC = 3,
    XCB_ATOM_ATOM = 4,
    XCB_ATOM_BITMAP = 5,
    XCB_ATOM_CARDINAL = 6,
    XCB_ATOM_COLORMAP = 7,
    XCB_ATOM_CURSOR = 8,
    XCB_ATOM_CUT_BUFFER0 = 9,
    XCB_ATOM_CUT_BUFFER1 = 10,
    XCB_ATOM_CUT_BUFFER2 = 11,
    XCB_ATOM_CUT_BUFFER3 = 12,
    XCB_ATOM_CUT_BUFFER4 = 13,
    XCB_ATOM_CUT_BUFFER5 = 14,
    XCB_ATOM_CUT_BUFFER6 = 15,
    XCB_ATOM_CUT_BUFFER7 = 16,
    XCB_ATOM_DRAWABLE = 17,
    XCB_ATOM_FONT = 18,
    XCB_ATOM_INTEGER = 19,
    XCB_ATOM_PIXMAP = 20,
    XCB_ATOM_POINT = 21,
    XCB_ATOM_RECTANGLE = 22,
    XCB_ATOM_RESOURCE_MANAGER = 23,
    XCB_ATOM_RGB_COLOR_MAP = 24,
    XCB_ATOM_RGB_BEST_MAP = 25,
    XCB_ATOM_RGB_BLUE_MAP = 26,
    XCB_ATOM_RGB_DEFAULT_MAP = 27,
    XCB_ATOM_RGB_GRAY_MAP = 28,
    XCB_ATOM_RGB_GREEN_MAP = 29,
    XCB_ATOM_RGB_RED_MAP = 30,
    XCB_ATOM_STRING = 31,
    XCB_ATOM_VISUALID = 32,
    XCB_ATOM_WINDOW = 33,
    XCB_ATOM_WM_COMMAND = 34,
    XCB_ATOM_WM_HINTS = 35,
    XCB_ATOM_WM_CLIENT_MACHINE = 36,
    XCB_ATOM_WM_ICON_NAME = 37,
    XCB_ATOM_WM_ICON_SIZE = 38,
    XCB_ATOM_WM_NAME = 39,
    XCB_ATOM_WM_NORMAL_HINTS = 40,
    XCB_ATOM_WM_SIZE_HINTS = 41,
    XCB_ATOM_WM_ZOOM_HINTS = 42,
    XCB_ATOM_MIN_SPACE = 43,
    XCB_ATOM_NORM_SPACE = 44,
    XCB_ATOM_MAX_SPACE = 45,
    XCB_ATOM_END_SPACE = 46,
    XCB_ATOM_SUPERSCRIPT_X = 47,
    XCB_ATOM_SUPERSCRIPT_Y = 48,
    XCB_ATOM_SUBSCRIPT_X = 49,
    XCB_ATOM_SUBSCRIPT_Y = 50,
    XCB_ATOM_UNDERLINE_POSITION = 51,
    XCB_ATOM_UNDERLINE_THICKNESS = 52,
    XCB_ATOM_STRIKEOUT_ASCENT = 53,
    XCB_ATOM_STRIKEOUT_DESCENT = 54,
    XCB_ATOM_ITALIC_ANGLE = 55,
    XCB_ATOM_X_HEIGHT = 56,
    XCB_ATOM_QUAD_WIDTH = 57,
    XCB_ATOM_WEIGHT = 58,
    XCB_ATOM_POINT_SIZE = 59,
    XCB_ATOM_RESOLUTION = 60,
    XCB_ATOM_COPYRIGHT = 61,
    XCB_ATOM_NOTICE = 62,
    XCB_ATOM_FONT_NAME = 63,
    XCB_ATOM_FAMILY_NAME = 64,
    XCB_ATOM_FULL_NAME = 65,
    XCB_ATOM_CAP_HEIGHT = 66,
    XCB_ATOM_WM_CLASS = 67,
    XCB_ATOM_WM_TRANSIENT_FOR = 68,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_selection_request_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub owner: xcb_window_t,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
}
impl ::std::default::Default for xcb_selection_request_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_selection_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
}
impl ::std::default::Default for xcb_selection_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_colormap_state_t {
    XCB_COLORMAP_STATE_UNINSTALLED = 0,
    XCB_COLORMAP_STATE_INSTALLED = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_colormap_enum_t {
    XCB_COLORMAP_NONE = 0,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_colormap_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub colormap: xcb_colormap_t,
    pub _new: uint8_t,
    pub state: uint8_t,
    pub pad1: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_colormap_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_client_message_data_t {
    pub data: [u32; 5usize],
}
impl ::std::default::Default for xcb_client_message_data_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_client_message_data_iterator_t {
    pub data: *mut xcb_client_message_data_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_client_message_data_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_client_message_event_t {
    pub response_type: uint8_t,
    pub format: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub type_: xcb_atom_t,
    pub data: xcb_client_message_data_t,
}
impl ::std::default::Default for xcb_client_message_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_mapping_t {
    XCB_MAPPING_MODIFIER = 0,
    XCB_MAPPING_KEYBOARD = 1,
    XCB_MAPPING_POINTER = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_mapping_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub request: uint8_t,
    pub first_keycode: xcb_keycode_t,
    pub count: uint8_t,
    pub pad1: uint8_t,
}
impl ::std::default::Default for xcb_mapping_notify_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_ge_generic_event_t {
    pub response_type: uint8_t,
    pub extension: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub event_type: uint16_t,
    pub pad0: [uint8_t; 22usize],
    pub full_sequence: uint32_t,
}
impl ::std::default::Default for xcb_ge_generic_event_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_request_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub bad_value: uint32_t,
    pub minor_opcode: uint16_t,
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
}
impl ::std::default::Default for xcb_request_error_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_value_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub bad_value: uint32_t,
    pub minor_opcode: uint16_t,
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
}
impl ::std::default::Default for xcb_value_error_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xcb_window_error_t = xcb_value_error_t;
pub type xcb_pixmap_error_t = xcb_value_error_t;
pub type xcb_atom_error_t = xcb_value_error_t;
pub type xcb_cursor_error_t = xcb_value_error_t;
pub type xcb_font_error_t = xcb_value_error_t;
pub type xcb_match_error_t = xcb_request_error_t;
pub type xcb_drawable_error_t = xcb_value_error_t;
pub type xcb_access_error_t = xcb_request_error_t;
pub type xcb_alloc_error_t = xcb_request_error_t;
pub type xcb_colormap_error_t = xcb_value_error_t;
pub type xcb_g_context_error_t = xcb_value_error_t;
pub type xcb_id_choice_error_t = xcb_value_error_t;
pub type xcb_name_error_t = xcb_request_error_t;
pub type xcb_length_error_t = xcb_request_error_t;
pub type xcb_implementation_error_t = xcb_request_error_t;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_window_class_t {
    XCB_WINDOW_CLASS_COPY_FROM_PARENT = 0,
    XCB_WINDOW_CLASS_INPUT_OUTPUT = 1,
    XCB_WINDOW_CLASS_INPUT_ONLY = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_cw_t {
    XCB_CW_BACK_PIXMAP = 1,
    XCB_CW_BACK_PIXEL = 2,
    XCB_CW_BORDER_PIXMAP = 4,
    XCB_CW_BORDER_PIXEL = 8,
    XCB_CW_BIT_GRAVITY = 16,
    XCB_CW_WIN_GRAVITY = 32,
    XCB_CW_BACKING_STORE = 64,
    XCB_CW_BACKING_PLANES = 128,
    XCB_CW_BACKING_PIXEL = 256,
    XCB_CW_OVERRIDE_REDIRECT = 512,
    XCB_CW_SAVE_UNDER = 1024,
    XCB_CW_EVENT_MASK = 2048,
    XCB_CW_DONT_PROPAGATE = 4096,
    XCB_CW_COLORMAP = 8192,
    XCB_CW_CURSOR = 16384,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_back_pixmap_t {
    XCB_BACK_PIXMAP_NONE = 0,
    XCB_BACK_PIXMAP_PARENT_RELATIVE = 1,
}
pub const XCB_GRAVITY_WIN_UNMAP: xcb_gravity_t = xcb_gravity_t::XCB_GRAVITY_BIT_FORGET;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_gravity_t {
    XCB_GRAVITY_BIT_FORGET = 0,
    XCB_GRAVITY_NORTH_WEST = 1,
    XCB_GRAVITY_NORTH = 2,
    XCB_GRAVITY_NORTH_EAST = 3,
    XCB_GRAVITY_WEST = 4,
    XCB_GRAVITY_CENTER = 5,
    XCB_GRAVITY_EAST = 6,
    XCB_GRAVITY_SOUTH_WEST = 7,
    XCB_GRAVITY_SOUTH = 8,
    XCB_GRAVITY_SOUTH_EAST = 9,
    XCB_GRAVITY_STATIC = 10,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_create_window_request_t {
    pub major_opcode: uint8_t,
    pub depth: uint8_t,
    pub length: uint16_t,
    pub wid: xcb_window_t,
    pub parent: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub _class: uint16_t,
    pub visual: xcb_visualid_t,
    pub value_mask: uint32_t,
}
impl ::std::default::Default for xcb_create_window_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_change_window_attributes_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
    pub value_mask: uint32_t,
}
impl ::std::default::Default for xcb_change_window_attributes_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_map_state_t {
    XCB_MAP_STATE_UNMAPPED = 0,
    XCB_MAP_STATE_UNVIEWABLE = 1,
    XCB_MAP_STATE_VIEWABLE = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_window_attributes_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_window_attributes_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_window_attributes_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_get_window_attributes_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_window_attributes_reply_t {
    pub response_type: uint8_t,
    pub backing_store: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub visual: xcb_visualid_t,
    pub _class: uint16_t,
    pub bit_gravity: uint8_t,
    pub win_gravity: uint8_t,
    pub backing_planes: uint32_t,
    pub backing_pixel: uint32_t,
    pub save_under: uint8_t,
    pub map_is_installed: uint8_t,
    pub map_state: uint8_t,
    pub override_redirect: uint8_t,
    pub colormap: xcb_colormap_t,
    pub all_event_masks: uint32_t,
    pub your_event_mask: uint32_t,
    pub do_not_propagate_mask: uint16_t,
    pub pad0: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_get_window_attributes_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_destroy_window_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_destroy_window_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_destroy_subwindows_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_destroy_subwindows_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_set_mode_t {
    XCB_SET_MODE_INSERT = 0,
    XCB_SET_MODE_DELETE = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_change_save_set_request_t {
    pub major_opcode: uint8_t,
    pub mode: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_change_save_set_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_reparent_window_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
    pub parent: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
}
impl ::std::default::Default for xcb_reparent_window_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_map_window_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_map_window_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_map_subwindows_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_map_subwindows_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_unmap_window_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_unmap_window_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_unmap_subwindows_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_unmap_subwindows_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_config_window_t {
    XCB_CONFIG_WINDOW_X = 1,
    XCB_CONFIG_WINDOW_Y = 2,
    XCB_CONFIG_WINDOW_WIDTH = 4,
    XCB_CONFIG_WINDOW_HEIGHT = 8,
    XCB_CONFIG_WINDOW_BORDER_WIDTH = 16,
    XCB_CONFIG_WINDOW_SIBLING = 32,
    XCB_CONFIG_WINDOW_STACK_MODE = 64,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_stack_mode_t {
    XCB_STACK_MODE_ABOVE = 0,
    XCB_STACK_MODE_BELOW = 1,
    XCB_STACK_MODE_TOP_IF = 2,
    XCB_STACK_MODE_BOTTOM_IF = 3,
    XCB_STACK_MODE_OPPOSITE = 4,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_configure_window_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
    pub value_mask: uint16_t,
    pub pad1: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_configure_window_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_circulate_t {
    XCB_CIRCULATE_RAISE_LOWEST = 0,
    XCB_CIRCULATE_LOWER_HIGHEST = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_circulate_window_request_t {
    pub major_opcode: uint8_t,
    pub direction: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_circulate_window_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_geometry_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_geometry_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_geometry_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
}
impl ::std::default::Default for xcb_get_geometry_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_geometry_reply_t {
    pub response_type: uint8_t,
    pub depth: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub pad0: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_get_geometry_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_tree_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_query_tree_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_tree_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_query_tree_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_tree_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub parent: xcb_window_t,
    pub children_len: uint16_t,
    pub pad1: [uint8_t; 14usize],
}
impl ::std::default::Default for xcb_query_tree_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_intern_atom_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_intern_atom_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_intern_atom_request_t {
    pub major_opcode: uint8_t,
    pub only_if_exists: uint8_t,
    pub length: uint16_t,
    pub name_len: uint16_t,
    pub pad0: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_intern_atom_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_intern_atom_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub atom: xcb_atom_t,
}
impl ::std::default::Default for xcb_intern_atom_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_atom_name_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_atom_name_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_atom_name_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub atom: xcb_atom_t,
}
impl ::std::default::Default for xcb_get_atom_name_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_atom_name_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub name_len: uint16_t,
    pub pad1: [uint8_t; 22usize],
}
impl ::std::default::Default for xcb_get_atom_name_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_prop_mode_t {
    XCB_PROP_MODE_REPLACE = 0,
    XCB_PROP_MODE_PREPEND = 1,
    XCB_PROP_MODE_APPEND = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_change_property_request_t {
    pub major_opcode: uint8_t,
    pub mode: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub format: uint8_t,
    pub pad0: [uint8_t; 3usize],
    pub data_len: uint32_t,
}
impl ::std::default::Default for xcb_change_property_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_delete_property_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
}
impl ::std::default::Default for xcb_delete_property_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_get_property_type_t {
    XCB_GET_PROPERTY_TYPE_ANY = 0,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_property_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_property_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_property_request_t {
    pub major_opcode: uint8_t,
    pub _delete: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub long_offset: uint32_t,
    pub long_length: uint32_t,
}
impl ::std::default::Default for xcb_get_property_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_property_reply_t {
    pub response_type: uint8_t,
    pub format: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub type_: xcb_atom_t,
    pub bytes_after: uint32_t,
    pub value_len: uint32_t,
    pub pad0: [uint8_t; 12usize],
}
impl ::std::default::Default for xcb_get_property_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_properties_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_list_properties_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_properties_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_list_properties_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_properties_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub atoms_len: uint16_t,
    pub pad1: [uint8_t; 22usize],
}
impl ::std::default::Default for xcb_list_properties_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_selection_owner_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
    pub time: xcb_timestamp_t,
}
impl ::std::default::Default for xcb_set_selection_owner_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_selection_owner_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_selection_owner_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_selection_owner_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub selection: xcb_atom_t,
}
impl ::std::default::Default for xcb_get_selection_owner_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_selection_owner_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub owner: xcb_window_t,
}
impl ::std::default::Default for xcb_get_selection_owner_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_convert_selection_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
    pub time: xcb_timestamp_t,
}
impl ::std::default::Default for xcb_convert_selection_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_send_event_dest_t {
    XCB_SEND_EVENT_DEST_POINTER_WINDOW = 0,
    XCB_SEND_EVENT_DEST_ITEM_FOCUS = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_send_event_request_t {
    pub major_opcode: uint8_t,
    pub propagate: uint8_t,
    pub length: uint16_t,
    pub destination: xcb_window_t,
    pub event_mask: uint32_t,
    pub event: [::std::os::raw::c_char; 32usize],
}
impl ::std::default::Default for xcb_send_event_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_grab_mode_t {
    XCB_GRAB_MODE_SYNC = 0,
    XCB_GRAB_MODE_ASYNC = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_grab_status_t {
    XCB_GRAB_STATUS_SUCCESS = 0,
    XCB_GRAB_STATUS_ALREADY_GRABBED = 1,
    XCB_GRAB_STATUS_INVALID_TIME = 2,
    XCB_GRAB_STATUS_NOT_VIEWABLE = 3,
    XCB_GRAB_STATUS_FROZEN = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_cursor_enum_t {
    XCB_CURSOR_NONE = 0,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_grab_pointer_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_grab_pointer_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_grab_pointer_request_t {
    pub major_opcode: uint8_t,
    pub owner_events: uint8_t,
    pub length: uint16_t,
    pub grab_window: xcb_window_t,
    pub event_mask: uint16_t,
    pub pointer_mode: uint8_t,
    pub keyboard_mode: uint8_t,
    pub confine_to: xcb_window_t,
    pub cursor: xcb_cursor_t,
    pub time: xcb_timestamp_t,
}
impl ::std::default::Default for xcb_grab_pointer_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_grab_pointer_reply_t {
    pub response_type: uint8_t,
    pub status: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
}
impl ::std::default::Default for xcb_grab_pointer_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_ungrab_pointer_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub time: xcb_timestamp_t,
}
impl ::std::default::Default for xcb_ungrab_pointer_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_button_index_t {
    XCB_BUTTON_INDEX_ANY = 0,
    XCB_BUTTON_INDEX_1 = 1,
    XCB_BUTTON_INDEX_2 = 2,
    XCB_BUTTON_INDEX_3 = 3,
    XCB_BUTTON_INDEX_4 = 4,
    XCB_BUTTON_INDEX_5 = 5,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_grab_button_request_t {
    pub major_opcode: uint8_t,
    pub owner_events: uint8_t,
    pub length: uint16_t,
    pub grab_window: xcb_window_t,
    pub event_mask: uint16_t,
    pub pointer_mode: uint8_t,
    pub keyboard_mode: uint8_t,
    pub confine_to: xcb_window_t,
    pub cursor: xcb_cursor_t,
    pub button: uint8_t,
    pub pad0: uint8_t,
    pub modifiers: uint16_t,
}
impl ::std::default::Default for xcb_grab_button_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_ungrab_button_request_t {
    pub major_opcode: uint8_t,
    pub button: uint8_t,
    pub length: uint16_t,
    pub grab_window: xcb_window_t,
    pub modifiers: uint16_t,
    pub pad0: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_ungrab_button_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_change_active_pointer_grab_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cursor: xcb_cursor_t,
    pub time: xcb_timestamp_t,
    pub event_mask: uint16_t,
    pub pad1: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_change_active_pointer_grab_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_grab_keyboard_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_grab_keyboard_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_grab_keyboard_request_t {
    pub major_opcode: uint8_t,
    pub owner_events: uint8_t,
    pub length: uint16_t,
    pub grab_window: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub pointer_mode: uint8_t,
    pub keyboard_mode: uint8_t,
    pub pad0: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_grab_keyboard_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_grab_keyboard_reply_t {
    pub response_type: uint8_t,
    pub status: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
}
impl ::std::default::Default for xcb_grab_keyboard_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_ungrab_keyboard_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub time: xcb_timestamp_t,
}
impl ::std::default::Default for xcb_ungrab_keyboard_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_grab_t {
    XCB_GRAB_ANY = 0,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_grab_key_request_t {
    pub major_opcode: uint8_t,
    pub owner_events: uint8_t,
    pub length: uint16_t,
    pub grab_window: xcb_window_t,
    pub modifiers: uint16_t,
    pub key: xcb_keycode_t,
    pub pointer_mode: uint8_t,
    pub keyboard_mode: uint8_t,
    pub pad0: [uint8_t; 3usize],
}
impl ::std::default::Default for xcb_grab_key_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_ungrab_key_request_t {
    pub major_opcode: uint8_t,
    pub key: xcb_keycode_t,
    pub length: uint16_t,
    pub grab_window: xcb_window_t,
    pub modifiers: uint16_t,
    pub pad0: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_ungrab_key_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_allow_t {
    XCB_ALLOW_ASYNC_POINTER = 0,
    XCB_ALLOW_SYNC_POINTER = 1,
    XCB_ALLOW_REPLAY_POINTER = 2,
    XCB_ALLOW_ASYNC_KEYBOARD = 3,
    XCB_ALLOW_SYNC_KEYBOARD = 4,
    XCB_ALLOW_REPLAY_KEYBOARD = 5,
    XCB_ALLOW_ASYNC_BOTH = 6,
    XCB_ALLOW_SYNC_BOTH = 7,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_allow_events_request_t {
    pub major_opcode: uint8_t,
    pub mode: uint8_t,
    pub length: uint16_t,
    pub time: xcb_timestamp_t,
}
impl ::std::default::Default for xcb_allow_events_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_grab_server_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_grab_server_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_ungrab_server_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_ungrab_server_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_pointer_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_query_pointer_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_pointer_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_query_pointer_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_pointer_reply_t {
    pub response_type: uint8_t,
    pub same_screen: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: int16_t,
    pub root_y: int16_t,
    pub win_x: int16_t,
    pub win_y: int16_t,
    pub mask: uint16_t,
    pub pad0: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_query_pointer_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_timecoord_t {
    pub time: xcb_timestamp_t,
    pub x: int16_t,
    pub y: int16_t,
}
impl ::std::default::Default for xcb_timecoord_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_timecoord_iterator_t {
    pub data: *mut xcb_timecoord_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_timecoord_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_motion_events_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_motion_events_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_motion_events_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
    pub start: xcb_timestamp_t,
    pub stop: xcb_timestamp_t,
}
impl ::std::default::Default for xcb_get_motion_events_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_motion_events_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub events_len: uint32_t,
    pub pad1: [uint8_t; 20usize],
}
impl ::std::default::Default for xcb_get_motion_events_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_translate_coordinates_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_translate_coordinates_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_translate_coordinates_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub src_window: xcb_window_t,
    pub dst_window: xcb_window_t,
    pub src_x: int16_t,
    pub src_y: int16_t,
}
impl ::std::default::Default for xcb_translate_coordinates_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_translate_coordinates_reply_t {
    pub response_type: uint8_t,
    pub same_screen: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub child: xcb_window_t,
    pub dst_x: int16_t,
    pub dst_y: int16_t,
}
impl ::std::default::Default for xcb_translate_coordinates_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_warp_pointer_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub src_window: xcb_window_t,
    pub dst_window: xcb_window_t,
    pub src_x: int16_t,
    pub src_y: int16_t,
    pub src_width: uint16_t,
    pub src_height: uint16_t,
    pub dst_x: int16_t,
    pub dst_y: int16_t,
}
impl ::std::default::Default for xcb_warp_pointer_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_input_focus_t {
    XCB_INPUT_FOCUS_NONE = 0,
    XCB_INPUT_FOCUS_POINTER_ROOT = 1,
    XCB_INPUT_FOCUS_PARENT = 2,
    XCB_INPUT_FOCUS_FOLLOW_KEYBOARD = 3,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_input_focus_request_t {
    pub major_opcode: uint8_t,
    pub revert_to: uint8_t,
    pub length: uint16_t,
    pub focus: xcb_window_t,
    pub time: xcb_timestamp_t,
}
impl ::std::default::Default for xcb_set_input_focus_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_input_focus_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_input_focus_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_input_focus_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_get_input_focus_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_input_focus_reply_t {
    pub response_type: uint8_t,
    pub revert_to: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub focus: xcb_window_t,
}
impl ::std::default::Default for xcb_get_input_focus_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_keymap_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_query_keymap_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_keymap_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_query_keymap_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_keymap_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub keys: [uint8_t; 32usize],
}
impl ::std::default::Default for xcb_query_keymap_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_open_font_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub fid: xcb_font_t,
    pub name_len: uint16_t,
    pub pad1: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_open_font_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_close_font_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub font: xcb_font_t,
}
impl ::std::default::Default for xcb_close_font_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_font_draw_t {
    XCB_FONT_DRAW_LEFT_TO_RIGHT = 0,
    XCB_FONT_DRAW_RIGHT_TO_LEFT = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_fontprop_t {
    pub name: xcb_atom_t,
    pub value: uint32_t,
}
impl ::std::default::Default for xcb_fontprop_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_fontprop_iterator_t {
    pub data: *mut xcb_fontprop_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_fontprop_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_charinfo_t {
    pub left_side_bearing: int16_t,
    pub right_side_bearing: int16_t,
    pub character_width: int16_t,
    pub ascent: int16_t,
    pub descent: int16_t,
    pub attributes: uint16_t,
}
impl ::std::default::Default for xcb_charinfo_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_charinfo_iterator_t {
    pub data: *mut xcb_charinfo_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_charinfo_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_font_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_query_font_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_font_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub font: xcb_fontable_t,
}
impl ::std::default::Default for xcb_query_font_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_font_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub min_bounds: xcb_charinfo_t,
    pub pad1: [uint8_t; 4usize],
    pub max_bounds: xcb_charinfo_t,
    pub pad2: [uint8_t; 4usize],
    pub min_char_or_byte2: uint16_t,
    pub max_char_or_byte2: uint16_t,
    pub default_char: uint16_t,
    pub properties_len: uint16_t,
    pub draw_direction: uint8_t,
    pub min_byte1: uint8_t,
    pub max_byte1: uint8_t,
    pub all_chars_exist: uint8_t,
    pub font_ascent: int16_t,
    pub font_descent: int16_t,
    pub char_infos_len: uint32_t,
}
impl ::std::default::Default for xcb_query_font_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_text_extents_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_query_text_extents_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_text_extents_request_t {
    pub major_opcode: uint8_t,
    pub odd_length: uint8_t,
    pub length: uint16_t,
    pub font: xcb_fontable_t,
}
impl ::std::default::Default for xcb_query_text_extents_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_text_extents_reply_t {
    pub response_type: uint8_t,
    pub draw_direction: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub font_ascent: int16_t,
    pub font_descent: int16_t,
    pub overall_ascent: int16_t,
    pub overall_descent: int16_t,
    pub overall_width: int32_t,
    pub overall_left: int32_t,
    pub overall_right: int32_t,
}
impl ::std::default::Default for xcb_query_text_extents_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_str_t {
    pub name_len: uint8_t,
}
impl ::std::default::Default for xcb_str_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_str_iterator_t {
    pub data: *mut xcb_str_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_str_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_fonts_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_list_fonts_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_fonts_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub max_names: uint16_t,
    pub pattern_len: uint16_t,
}
impl ::std::default::Default for xcb_list_fonts_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_fonts_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub names_len: uint16_t,
    pub pad1: [uint8_t; 22usize],
}
impl ::std::default::Default for xcb_list_fonts_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_fonts_with_info_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_list_fonts_with_info_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_fonts_with_info_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub max_names: uint16_t,
    pub pattern_len: uint16_t,
}
impl ::std::default::Default for xcb_list_fonts_with_info_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_fonts_with_info_reply_t {
    pub response_type: uint8_t,
    pub name_len: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub min_bounds: xcb_charinfo_t,
    pub pad0: [uint8_t; 4usize],
    pub max_bounds: xcb_charinfo_t,
    pub pad1: [uint8_t; 4usize],
    pub min_char_or_byte2: uint16_t,
    pub max_char_or_byte2: uint16_t,
    pub default_char: uint16_t,
    pub properties_len: uint16_t,
    pub draw_direction: uint8_t,
    pub min_byte1: uint8_t,
    pub max_byte1: uint8_t,
    pub all_chars_exist: uint8_t,
    pub font_ascent: int16_t,
    pub font_descent: int16_t,
    pub replies_hint: uint32_t,
}
impl ::std::default::Default for xcb_list_fonts_with_info_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_font_path_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub font_qty: uint16_t,
    pub pad1: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_set_font_path_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_font_path_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_font_path_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_font_path_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_get_font_path_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_font_path_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub path_len: uint16_t,
    pub pad1: [uint8_t; 22usize],
}
impl ::std::default::Default for xcb_get_font_path_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_create_pixmap_request_t {
    pub major_opcode: uint8_t,
    pub depth: uint8_t,
    pub length: uint16_t,
    pub pid: xcb_pixmap_t,
    pub drawable: xcb_drawable_t,
    pub width: uint16_t,
    pub height: uint16_t,
}
impl ::std::default::Default for xcb_create_pixmap_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_free_pixmap_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub pixmap: xcb_pixmap_t,
}
impl ::std::default::Default for xcb_free_pixmap_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_gc_t {
    XCB_GC_FUNCTION = 1,
    XCB_GC_PLANE_MASK = 2,
    XCB_GC_FOREGROUND = 4,
    XCB_GC_BACKGROUND = 8,
    XCB_GC_LINE_WIDTH = 16,
    XCB_GC_LINE_STYLE = 32,
    XCB_GC_CAP_STYLE = 64,
    XCB_GC_JOIN_STYLE = 128,
    XCB_GC_FILL_STYLE = 256,
    XCB_GC_FILL_RULE = 512,
    XCB_GC_TILE = 1024,
    XCB_GC_STIPPLE = 2048,
    XCB_GC_TILE_STIPPLE_ORIGIN_X = 4096,
    XCB_GC_TILE_STIPPLE_ORIGIN_Y = 8192,
    XCB_GC_FONT = 16384,
    XCB_GC_SUBWINDOW_MODE = 32768,
    XCB_GC_GRAPHICS_EXPOSURES = 65536,
    XCB_GC_CLIP_ORIGIN_X = 131072,
    XCB_GC_CLIP_ORIGIN_Y = 262144,
    XCB_GC_CLIP_MASK = 524288,
    XCB_GC_DASH_OFFSET = 1048576,
    XCB_GC_DASH_LIST = 2097152,
    XCB_GC_ARC_MODE = 4194304,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_gx_t {
    XCB_GX_CLEAR = 0,
    XCB_GX_AND = 1,
    XCB_GX_AND_REVERSE = 2,
    XCB_GX_COPY = 3,
    XCB_GX_AND_INVERTED = 4,
    XCB_GX_NOOP = 5,
    XCB_GX_XOR = 6,
    XCB_GX_OR = 7,
    XCB_GX_NOR = 8,
    XCB_GX_EQUIV = 9,
    XCB_GX_INVERT = 10,
    XCB_GX_OR_REVERSE = 11,
    XCB_GX_COPY_INVERTED = 12,
    XCB_GX_OR_INVERTED = 13,
    XCB_GX_NAND = 14,
    XCB_GX_SET = 15,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_line_style_t {
    XCB_LINE_STYLE_SOLID = 0,
    XCB_LINE_STYLE_ON_OFF_DASH = 1,
    XCB_LINE_STYLE_DOUBLE_DASH = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_cap_style_t {
    XCB_CAP_STYLE_NOT_LAST = 0,
    XCB_CAP_STYLE_BUTT = 1,
    XCB_CAP_STYLE_ROUND = 2,
    XCB_CAP_STYLE_PROJECTING = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_join_style_t {
    XCB_JOIN_STYLE_MITER = 0,
    XCB_JOIN_STYLE_ROUND = 1,
    XCB_JOIN_STYLE_BEVEL = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_fill_style_t {
    XCB_FILL_STYLE_SOLID = 0,
    XCB_FILL_STYLE_TILED = 1,
    XCB_FILL_STYLE_STIPPLED = 2,
    XCB_FILL_STYLE_OPAQUE_STIPPLED = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_fill_rule_t {
    XCB_FILL_RULE_EVEN_ODD = 0,
    XCB_FILL_RULE_WINDING = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_subwindow_mode_t {
    XCB_SUBWINDOW_MODE_CLIP_BY_CHILDREN = 0,
    XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_arc_mode_t {
    XCB_ARC_MODE_CHORD = 0,
    XCB_ARC_MODE_PIE_SLICE = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_create_gc_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cid: xcb_gcontext_t,
    pub drawable: xcb_drawable_t,
    pub value_mask: uint32_t,
}
impl ::std::default::Default for xcb_create_gc_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_change_gc_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub gc: xcb_gcontext_t,
    pub value_mask: uint32_t,
}
impl ::std::default::Default for xcb_change_gc_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_copy_gc_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub src_gc: xcb_gcontext_t,
    pub dst_gc: xcb_gcontext_t,
    pub value_mask: uint32_t,
}
impl ::std::default::Default for xcb_copy_gc_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_dashes_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub gc: xcb_gcontext_t,
    pub dash_offset: uint16_t,
    pub dashes_len: uint16_t,
}
impl ::std::default::Default for xcb_set_dashes_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_clip_ordering_t {
    XCB_CLIP_ORDERING_UNSORTED = 0,
    XCB_CLIP_ORDERING_Y_SORTED = 1,
    XCB_CLIP_ORDERING_YX_SORTED = 2,
    XCB_CLIP_ORDERING_YX_BANDED = 3,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_clip_rectangles_request_t {
    pub major_opcode: uint8_t,
    pub ordering: uint8_t,
    pub length: uint16_t,
    pub gc: xcb_gcontext_t,
    pub clip_x_origin: int16_t,
    pub clip_y_origin: int16_t,
}
impl ::std::default::Default for xcb_set_clip_rectangles_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_free_gc_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub gc: xcb_gcontext_t,
}
impl ::std::default::Default for xcb_free_gc_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_clear_area_request_t {
    pub major_opcode: uint8_t,
    pub exposures: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
}
impl ::std::default::Default for xcb_clear_area_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_copy_area_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub src_drawable: xcb_drawable_t,
    pub dst_drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub src_x: int16_t,
    pub src_y: int16_t,
    pub dst_x: int16_t,
    pub dst_y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
}
impl ::std::default::Default for xcb_copy_area_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_copy_plane_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub src_drawable: xcb_drawable_t,
    pub dst_drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub src_x: int16_t,
    pub src_y: int16_t,
    pub dst_x: int16_t,
    pub dst_y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub bit_plane: uint32_t,
}
impl ::std::default::Default for xcb_copy_plane_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_coord_mode_t {
    XCB_COORD_MODE_ORIGIN = 0,
    XCB_COORD_MODE_PREVIOUS = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_poly_point_request_t {
    pub major_opcode: uint8_t,
    pub coordinate_mode: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
impl ::std::default::Default for xcb_poly_point_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_poly_line_request_t {
    pub major_opcode: uint8_t,
    pub coordinate_mode: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
impl ::std::default::Default for xcb_poly_line_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_segment_t {
    pub x1: int16_t,
    pub y1: int16_t,
    pub x2: int16_t,
    pub y2: int16_t,
}
impl ::std::default::Default for xcb_segment_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_segment_iterator_t {
    pub data: *mut xcb_segment_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_segment_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_poly_segment_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
impl ::std::default::Default for xcb_poly_segment_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_poly_rectangle_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
impl ::std::default::Default for xcb_poly_rectangle_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_poly_arc_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
impl ::std::default::Default for xcb_poly_arc_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_poly_shape_t {
    XCB_POLY_SHAPE_COMPLEX = 0,
    XCB_POLY_SHAPE_NONCONVEX = 1,
    XCB_POLY_SHAPE_CONVEX = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_fill_poly_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub shape: uint8_t,
    pub coordinate_mode: uint8_t,
    pub pad1: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_fill_poly_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_poly_fill_rectangle_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
impl ::std::default::Default for xcb_poly_fill_rectangle_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_poly_fill_arc_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
impl ::std::default::Default for xcb_poly_fill_arc_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_image_format_t {
    XCB_IMAGE_FORMAT_XY_BITMAP = 0,
    XCB_IMAGE_FORMAT_XY_PIXMAP = 1,
    XCB_IMAGE_FORMAT_Z_PIXMAP = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_put_image_request_t {
    pub major_opcode: uint8_t,
    pub format: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub dst_x: int16_t,
    pub dst_y: int16_t,
    pub left_pad: uint8_t,
    pub depth: uint8_t,
    pub pad0: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_put_image_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_image_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_image_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_image_request_t {
    pub major_opcode: uint8_t,
    pub format: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub plane_mask: uint32_t,
}
impl ::std::default::Default for xcb_get_image_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_image_reply_t {
    pub response_type: uint8_t,
    pub depth: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub visual: xcb_visualid_t,
    pub pad0: [uint8_t; 20usize],
}
impl ::std::default::Default for xcb_get_image_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_poly_text_8_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: int16_t,
    pub y: int16_t,
}
impl ::std::default::Default for xcb_poly_text_8_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_poly_text_16_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: int16_t,
    pub y: int16_t,
}
impl ::std::default::Default for xcb_poly_text_16_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_image_text_8_request_t {
    pub major_opcode: uint8_t,
    pub string_len: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: int16_t,
    pub y: int16_t,
}
impl ::std::default::Default for xcb_image_text_8_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_image_text_16_request_t {
    pub major_opcode: uint8_t,
    pub string_len: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: int16_t,
    pub y: int16_t,
}
impl ::std::default::Default for xcb_image_text_16_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_colormap_alloc_t {
    XCB_COLORMAP_ALLOC_NONE = 0,
    XCB_COLORMAP_ALLOC_ALL = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_create_colormap_request_t {
    pub major_opcode: uint8_t,
    pub alloc: uint8_t,
    pub length: uint16_t,
    pub mid: xcb_colormap_t,
    pub window: xcb_window_t,
    pub visual: xcb_visualid_t,
}
impl ::std::default::Default for xcb_create_colormap_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_free_colormap_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cmap: xcb_colormap_t,
}
impl ::std::default::Default for xcb_free_colormap_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_copy_colormap_and_free_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub mid: xcb_colormap_t,
    pub src_cmap: xcb_colormap_t,
}
impl ::std::default::Default for xcb_copy_colormap_and_free_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_install_colormap_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cmap: xcb_colormap_t,
}
impl ::std::default::Default for xcb_install_colormap_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_uninstall_colormap_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cmap: xcb_colormap_t,
}
impl ::std::default::Default for xcb_uninstall_colormap_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_installed_colormaps_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_list_installed_colormaps_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_installed_colormaps_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for xcb_list_installed_colormaps_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_installed_colormaps_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub cmaps_len: uint16_t,
    pub pad1: [uint8_t; 22usize],
}
impl ::std::default::Default for xcb_list_installed_colormaps_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_alloc_color_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_alloc_color_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_alloc_color_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cmap: xcb_colormap_t,
    pub red: uint16_t,
    pub green: uint16_t,
    pub blue: uint16_t,
    pub pad1: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_alloc_color_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_alloc_color_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub red: uint16_t,
    pub green: uint16_t,
    pub blue: uint16_t,
    pub pad1: [uint8_t; 2usize],
    pub pixel: uint32_t,
}
impl ::std::default::Default for xcb_alloc_color_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_alloc_named_color_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_alloc_named_color_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_alloc_named_color_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cmap: xcb_colormap_t,
    pub name_len: uint16_t,
    pub pad1: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_alloc_named_color_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_alloc_named_color_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub pixel: uint32_t,
    pub exact_red: uint16_t,
    pub exact_green: uint16_t,
    pub exact_blue: uint16_t,
    pub visual_red: uint16_t,
    pub visual_green: uint16_t,
    pub visual_blue: uint16_t,
}
impl ::std::default::Default for xcb_alloc_named_color_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_alloc_color_cells_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_alloc_color_cells_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_alloc_color_cells_request_t {
    pub major_opcode: uint8_t,
    pub contiguous: uint8_t,
    pub length: uint16_t,
    pub cmap: xcb_colormap_t,
    pub colors: uint16_t,
    pub planes: uint16_t,
}
impl ::std::default::Default for xcb_alloc_color_cells_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_alloc_color_cells_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub pixels_len: uint16_t,
    pub masks_len: uint16_t,
    pub pad1: [uint8_t; 20usize],
}
impl ::std::default::Default for xcb_alloc_color_cells_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_alloc_color_planes_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_alloc_color_planes_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_alloc_color_planes_request_t {
    pub major_opcode: uint8_t,
    pub contiguous: uint8_t,
    pub length: uint16_t,
    pub cmap: xcb_colormap_t,
    pub colors: uint16_t,
    pub reds: uint16_t,
    pub greens: uint16_t,
    pub blues: uint16_t,
}
impl ::std::default::Default for xcb_alloc_color_planes_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_alloc_color_planes_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub pixels_len: uint16_t,
    pub pad1: [uint8_t; 2usize],
    pub red_mask: uint32_t,
    pub green_mask: uint32_t,
    pub blue_mask: uint32_t,
    pub pad2: [uint8_t; 8usize],
}
impl ::std::default::Default for xcb_alloc_color_planes_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_free_colors_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cmap: xcb_colormap_t,
    pub plane_mask: uint32_t,
}
impl ::std::default::Default for xcb_free_colors_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_color_flag_t {
    XCB_COLOR_FLAG_RED = 1,
    XCB_COLOR_FLAG_GREEN = 2,
    XCB_COLOR_FLAG_BLUE = 4,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_coloritem_t {
    pub pixel: uint32_t,
    pub red: uint16_t,
    pub green: uint16_t,
    pub blue: uint16_t,
    pub flags: uint8_t,
    pub pad0: uint8_t,
}
impl ::std::default::Default for xcb_coloritem_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_coloritem_iterator_t {
    pub data: *mut xcb_coloritem_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_coloritem_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_store_colors_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cmap: xcb_colormap_t,
}
impl ::std::default::Default for xcb_store_colors_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_store_named_color_request_t {
    pub major_opcode: uint8_t,
    pub flags: uint8_t,
    pub length: uint16_t,
    pub cmap: xcb_colormap_t,
    pub pixel: uint32_t,
    pub name_len: uint16_t,
    pub pad0: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_store_named_color_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_rgb_t {
    pub red: uint16_t,
    pub green: uint16_t,
    pub blue: uint16_t,
    pub pad0: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_rgb_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_rgb_iterator_t {
    pub data: *mut xcb_rgb_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_rgb_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_colors_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_query_colors_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_colors_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cmap: xcb_colormap_t,
}
impl ::std::default::Default for xcb_query_colors_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_colors_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub colors_len: uint16_t,
    pub pad1: [uint8_t; 22usize],
}
impl ::std::default::Default for xcb_query_colors_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_lookup_color_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_lookup_color_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_lookup_color_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cmap: xcb_colormap_t,
    pub name_len: uint16_t,
    pub pad1: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_lookup_color_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_lookup_color_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub exact_red: uint16_t,
    pub exact_green: uint16_t,
    pub exact_blue: uint16_t,
    pub visual_red: uint16_t,
    pub visual_green: uint16_t,
    pub visual_blue: uint16_t,
}
impl ::std::default::Default for xcb_lookup_color_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_pixmap_enum_t {
    XCB_PIXMAP_NONE = 0,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_create_cursor_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cid: xcb_cursor_t,
    pub source: xcb_pixmap_t,
    pub mask: xcb_pixmap_t,
    pub fore_red: uint16_t,
    pub fore_green: uint16_t,
    pub fore_blue: uint16_t,
    pub back_red: uint16_t,
    pub back_green: uint16_t,
    pub back_blue: uint16_t,
    pub x: uint16_t,
    pub y: uint16_t,
}
impl ::std::default::Default for xcb_create_cursor_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_font_enum_t {
    XCB_FONT_NONE = 0,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_create_glyph_cursor_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cid: xcb_cursor_t,
    pub source_font: xcb_font_t,
    pub mask_font: xcb_font_t,
    pub source_char: uint16_t,
    pub mask_char: uint16_t,
    pub fore_red: uint16_t,
    pub fore_green: uint16_t,
    pub fore_blue: uint16_t,
    pub back_red: uint16_t,
    pub back_green: uint16_t,
    pub back_blue: uint16_t,
}
impl ::std::default::Default for xcb_create_glyph_cursor_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_free_cursor_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cursor: xcb_cursor_t,
}
impl ::std::default::Default for xcb_free_cursor_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_recolor_cursor_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub cursor: xcb_cursor_t,
    pub fore_red: uint16_t,
    pub fore_green: uint16_t,
    pub fore_blue: uint16_t,
    pub back_red: uint16_t,
    pub back_green: uint16_t,
    pub back_blue: uint16_t,
}
impl ::std::default::Default for xcb_recolor_cursor_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_query_shape_of_t {
    XCB_QUERY_SHAPE_OF_LARGEST_CURSOR = 0,
    XCB_QUERY_SHAPE_OF_FASTEST_TILE = 1,
    XCB_QUERY_SHAPE_OF_FASTEST_STIPPLE = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_best_size_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_query_best_size_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_best_size_request_t {
    pub major_opcode: uint8_t,
    pub _class: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub width: uint16_t,
    pub height: uint16_t,
}
impl ::std::default::Default for xcb_query_best_size_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_best_size_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub width: uint16_t,
    pub height: uint16_t,
}
impl ::std::default::Default for xcb_query_best_size_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_extension_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_query_extension_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_extension_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub name_len: uint16_t,
    pub pad1: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_query_extension_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_query_extension_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub present: uint8_t,
    pub major_opcode: uint8_t,
    pub first_event: uint8_t,
    pub first_error: uint8_t,
}
impl ::std::default::Default for xcb_query_extension_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_extensions_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_list_extensions_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_extensions_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_list_extensions_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_extensions_reply_t {
    pub response_type: uint8_t,
    pub names_len: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub pad0: [uint8_t; 24usize],
}
impl ::std::default::Default for xcb_list_extensions_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_change_keyboard_mapping_request_t {
    pub major_opcode: uint8_t,
    pub keycode_count: uint8_t,
    pub length: uint16_t,
    pub first_keycode: xcb_keycode_t,
    pub keysyms_per_keycode: uint8_t,
    pub pad0: [uint8_t; 2usize],
}
impl ::std::default::Default for xcb_change_keyboard_mapping_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_keyboard_mapping_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_keyboard_mapping_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_keyboard_mapping_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub first_keycode: xcb_keycode_t,
    pub count: uint8_t,
}
impl ::std::default::Default for xcb_get_keyboard_mapping_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_keyboard_mapping_reply_t {
    pub response_type: uint8_t,
    pub keysyms_per_keycode: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub pad0: [uint8_t; 24usize],
}
impl ::std::default::Default for xcb_get_keyboard_mapping_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_kb_t {
    XCB_KB_KEY_CLICK_PERCENT = 1,
    XCB_KB_BELL_PERCENT = 2,
    XCB_KB_BELL_PITCH = 4,
    XCB_KB_BELL_DURATION = 8,
    XCB_KB_LED = 16,
    XCB_KB_LED_MODE = 32,
    XCB_KB_KEY = 64,
    XCB_KB_AUTO_REPEAT_MODE = 128,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_led_mode_t {
    XCB_LED_MODE_OFF = 0,
    XCB_LED_MODE_ON = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_auto_repeat_mode_t {
    XCB_AUTO_REPEAT_MODE_OFF = 0,
    XCB_AUTO_REPEAT_MODE_ON = 1,
    XCB_AUTO_REPEAT_MODE_DEFAULT = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_change_keyboard_control_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub value_mask: uint32_t,
}
impl ::std::default::Default for xcb_change_keyboard_control_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_keyboard_control_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_keyboard_control_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_keyboard_control_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_get_keyboard_control_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_keyboard_control_reply_t {
    pub response_type: uint8_t,
    pub global_auto_repeat: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub led_mask: uint32_t,
    pub key_click_percent: uint8_t,
    pub bell_percent: uint8_t,
    pub bell_pitch: uint16_t,
    pub bell_duration: uint16_t,
    pub pad0: [uint8_t; 2usize],
    pub auto_repeats: [uint8_t; 32usize],
}
impl ::std::default::Default for xcb_get_keyboard_control_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_bell_request_t {
    pub major_opcode: uint8_t,
    pub percent: int8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_bell_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_change_pointer_control_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub acceleration_numerator: int16_t,
    pub acceleration_denominator: int16_t,
    pub threshold: int16_t,
    pub do_acceleration: uint8_t,
    pub do_threshold: uint8_t,
}
impl ::std::default::Default for xcb_change_pointer_control_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_pointer_control_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_pointer_control_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_pointer_control_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_get_pointer_control_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_pointer_control_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub acceleration_numerator: uint16_t,
    pub acceleration_denominator: uint16_t,
    pub threshold: uint16_t,
    pub pad1: [uint8_t; 18usize],
}
impl ::std::default::Default for xcb_get_pointer_control_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_blanking_t {
    XCB_BLANKING_NOT_PREFERRED = 0,
    XCB_BLANKING_PREFERRED = 1,
    XCB_BLANKING_DEFAULT = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_exposures_t {
    XCB_EXPOSURES_NOT_ALLOWED = 0,
    XCB_EXPOSURES_ALLOWED = 1,
    XCB_EXPOSURES_DEFAULT = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_screen_saver_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub timeout: int16_t,
    pub interval: int16_t,
    pub prefer_blanking: uint8_t,
    pub allow_exposures: uint8_t,
}
impl ::std::default::Default for xcb_set_screen_saver_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_screen_saver_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_screen_saver_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_screen_saver_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_get_screen_saver_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_screen_saver_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub timeout: uint16_t,
    pub interval: uint16_t,
    pub prefer_blanking: uint8_t,
    pub allow_exposures: uint8_t,
    pub pad1: [uint8_t; 18usize],
}
impl ::std::default::Default for xcb_get_screen_saver_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_host_mode_t {
    XCB_HOST_MODE_INSERT = 0,
    XCB_HOST_MODE_DELETE = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_family_t {
    XCB_FAMILY_INTERNET = 0,
    XCB_FAMILY_DECNET = 1,
    XCB_FAMILY_CHAOS = 2,
    XCB_FAMILY_SERVER_INTERPRETED = 5,
    XCB_FAMILY_INTERNET_6 = 6,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_change_hosts_request_t {
    pub major_opcode: uint8_t,
    pub mode: uint8_t,
    pub length: uint16_t,
    pub family: uint8_t,
    pub pad0: uint8_t,
    pub address_len: uint16_t,
}
impl ::std::default::Default for xcb_change_hosts_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_host_t {
    pub family: uint8_t,
    pub pad0: uint8_t,
    pub address_len: uint16_t,
}
impl ::std::default::Default for xcb_host_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_host_iterator_t {
    pub data: *mut xcb_host_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
impl ::std::default::Default for xcb_host_iterator_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_hosts_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_list_hosts_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_hosts_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_list_hosts_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_list_hosts_reply_t {
    pub response_type: uint8_t,
    pub mode: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub hosts_len: uint16_t,
    pub pad0: [uint8_t; 22usize],
}
impl ::std::default::Default for xcb_list_hosts_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_access_control_t {
    XCB_ACCESS_CONTROL_DISABLE = 0,
    XCB_ACCESS_CONTROL_ENABLE = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_access_control_request_t {
    pub major_opcode: uint8_t,
    pub mode: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_set_access_control_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_close_down_t {
    XCB_CLOSE_DOWN_DESTROY_ALL = 0,
    XCB_CLOSE_DOWN_RETAIN_PERMANENT = 1,
    XCB_CLOSE_DOWN_RETAIN_TEMPORARY = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_close_down_mode_request_t {
    pub major_opcode: uint8_t,
    pub mode: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_set_close_down_mode_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_kill_t {
    XCB_KILL_ALL_TEMPORARY = 0,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_kill_client_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub resource: uint32_t,
}
impl ::std::default::Default for xcb_kill_client_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_rotate_properties_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
    pub window: xcb_window_t,
    pub atoms_len: uint16_t,
    pub delta: int16_t,
}
impl ::std::default::Default for xcb_rotate_properties_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_screen_saver_t {
    XCB_SCREEN_SAVER_RESET = 0,
    XCB_SCREEN_SAVER_ACTIVE = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_force_screen_saver_request_t {
    pub major_opcode: uint8_t,
    pub mode: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_force_screen_saver_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_mapping_status_t {
    XCB_MAPPING_STATUS_SUCCESS = 0,
    XCB_MAPPING_STATUS_BUSY = 1,
    XCB_MAPPING_STATUS_FAILURE = 2,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_pointer_mapping_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_set_pointer_mapping_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_pointer_mapping_request_t {
    pub major_opcode: uint8_t,
    pub map_len: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_set_pointer_mapping_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_pointer_mapping_reply_t {
    pub response_type: uint8_t,
    pub status: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
}
impl ::std::default::Default for xcb_set_pointer_mapping_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_pointer_mapping_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_pointer_mapping_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_pointer_mapping_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_get_pointer_mapping_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_pointer_mapping_reply_t {
    pub response_type: uint8_t,
    pub map_len: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub pad0: [uint8_t; 24usize],
}
impl ::std::default::Default for xcb_get_pointer_mapping_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xcb_map_index_t {
    XCB_MAP_INDEX_SHIFT = 0,
    XCB_MAP_INDEX_LOCK = 1,
    XCB_MAP_INDEX_CONTROL = 2,
    XCB_MAP_INDEX_1 = 3,
    XCB_MAP_INDEX_2 = 4,
    XCB_MAP_INDEX_3 = 5,
    XCB_MAP_INDEX_4 = 6,
    XCB_MAP_INDEX_5 = 7,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_modifier_mapping_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_set_modifier_mapping_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_modifier_mapping_request_t {
    pub major_opcode: uint8_t,
    pub keycodes_per_modifier: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_set_modifier_mapping_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_set_modifier_mapping_reply_t {
    pub response_type: uint8_t,
    pub status: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
}
impl ::std::default::Default for xcb_set_modifier_mapping_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_modifier_mapping_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
impl ::std::default::Default for xcb_get_modifier_mapping_cookie_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_modifier_mapping_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_get_modifier_mapping_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_get_modifier_mapping_reply_t {
    pub response_type: uint8_t,
    pub keycodes_per_modifier: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub pad0: [uint8_t; 24usize],
}
impl ::std::default::Default for xcb_get_modifier_mapping_reply_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_no_operation_request_t {
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
    pub length: uint16_t,
}
impl ::std::default::Default for xcb_no_operation_request_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct xcb_auth_info_t {
    pub namelen: ::std::os::raw::c_int,
    pub name: *mut ::std::os::raw::c_char,
    pub datalen: ::std::os::raw::c_int,
    pub data: *mut ::std::os::raw::c_char,
}
impl ::std::default::Default for xcb_auth_info_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub enum xcb_special_event {
}
pub type xcb_special_event_t = xcb_special_event;
pub enum xcb_extension_t {
}
extern "C" {
    pub static mut __tzname: [*mut ::std::os::raw::c_char; 2usize];
    pub static mut __daylight: ::std::os::raw::c_int;
    pub static mut __timezone: ::std::os::raw::c_long;
    pub static mut tzname: [*mut ::std::os::raw::c_char; 2usize];
    pub static mut daylight: ::std::os::raw::c_int;
    pub static mut timezone: ::std::os::raw::c_long;
}
#[link(name = "xcb", kind = "dylib")]
extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
    pub fn gnu_dev_major(__dev: ::std::os::raw::c_ulonglong) -> ::std::os::raw::c_uint;
    pub fn gnu_dev_minor(__dev: ::std::os::raw::c_ulonglong) -> ::std::os::raw::c_uint;
    pub fn gnu_dev_makedev(
        __major: ::std::os::raw::c_uint,
        __minor: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_ulonglong;
    pub fn readv(
        __fd: ::std::os::raw::c_int,
        __iovec: *const iovec,
        __count: ::std::os::raw::c_int,
    ) -> ssize_t;
    pub fn writev(
        __fd: ::std::os::raw::c_int,
        __iovec: *const iovec,
        __count: ::std::os::raw::c_int,
    ) -> ssize_t;
    pub fn preadv(
        __fd: ::std::os::raw::c_int,
        __iovec: *const iovec,
        __count: ::std::os::raw::c_int,
        __offset: __off_t,
    ) -> ssize_t;
    pub fn pwritev(
        __fd: ::std::os::raw::c_int,
        __iovec: *const iovec,
        __count: ::std::os::raw::c_int,
        __offset: __off_t,
    ) -> ssize_t;
    pub fn __sched_cpucount(__setsize: size_t, __setp: *const cpu_set_t) -> ::std::os::raw::c_int;
    pub fn __sched_cpualloc(__count: size_t) -> *mut cpu_set_t;
    pub fn __sched_cpufree(__set: *mut cpu_set_t);
    pub fn sched_setparam(__pid: __pid_t, __param: *const sched_param) -> ::std::os::raw::c_int;
    pub fn sched_getparam(__pid: __pid_t, __param: *mut sched_param) -> ::std::os::raw::c_int;
    pub fn sched_setscheduler(
        __pid: __pid_t,
        __policy: ::std::os::raw::c_int,
        __param: *const sched_param,
    ) -> ::std::os::raw::c_int;
    pub fn sched_getscheduler(__pid: __pid_t) -> ::std::os::raw::c_int;
    pub fn sched_yield() -> ::std::os::raw::c_int;
    pub fn sched_get_priority_max(__algorithm: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn sched_get_priority_min(__algorithm: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn sched_rr_get_interval(__pid: __pid_t, __t: *mut timespec) -> ::std::os::raw::c_int;
    pub fn clock() -> clock_t;
    pub fn time(__timer: *mut time_t) -> time_t;
    pub fn difftime(__time1: time_t, __time0: time_t) -> f64;
    pub fn mktime(__tp: *mut tm) -> time_t;
    pub fn strftime(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: size_t,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
    ) -> size_t;
    pub fn strftime_l(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: size_t,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
        __loc: __locale_t,
    ) -> size_t;
    pub fn gmtime(__timer: *const time_t) -> *mut tm;
    pub fn localtime(__timer: *const time_t) -> *mut tm;
    pub fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    pub fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    pub fn asctime(__tp: *const tm) -> *mut ::std::os::raw::c_char;
    pub fn ctime(__timer: *const time_t) -> *mut ::std::os::raw::c_char;
    pub fn asctime_r(
        __tp: *const tm,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn ctime_r(
        __timer: *const time_t,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn tzset();
    pub fn stime(__when: *const time_t) -> ::std::os::raw::c_int;
    pub fn timegm(__tp: *mut tm) -> time_t;
    pub fn timelocal(__tp: *mut tm) -> time_t;
    pub fn dysize(__year: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::std::os::raw::c_int;
    pub fn clock_getres(__clock_id: clockid_t, __res: *mut timespec) -> ::std::os::raw::c_int;
    pub fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::std::os::raw::c_int;
    pub fn clock_settime(__clock_id: clockid_t, __tp: *const timespec) -> ::std::os::raw::c_int;
    pub fn clock_nanosleep(
        __clock_id: clockid_t,
        __flags: ::std::os::raw::c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> ::std::os::raw::c_int;
    pub fn clock_getcpuclockid(__pid: pid_t, __clock_id: *mut clockid_t) -> ::std::os::raw::c_int;
    pub fn timer_create(
        __clock_id: clockid_t,
        __evp: *mut sigevent,
        __timerid: *mut timer_t,
    ) -> ::std::os::raw::c_int;
    pub fn timer_delete(__timerid: timer_t) -> ::std::os::raw::c_int;
    pub fn timer_settime(
        __timerid: timer_t,
        __flags: ::std::os::raw::c_int,
        __value: *const itimerspec,
        __ovalue: *mut itimerspec,
    ) -> ::std::os::raw::c_int;
    pub fn timer_gettime(__timerid: timer_t, __value: *mut itimerspec) -> ::std::os::raw::c_int;
    pub fn timer_getoverrun(__timerid: timer_t) -> ::std::os::raw::c_int;
    pub fn timespec_get(
        __ts: *mut timespec,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_exit(__retval: *mut ::std::os::raw::c_void);
    pub fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_detach(__th: pthread_t) -> ::std::os::raw::c_int;
    pub fn pthread_self() -> pthread_t;
    pub fn pthread_equal(__thread1: pthread_t, __thread2: pthread_t) -> ::std::os::raw::c_int;
    pub fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::std::os::raw::c_int;
    pub fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::std::os::raw::c_int;
    pub fn pthread_attr_getdetachstate(
        __attr: *const pthread_attr_t,
        __detachstate: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_getguardsize(
        __attr: *const pthread_attr_t,
        __guardsize: *mut size_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_setguardsize(
        __attr: *mut pthread_attr_t,
        __guardsize: size_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_getschedparam(
        __attr: *const pthread_attr_t,
        __param: *mut sched_param,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_setschedparam(
        __attr: *mut pthread_attr_t,
        __param: *const sched_param,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_getschedpolicy(
        __attr: *const pthread_attr_t,
        __policy: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_setschedpolicy(
        __attr: *mut pthread_attr_t,
        __policy: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_getinheritsched(
        __attr: *const pthread_attr_t,
        __inherit: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_setinheritsched(
        __attr: *mut pthread_attr_t,
        __inherit: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_getscope(
        __attr: *const pthread_attr_t,
        __scope: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_setscope(
        __attr: *mut pthread_attr_t,
        __scope: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_getstackaddr(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_setstackaddr(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_getstacksize(
        __attr: *const pthread_attr_t,
        __stacksize: *mut size_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_getstack(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut *mut ::std::os::raw::c_void,
        __stacksize: *mut size_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_attr_setstack(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut ::std::os::raw::c_void,
        __stacksize: size_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_setschedparam(
        __target_thread: pthread_t,
        __policy: ::std::os::raw::c_int,
        __param: *const sched_param,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_getschedparam(
        __target_thread: pthread_t,
        __policy: *mut ::std::os::raw::c_int,
        __param: *mut sched_param,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_setschedprio(
        __target_thread: pthread_t,
        __prio: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: ::std::option::Option<extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_setcancelstate(
        __state: ::std::os::raw::c_int,
        __oldstate: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_setcanceltype(
        __type: ::std::os::raw::c_int,
        __oldtype: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_cancel(__th: pthread_t) -> ::std::os::raw::c_int;
    pub fn pthread_testcancel();
    pub fn __pthread_register_cancel(__buf: *mut __pthread_unwind_buf_t);
    pub fn __pthread_unregister_cancel(__buf: *mut __pthread_unwind_buf_t);
    pub fn __pthread_unwind_next(__buf: *mut __pthread_unwind_buf_t);
    pub fn __sigsetjmp(
        __env: *mut __jmp_buf_tag,
        __savemask: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
    pub fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
    pub fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
    pub fn pthread_mutex_timedlock(
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
    pub fn pthread_mutex_getprioceiling(
        __mutex: *const pthread_mutex_t,
        __prioceiling: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutex_setprioceiling(
        __mutex: *mut pthread_mutex_t,
        __prioceiling: ::std::os::raw::c_int,
        __old_ceiling: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutex_consistent(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
    pub fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> ::std::os::raw::c_int;
    pub fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> ::std::os::raw::c_int;
    pub fn pthread_mutexattr_getpshared(
        __attr: *const pthread_mutexattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutexattr_setpshared(
        __attr: *mut pthread_mutexattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutexattr_gettype(
        __attr: *const pthread_mutexattr_t,
        __kind: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutexattr_getprotocol(
        __attr: *const pthread_mutexattr_t,
        __protocol: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutexattr_setprotocol(
        __attr: *mut pthread_mutexattr_t,
        __protocol: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutexattr_getprioceiling(
        __attr: *const pthread_mutexattr_t,
        __prioceiling: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutexattr_setprioceiling(
        __attr: *mut pthread_mutexattr_t,
        __prioceiling: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutexattr_getrobust(
        __attr: *const pthread_mutexattr_t,
        __robustness: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_mutexattr_setrobust(
        __attr: *mut pthread_mutexattr_t,
        __robustness: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_rwlock_destroy(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
    pub fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
    pub fn pthread_rwlock_tryrdlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
    pub fn pthread_rwlock_timedrdlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
    pub fn pthread_rwlock_trywrlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
    pub fn pthread_rwlock_timedwrlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
    pub fn pthread_rwlockattr_init(__attr: *mut pthread_rwlockattr_t) -> ::std::os::raw::c_int;
    pub fn pthread_rwlockattr_destroy(__attr: *mut pthread_rwlockattr_t) -> ::std::os::raw::c_int;
    pub fn pthread_rwlockattr_getpshared(
        __attr: *const pthread_rwlockattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_rwlockattr_setpshared(
        __attr: *mut pthread_rwlockattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_rwlockattr_getkind_np(
        __attr: *const pthread_rwlockattr_t,
        __pref: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_rwlockattr_setkind_np(
        __attr: *mut pthread_rwlockattr_t,
        __pref: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::std::os::raw::c_int;
    pub fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::std::os::raw::c_int;
    pub fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::std::os::raw::c_int;
    pub fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_condattr_init(__attr: *mut pthread_condattr_t) -> ::std::os::raw::c_int;
    pub fn pthread_condattr_destroy(__attr: *mut pthread_condattr_t) -> ::std::os::raw::c_int;
    pub fn pthread_condattr_getpshared(
        __attr: *const pthread_condattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_condattr_setpshared(
        __attr: *mut pthread_condattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_condattr_getclock(
        __attr: *const pthread_condattr_t,
        __clock_id: *mut __clockid_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_condattr_setclock(
        __attr: *mut pthread_condattr_t,
        __clock_id: __clockid_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_spin_destroy(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
    pub fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
    pub fn pthread_spin_trylock(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
    pub fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
    pub fn pthread_barrier_init(
        __barrier: *mut pthread_barrier_t,
        __attr: *const pthread_barrierattr_t,
        __count: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_barrier_destroy(__barrier: *mut pthread_barrier_t) -> ::std::os::raw::c_int;
    pub fn pthread_barrier_wait(__barrier: *mut pthread_barrier_t) -> ::std::os::raw::c_int;
    pub fn pthread_barrierattr_init(__attr: *mut pthread_barrierattr_t) -> ::std::os::raw::c_int;
    pub fn pthread_barrierattr_destroy(__attr: *mut pthread_barrierattr_t)
        -> ::std::os::raw::c_int;
    pub fn pthread_barrierattr_getpshared(
        __attr: *const pthread_barrierattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_barrierattr_setpshared(
        __attr: *mut pthread_barrierattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
        >,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_key_delete(__key: pthread_key_t) -> ::std::os::raw::c_int;
    pub fn pthread_getspecific(__key: pthread_key_t) -> *mut ::std::os::raw::c_void;
    pub fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_getcpuclockid(
        __thread_id: pthread_t,
        __clock_id: *mut __clockid_t,
    ) -> ::std::os::raw::c_int;
    pub fn pthread_atfork(
        __prepare: ::std::option::Option<extern "C" fn()>,
        __parent: ::std::option::Option<extern "C" fn()>,
        __child: ::std::option::Option<extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_char2b_next(i: *mut xcb_char2b_iterator_t);
    pub fn xcb_char2b_end(i: xcb_char2b_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_window_next(i: *mut xcb_window_iterator_t);
    pub fn xcb_window_end(i: xcb_window_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_pixmap_next(i: *mut xcb_pixmap_iterator_t);
    pub fn xcb_pixmap_end(i: xcb_pixmap_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_cursor_next(i: *mut xcb_cursor_iterator_t);
    pub fn xcb_cursor_end(i: xcb_cursor_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_font_next(i: *mut xcb_font_iterator_t);
    pub fn xcb_font_end(i: xcb_font_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_gcontext_next(i: *mut xcb_gcontext_iterator_t);
    pub fn xcb_gcontext_end(i: xcb_gcontext_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_colormap_next(i: *mut xcb_colormap_iterator_t);
    pub fn xcb_colormap_end(i: xcb_colormap_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_atom_next(i: *mut xcb_atom_iterator_t);
    pub fn xcb_atom_end(i: xcb_atom_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_drawable_next(i: *mut xcb_drawable_iterator_t);
    pub fn xcb_drawable_end(i: xcb_drawable_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_fontable_next(i: *mut xcb_fontable_iterator_t);
    pub fn xcb_fontable_end(i: xcb_fontable_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_visualid_next(i: *mut xcb_visualid_iterator_t);
    pub fn xcb_visualid_end(i: xcb_visualid_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_timestamp_next(i: *mut xcb_timestamp_iterator_t);
    pub fn xcb_timestamp_end(i: xcb_timestamp_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_keysym_next(i: *mut xcb_keysym_iterator_t);
    pub fn xcb_keysym_end(i: xcb_keysym_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_keycode_next(i: *mut xcb_keycode_iterator_t);
    pub fn xcb_keycode_end(i: xcb_keycode_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_button_next(i: *mut xcb_button_iterator_t);
    pub fn xcb_button_end(i: xcb_button_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_point_next(i: *mut xcb_point_iterator_t);
    pub fn xcb_point_end(i: xcb_point_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_rectangle_next(i: *mut xcb_rectangle_iterator_t);
    pub fn xcb_rectangle_end(i: xcb_rectangle_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_arc_next(i: *mut xcb_arc_iterator_t);
    pub fn xcb_arc_end(i: xcb_arc_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_format_next(i: *mut xcb_format_iterator_t);
    pub fn xcb_format_end(i: xcb_format_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_visualtype_next(i: *mut xcb_visualtype_iterator_t);
    pub fn xcb_visualtype_end(i: xcb_visualtype_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_depth_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_depth_visuals(R: *const xcb_depth_t) -> *mut xcb_visualtype_t;
    pub fn xcb_depth_visuals_length(R: *const xcb_depth_t) -> ::std::os::raw::c_int;
    pub fn xcb_depth_visuals_iterator(R: *const xcb_depth_t) -> xcb_visualtype_iterator_t;
    pub fn xcb_depth_next(i: *mut xcb_depth_iterator_t);
    pub fn xcb_depth_end(i: xcb_depth_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_screen_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_screen_allowed_depths_length(R: *const xcb_screen_t) -> ::std::os::raw::c_int;
    pub fn xcb_screen_allowed_depths_iterator(R: *const xcb_screen_t) -> xcb_depth_iterator_t;
    pub fn xcb_screen_next(i: *mut xcb_screen_iterator_t);
    pub fn xcb_screen_end(i: xcb_screen_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_setup_request_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_setup_request_authorization_protocol_name(
        R: *const xcb_setup_request_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn xcb_setup_request_authorization_protocol_name_length(
        R: *const xcb_setup_request_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_setup_request_authorization_protocol_name_end(
        R: *const xcb_setup_request_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_setup_request_authorization_protocol_data(
        R: *const xcb_setup_request_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn xcb_setup_request_authorization_protocol_data_length(
        R: *const xcb_setup_request_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_setup_request_authorization_protocol_data_end(
        R: *const xcb_setup_request_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_setup_request_next(i: *mut xcb_setup_request_iterator_t);
    pub fn xcb_setup_request_end(i: xcb_setup_request_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_setup_failed_sizeof(_buffer: *const ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
    pub fn xcb_setup_failed_reason(R: *const xcb_setup_failed_t) -> *mut ::std::os::raw::c_char;
    pub fn xcb_setup_failed_reason_length(R: *const xcb_setup_failed_t) -> ::std::os::raw::c_int;
    pub fn xcb_setup_failed_reason_end(R: *const xcb_setup_failed_t) -> xcb_generic_iterator_t;
    pub fn xcb_setup_failed_next(i: *mut xcb_setup_failed_iterator_t);
    pub fn xcb_setup_failed_end(i: xcb_setup_failed_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_setup_authenticate_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_setup_authenticate_reason(
        R: *const xcb_setup_authenticate_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn xcb_setup_authenticate_reason_length(
        R: *const xcb_setup_authenticate_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_setup_authenticate_reason_end(
        R: *const xcb_setup_authenticate_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_setup_authenticate_next(i: *mut xcb_setup_authenticate_iterator_t);
    pub fn xcb_setup_authenticate_end(
        i: xcb_setup_authenticate_iterator_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_setup_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_setup_vendor(R: *const xcb_setup_t) -> *mut ::std::os::raw::c_char;
    pub fn xcb_setup_vendor_length(R: *const xcb_setup_t) -> ::std::os::raw::c_int;
    pub fn xcb_setup_vendor_end(R: *const xcb_setup_t) -> xcb_generic_iterator_t;
    pub fn xcb_setup_pixmap_formats(R: *const xcb_setup_t) -> *mut xcb_format_t;
    pub fn xcb_setup_pixmap_formats_length(R: *const xcb_setup_t) -> ::std::os::raw::c_int;
    pub fn xcb_setup_pixmap_formats_iterator(R: *const xcb_setup_t) -> xcb_format_iterator_t;
    pub fn xcb_setup_roots_length(R: *const xcb_setup_t) -> ::std::os::raw::c_int;
    pub fn xcb_setup_roots_iterator(R: *const xcb_setup_t) -> xcb_screen_iterator_t;
    pub fn xcb_setup_next(i: *mut xcb_setup_iterator_t);
    pub fn xcb_setup_end(i: xcb_setup_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_client_message_data_next(i: *mut xcb_client_message_data_iterator_t);
    pub fn xcb_client_message_data_end(
        i: xcb_client_message_data_iterator_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_create_window_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_create_window_checked(
        c: *mut xcb_connection_t,
        depth: uint8_t,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
        border_width: uint16_t,
        _class: uint16_t,
        visual: xcb_visualid_t,
        value_mask: uint32_t,
        value_list: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_window(
        c: *mut xcb_connection_t,
        depth: uint8_t,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
        border_width: uint16_t,
        _class: uint16_t,
        visual: xcb_visualid_t,
        value_mask: uint32_t,
        value_list: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_window_attributes_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_change_window_attributes_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint32_t,
        value_list: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint32_t,
        value_list: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_get_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t;
    pub fn xcb_get_window_attributes_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t;
    pub fn xcb_get_window_attributes_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_window_attributes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_window_attributes_reply_t;
    pub fn xcb_destroy_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    pub fn xcb_destroy_subwindows_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_destroy_subwindows(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_save_set_checked(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_save_set(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_reparent_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_reparent_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_map_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    pub fn xcb_map_subwindows_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_map_subwindows(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    pub fn xcb_unmap_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_unmap_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    pub fn xcb_unmap_subwindows_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_unmap_subwindows(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_configure_window_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_configure_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint16_t,
        value_list: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_configure_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint16_t,
        value_list: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_circulate_window_checked(
        c: *mut xcb_connection_t,
        direction: uint8_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_circulate_window(
        c: *mut xcb_connection_t,
        direction: uint8_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_get_geometry(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t;
    pub fn xcb_get_geometry_unchecked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t;
    pub fn xcb_get_geometry_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_geometry_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_geometry_reply_t;
    pub fn xcb_query_tree_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_query_tree(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_tree_cookie_t;
    pub fn xcb_query_tree_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_tree_cookie_t;
    pub fn xcb_query_tree_children(R: *const xcb_query_tree_reply_t) -> *mut xcb_window_t;
    pub fn xcb_query_tree_children_length(
        R: *const xcb_query_tree_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_query_tree_children_end(R: *const xcb_query_tree_reply_t) -> xcb_generic_iterator_t;
    pub fn xcb_query_tree_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_tree_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_tree_reply_t;
    pub fn xcb_intern_atom_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_intern_atom(
        c: *mut xcb_connection_t,
        only_if_exists: uint8_t,
        name_len: uint16_t,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_intern_atom_cookie_t;
    pub fn xcb_intern_atom_unchecked(
        c: *mut xcb_connection_t,
        only_if_exists: uint8_t,
        name_len: uint16_t,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_intern_atom_cookie_t;
    pub fn xcb_intern_atom_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_intern_atom_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_intern_atom_reply_t;
    pub fn xcb_get_atom_name_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_atom_name(
        c: *mut xcb_connection_t,
        atom: xcb_atom_t,
    ) -> xcb_get_atom_name_cookie_t;
    pub fn xcb_get_atom_name_unchecked(
        c: *mut xcb_connection_t,
        atom: xcb_atom_t,
    ) -> xcb_get_atom_name_cookie_t;
    pub fn xcb_get_atom_name_name(
        R: *const xcb_get_atom_name_reply_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn xcb_get_atom_name_name_length(
        R: *const xcb_get_atom_name_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_atom_name_name_end(
        R: *const xcb_get_atom_name_reply_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_get_atom_name_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_atom_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_atom_name_reply_t;
    pub fn xcb_change_property_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_change_property_checked(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: uint8_t,
        data_len: uint32_t,
        data: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_property(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: uint8_t,
        data_len: uint32_t,
        data: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_delete_property_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_delete_property(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_get_property_sizeof(_buffer: *const ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
    pub fn xcb_get_property(
        c: *mut xcb_connection_t,
        _delete: uint8_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: uint32_t,
        long_length: uint32_t,
    ) -> xcb_get_property_cookie_t;
    pub fn xcb_get_property_unchecked(
        c: *mut xcb_connection_t,
        _delete: uint8_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: uint32_t,
        long_length: uint32_t,
    ) -> xcb_get_property_cookie_t;
    pub fn xcb_get_property_value(
        R: *const xcb_get_property_reply_t,
    ) -> *mut ::std::os::raw::c_void;
    pub fn xcb_get_property_value_length(
        R: *const xcb_get_property_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_property_value_end(R: *const xcb_get_property_reply_t)
        -> xcb_generic_iterator_t;
    pub fn xcb_get_property_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_property_reply_t;
    pub fn xcb_list_properties_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_list_properties(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_properties_cookie_t;
    pub fn xcb_list_properties_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_properties_cookie_t;
    pub fn xcb_list_properties_atoms(R: *const xcb_list_properties_reply_t) -> *mut xcb_atom_t;
    pub fn xcb_list_properties_atoms_length(
        R: *const xcb_list_properties_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_list_properties_atoms_end(
        R: *const xcb_list_properties_reply_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_list_properties_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_list_properties_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_properties_reply_t;
    pub fn xcb_set_selection_owner_checked(
        c: *mut xcb_connection_t,
        owner: xcb_window_t,
        selection: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_selection_owner(
        c: *mut xcb_connection_t,
        owner: xcb_window_t,
        selection: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_get_selection_owner(
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_get_selection_owner_cookie_t;
    pub fn xcb_get_selection_owner_unchecked(
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_get_selection_owner_cookie_t;
    pub fn xcb_get_selection_owner_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_selection_owner_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_selection_owner_reply_t;
    pub fn xcb_convert_selection_checked(
        c: *mut xcb_connection_t,
        requestor: xcb_window_t,
        selection: xcb_atom_t,
        target: xcb_atom_t,
        property: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_convert_selection(
        c: *mut xcb_connection_t,
        requestor: xcb_window_t,
        selection: xcb_atom_t,
        target: xcb_atom_t,
        property: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_send_event_checked(
        c: *mut xcb_connection_t,
        propagate: uint8_t,
        destination: xcb_window_t,
        event_mask: uint32_t,
        event: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
    pub fn xcb_send_event(
        c: *mut xcb_connection_t,
        propagate: uint8_t,
        destination: xcb_window_t,
        event_mask: uint32_t,
        event: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
    pub fn xcb_grab_pointer(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        event_mask: uint16_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
    ) -> xcb_grab_pointer_cookie_t;
    pub fn xcb_grab_pointer_unchecked(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        event_mask: uint16_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
    ) -> xcb_grab_pointer_cookie_t;
    pub fn xcb_grab_pointer_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_grab_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_grab_pointer_reply_t;
    pub fn xcb_ungrab_pointer_checked(
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_ungrab_pointer(c: *mut xcb_connection_t, time: xcb_timestamp_t)
        -> xcb_void_cookie_t;
    pub fn xcb_grab_button_checked(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        event_mask: uint16_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        button: uint8_t,
        modifiers: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_grab_button(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        event_mask: uint16_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        button: uint8_t,
        modifiers: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_ungrab_button_checked(
        c: *mut xcb_connection_t,
        button: uint8_t,
        grab_window: xcb_window_t,
        modifiers: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_ungrab_button(
        c: *mut xcb_connection_t,
        button: uint8_t,
        grab_window: xcb_window_t,
        modifiers: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_active_pointer_grab_checked(
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
        event_mask: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_active_pointer_grab(
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
        event_mask: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_grab_keyboard(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
    ) -> xcb_grab_keyboard_cookie_t;
    pub fn xcb_grab_keyboard_unchecked(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
    ) -> xcb_grab_keyboard_cookie_t;
    pub fn xcb_grab_keyboard_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_grab_keyboard_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_grab_keyboard_reply_t;
    pub fn xcb_ungrab_keyboard_checked(
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_ungrab_keyboard(
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_grab_key_checked(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        modifiers: uint16_t,
        key: xcb_keycode_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_grab_key(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        modifiers: uint16_t,
        key: xcb_keycode_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_ungrab_key_checked(
        c: *mut xcb_connection_t,
        key: xcb_keycode_t,
        grab_window: xcb_window_t,
        modifiers: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_ungrab_key(
        c: *mut xcb_connection_t,
        key: xcb_keycode_t,
        grab_window: xcb_window_t,
        modifiers: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_allow_events_checked(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_allow_events(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_grab_server_checked(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    pub fn xcb_grab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    pub fn xcb_ungrab_server_checked(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    pub fn xcb_ungrab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    pub fn xcb_query_pointer(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_pointer_cookie_t;
    pub fn xcb_query_pointer_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_pointer_cookie_t;
    pub fn xcb_query_pointer_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_pointer_reply_t;
    pub fn xcb_timecoord_next(i: *mut xcb_timecoord_iterator_t);
    pub fn xcb_timecoord_end(i: xcb_timecoord_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_get_motion_events_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_motion_events(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        start: xcb_timestamp_t,
        stop: xcb_timestamp_t,
    ) -> xcb_get_motion_events_cookie_t;
    pub fn xcb_get_motion_events_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        start: xcb_timestamp_t,
        stop: xcb_timestamp_t,
    ) -> xcb_get_motion_events_cookie_t;
    pub fn xcb_get_motion_events_events(
        R: *const xcb_get_motion_events_reply_t,
    ) -> *mut xcb_timecoord_t;
    pub fn xcb_get_motion_events_events_length(
        R: *const xcb_get_motion_events_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_motion_events_events_iterator(
        R: *const xcb_get_motion_events_reply_t,
    ) -> xcb_timecoord_iterator_t;
    pub fn xcb_get_motion_events_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_motion_events_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_motion_events_reply_t;
    pub fn xcb_translate_coordinates(
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: int16_t,
        src_y: int16_t,
    ) -> xcb_translate_coordinates_cookie_t;
    pub fn xcb_translate_coordinates_unchecked(
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: int16_t,
        src_y: int16_t,
    ) -> xcb_translate_coordinates_cookie_t;
    pub fn xcb_translate_coordinates_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_translate_coordinates_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_translate_coordinates_reply_t;
    pub fn xcb_warp_pointer_checked(
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: int16_t,
        src_y: int16_t,
        src_width: uint16_t,
        src_height: uint16_t,
        dst_x: int16_t,
        dst_y: int16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_warp_pointer(
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: int16_t,
        src_y: int16_t,
        src_width: uint16_t,
        src_height: uint16_t,
        dst_x: int16_t,
        dst_y: int16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_input_focus_checked(
        c: *mut xcb_connection_t,
        revert_to: uint8_t,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_input_focus(
        c: *mut xcb_connection_t,
        revert_to: uint8_t,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_get_input_focus(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t;
    pub fn xcb_get_input_focus_unchecked(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t;
    pub fn xcb_get_input_focus_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_input_focus_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_input_focus_reply_t;
    pub fn xcb_query_keymap(c: *mut xcb_connection_t) -> xcb_query_keymap_cookie_t;
    pub fn xcb_query_keymap_unchecked(c: *mut xcb_connection_t) -> xcb_query_keymap_cookie_t;
    pub fn xcb_query_keymap_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_keymap_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_keymap_reply_t;
    pub fn xcb_open_font_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_open_font_checked(
        c: *mut xcb_connection_t,
        fid: xcb_font_t,
        name_len: uint16_t,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
    pub fn xcb_open_font(
        c: *mut xcb_connection_t,
        fid: xcb_font_t,
        name_len: uint16_t,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
    pub fn xcb_close_font_checked(c: *mut xcb_connection_t, font: xcb_font_t) -> xcb_void_cookie_t;
    pub fn xcb_close_font(c: *mut xcb_connection_t, font: xcb_font_t) -> xcb_void_cookie_t;
    pub fn xcb_fontprop_next(i: *mut xcb_fontprop_iterator_t);
    pub fn xcb_fontprop_end(i: xcb_fontprop_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_charinfo_next(i: *mut xcb_charinfo_iterator_t);
    pub fn xcb_charinfo_end(i: xcb_charinfo_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_query_font_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_query_font(
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
    ) -> xcb_query_font_cookie_t;
    pub fn xcb_query_font_unchecked(
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
    ) -> xcb_query_font_cookie_t;
    pub fn xcb_query_font_properties(R: *const xcb_query_font_reply_t) -> *mut xcb_fontprop_t;
    pub fn xcb_query_font_properties_length(
        R: *const xcb_query_font_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_query_font_properties_iterator(
        R: *const xcb_query_font_reply_t,
    ) -> xcb_fontprop_iterator_t;
    pub fn xcb_query_font_char_infos(R: *const xcb_query_font_reply_t) -> *mut xcb_charinfo_t;
    pub fn xcb_query_font_char_infos_length(
        R: *const xcb_query_font_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_query_font_char_infos_iterator(
        R: *const xcb_query_font_reply_t,
    ) -> xcb_charinfo_iterator_t;
    pub fn xcb_query_font_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_font_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_font_reply_t;
    pub fn xcb_query_text_extents_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        string_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_query_text_extents(
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
        string_len: uint32_t,
        string: *const xcb_char2b_t,
    ) -> xcb_query_text_extents_cookie_t;
    pub fn xcb_query_text_extents_unchecked(
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
        string_len: uint32_t,
        string: *const xcb_char2b_t,
    ) -> xcb_query_text_extents_cookie_t;
    pub fn xcb_query_text_extents_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_text_extents_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_text_extents_reply_t;
    pub fn xcb_str_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_str_name(R: *const xcb_str_t) -> *mut ::std::os::raw::c_char;
    pub fn xcb_str_name_length(R: *const xcb_str_t) -> ::std::os::raw::c_int;
    pub fn xcb_str_name_end(R: *const xcb_str_t) -> xcb_generic_iterator_t;
    pub fn xcb_str_next(i: *mut xcb_str_iterator_t);
    pub fn xcb_str_end(i: xcb_str_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_list_fonts_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_list_fonts(
        c: *mut xcb_connection_t,
        max_names: uint16_t,
        pattern_len: uint16_t,
        pattern: *const ::std::os::raw::c_char,
    ) -> xcb_list_fonts_cookie_t;
    pub fn xcb_list_fonts_unchecked(
        c: *mut xcb_connection_t,
        max_names: uint16_t,
        pattern_len: uint16_t,
        pattern: *const ::std::os::raw::c_char,
    ) -> xcb_list_fonts_cookie_t;
    pub fn xcb_list_fonts_names_length(R: *const xcb_list_fonts_reply_t) -> ::std::os::raw::c_int;
    pub fn xcb_list_fonts_names_iterator(R: *const xcb_list_fonts_reply_t) -> xcb_str_iterator_t;
    pub fn xcb_list_fonts_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_list_fonts_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_fonts_reply_t;
    pub fn xcb_list_fonts_with_info_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_list_fonts_with_info(
        c: *mut xcb_connection_t,
        max_names: uint16_t,
        pattern_len: uint16_t,
        pattern: *const ::std::os::raw::c_char,
    ) -> xcb_list_fonts_with_info_cookie_t;
    pub fn xcb_list_fonts_with_info_unchecked(
        c: *mut xcb_connection_t,
        max_names: uint16_t,
        pattern_len: uint16_t,
        pattern: *const ::std::os::raw::c_char,
    ) -> xcb_list_fonts_with_info_cookie_t;
    pub fn xcb_list_fonts_with_info_properties(
        R: *const xcb_list_fonts_with_info_reply_t,
    ) -> *mut xcb_fontprop_t;
    pub fn xcb_list_fonts_with_info_properties_length(
        R: *const xcb_list_fonts_with_info_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_list_fonts_with_info_properties_iterator(
        R: *const xcb_list_fonts_with_info_reply_t,
    ) -> xcb_fontprop_iterator_t;
    pub fn xcb_list_fonts_with_info_name(
        R: *const xcb_list_fonts_with_info_reply_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn xcb_list_fonts_with_info_name_length(
        R: *const xcb_list_fonts_with_info_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_list_fonts_with_info_name_end(
        R: *const xcb_list_fonts_with_info_reply_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_list_fonts_with_info_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_list_fonts_with_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_fonts_with_info_reply_t;
    pub fn xcb_set_font_path_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_set_font_path_checked(
        c: *mut xcb_connection_t,
        font_qty: uint16_t,
        font: *const xcb_str_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_font_path(
        c: *mut xcb_connection_t,
        font_qty: uint16_t,
        font: *const xcb_str_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_get_font_path_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_font_path(c: *mut xcb_connection_t) -> xcb_get_font_path_cookie_t;
    pub fn xcb_get_font_path_unchecked(c: *mut xcb_connection_t) -> xcb_get_font_path_cookie_t;
    pub fn xcb_get_font_path_path_length(
        R: *const xcb_get_font_path_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_font_path_path_iterator(
        R: *const xcb_get_font_path_reply_t,
    ) -> xcb_str_iterator_t;
    pub fn xcb_get_font_path_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_font_path_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_font_path_reply_t;
    pub fn xcb_create_pixmap_checked(
        c: *mut xcb_connection_t,
        depth: uint8_t,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_pixmap(
        c: *mut xcb_connection_t,
        depth: uint8_t,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_free_pixmap_checked(
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_free_pixmap(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t) -> xcb_void_cookie_t;
    pub fn xcb_create_gc_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_create_gc_checked(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: uint32_t,
        value_list: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_gc(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: uint32_t,
        value_list: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_gc_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_change_gc_checked(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: uint32_t,
        value_list: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_gc(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: uint32_t,
        value_list: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_copy_gc_checked(
        c: *mut xcb_connection_t,
        src_gc: xcb_gcontext_t,
        dst_gc: xcb_gcontext_t,
        value_mask: uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_copy_gc(
        c: *mut xcb_connection_t,
        src_gc: xcb_gcontext_t,
        dst_gc: xcb_gcontext_t,
        value_mask: uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_dashes_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_set_dashes_checked(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        dash_offset: uint16_t,
        dashes_len: uint16_t,
        dashes: *const uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_dashes(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        dash_offset: uint16_t,
        dashes_len: uint16_t,
        dashes: *const uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_clip_rectangles_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        rectangles_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_set_clip_rectangles_checked(
        c: *mut xcb_connection_t,
        ordering: uint8_t,
        gc: xcb_gcontext_t,
        clip_x_origin: int16_t,
        clip_y_origin: int16_t,
        rectangles_len: uint32_t,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_clip_rectangles(
        c: *mut xcb_connection_t,
        ordering: uint8_t,
        gc: xcb_gcontext_t,
        clip_x_origin: int16_t,
        clip_y_origin: int16_t,
        rectangles_len: uint32_t,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_free_gc_checked(c: *mut xcb_connection_t, gc: xcb_gcontext_t) -> xcb_void_cookie_t;
    pub fn xcb_free_gc(c: *mut xcb_connection_t, gc: xcb_gcontext_t) -> xcb_void_cookie_t;
    pub fn xcb_clear_area_checked(
        c: *mut xcb_connection_t,
        exposures: uint8_t,
        window: xcb_window_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_clear_area(
        c: *mut xcb_connection_t,
        exposures: uint8_t,
        window: xcb_window_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_copy_area_checked(
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: int16_t,
        src_y: int16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_copy_area(
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: int16_t,
        src_y: int16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_copy_plane_checked(
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: int16_t,
        src_y: int16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        width: uint16_t,
        height: uint16_t,
        bit_plane: uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_copy_plane(
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: int16_t,
        src_y: int16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        width: uint16_t,
        height: uint16_t,
        bit_plane: uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_point_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        points_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_poly_point_checked(
        c: *mut xcb_connection_t,
        coordinate_mode: uint8_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: uint32_t,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_point(
        c: *mut xcb_connection_t,
        coordinate_mode: uint8_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: uint32_t,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_line_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        points_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_poly_line_checked(
        c: *mut xcb_connection_t,
        coordinate_mode: uint8_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: uint32_t,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_line(
        c: *mut xcb_connection_t,
        coordinate_mode: uint8_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: uint32_t,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_segment_next(i: *mut xcb_segment_iterator_t);
    pub fn xcb_segment_end(i: xcb_segment_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_poly_segment_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        segments_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_poly_segment_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        segments_len: uint32_t,
        segments: *const xcb_segment_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_segment(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        segments_len: uint32_t,
        segments: *const xcb_segment_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_rectangle_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        rectangles_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_poly_rectangle_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: uint32_t,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_rectangle(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: uint32_t,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_arc_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        arcs_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_poly_arc_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: uint32_t,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_arc(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: uint32_t,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_fill_poly_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        points_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_fill_poly_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        shape: uint8_t,
        coordinate_mode: uint8_t,
        points_len: uint32_t,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_fill_poly(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        shape: uint8_t,
        coordinate_mode: uint8_t,
        points_len: uint32_t,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_fill_rectangle_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        rectangles_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_poly_fill_rectangle_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: uint32_t,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_fill_rectangle(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: uint32_t,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_fill_arc_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        arcs_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_poly_fill_arc_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: uint32_t,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_fill_arc(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: uint32_t,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_put_image_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        data_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_put_image_checked(
        c: *mut xcb_connection_t,
        format: uint8_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        width: uint16_t,
        height: uint16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        left_pad: uint8_t,
        depth: uint8_t,
        data_len: uint32_t,
        data: *const uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_put_image(
        c: *mut xcb_connection_t,
        format: uint8_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        width: uint16_t,
        height: uint16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        left_pad: uint8_t,
        depth: uint8_t,
        data_len: uint32_t,
        data: *const uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_get_image_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_get_image(
        c: *mut xcb_connection_t,
        format: uint8_t,
        drawable: xcb_drawable_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
        plane_mask: uint32_t,
    ) -> xcb_get_image_cookie_t;
    pub fn xcb_get_image_unchecked(
        c: *mut xcb_connection_t,
        format: uint8_t,
        drawable: xcb_drawable_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
        plane_mask: uint32_t,
    ) -> xcb_get_image_cookie_t;
    pub fn xcb_get_image_data(R: *const xcb_get_image_reply_t) -> *mut uint8_t;
    pub fn xcb_get_image_data_length(R: *const xcb_get_image_reply_t) -> ::std::os::raw::c_int;
    pub fn xcb_get_image_data_end(R: *const xcb_get_image_reply_t) -> xcb_generic_iterator_t;
    pub fn xcb_get_image_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_image_reply_t;
    pub fn xcb_poly_text_8_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        items_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_poly_text_8_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: int16_t,
        y: int16_t,
        items_len: uint32_t,
        items: *const uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_text_8(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: int16_t,
        y: int16_t,
        items_len: uint32_t,
        items: *const uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_text_16_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        items_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_poly_text_16_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: int16_t,
        y: int16_t,
        items_len: uint32_t,
        items: *const uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poly_text_16(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: int16_t,
        y: int16_t,
        items_len: uint32_t,
        items: *const uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_image_text_8_sizeof(_buffer: *const ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
    pub fn xcb_image_text_8_checked(
        c: *mut xcb_connection_t,
        string_len: uint8_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: int16_t,
        y: int16_t,
        string: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
    pub fn xcb_image_text_8(
        c: *mut xcb_connection_t,
        string_len: uint8_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: int16_t,
        y: int16_t,
        string: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
    pub fn xcb_image_text_16_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_image_text_16_checked(
        c: *mut xcb_connection_t,
        string_len: uint8_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: int16_t,
        y: int16_t,
        string: *const xcb_char2b_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_image_text_16(
        c: *mut xcb_connection_t,
        string_len: uint8_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: int16_t,
        y: int16_t,
        string: *const xcb_char2b_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_colormap_checked(
        c: *mut xcb_connection_t,
        alloc: uint8_t,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_colormap(
        c: *mut xcb_connection_t,
        alloc: uint8_t,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_free_colormap_checked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_free_colormap(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t;
    pub fn xcb_copy_colormap_and_free_checked(
        c: *mut xcb_connection_t,
        mid: xcb_colormap_t,
        src_cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_copy_colormap_and_free(
        c: *mut xcb_connection_t,
        mid: xcb_colormap_t,
        src_cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_install_colormap_checked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_install_colormap(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_uninstall_colormap_checked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_uninstall_colormap(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_list_installed_colormaps_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_list_installed_colormaps(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_installed_colormaps_cookie_t;
    pub fn xcb_list_installed_colormaps_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_installed_colormaps_cookie_t;
    pub fn xcb_list_installed_colormaps_cmaps(
        R: *const xcb_list_installed_colormaps_reply_t,
    ) -> *mut xcb_colormap_t;
    pub fn xcb_list_installed_colormaps_cmaps_length(
        R: *const xcb_list_installed_colormaps_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_list_installed_colormaps_cmaps_end(
        R: *const xcb_list_installed_colormaps_reply_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_list_installed_colormaps_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_list_installed_colormaps_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_installed_colormaps_reply_t;
    pub fn xcb_alloc_color(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        red: uint16_t,
        green: uint16_t,
        blue: uint16_t,
    ) -> xcb_alloc_color_cookie_t;
    pub fn xcb_alloc_color_unchecked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        red: uint16_t,
        green: uint16_t,
        blue: uint16_t,
    ) -> xcb_alloc_color_cookie_t;
    pub fn xcb_alloc_color_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_color_reply_t;
    pub fn xcb_alloc_named_color_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_alloc_named_color(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: uint16_t,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_alloc_named_color_cookie_t;
    pub fn xcb_alloc_named_color_unchecked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: uint16_t,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_alloc_named_color_cookie_t;
    pub fn xcb_alloc_named_color_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_named_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_named_color_reply_t;
    pub fn xcb_alloc_color_cells_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_alloc_color_cells(
        c: *mut xcb_connection_t,
        contiguous: uint8_t,
        cmap: xcb_colormap_t,
        colors: uint16_t,
        planes: uint16_t,
    ) -> xcb_alloc_color_cells_cookie_t;
    pub fn xcb_alloc_color_cells_unchecked(
        c: *mut xcb_connection_t,
        contiguous: uint8_t,
        cmap: xcb_colormap_t,
        colors: uint16_t,
        planes: uint16_t,
    ) -> xcb_alloc_color_cells_cookie_t;
    pub fn xcb_alloc_color_cells_pixels(R: *const xcb_alloc_color_cells_reply_t) -> *mut uint32_t;
    pub fn xcb_alloc_color_cells_pixels_length(
        R: *const xcb_alloc_color_cells_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_alloc_color_cells_pixels_end(
        R: *const xcb_alloc_color_cells_reply_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_alloc_color_cells_masks(R: *const xcb_alloc_color_cells_reply_t) -> *mut uint32_t;
    pub fn xcb_alloc_color_cells_masks_length(
        R: *const xcb_alloc_color_cells_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_alloc_color_cells_masks_end(
        R: *const xcb_alloc_color_cells_reply_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_alloc_color_cells_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_cells_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_color_cells_reply_t;
    pub fn xcb_alloc_color_planes_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_alloc_color_planes(
        c: *mut xcb_connection_t,
        contiguous: uint8_t,
        cmap: xcb_colormap_t,
        colors: uint16_t,
        reds: uint16_t,
        greens: uint16_t,
        blues: uint16_t,
    ) -> xcb_alloc_color_planes_cookie_t;
    pub fn xcb_alloc_color_planes_unchecked(
        c: *mut xcb_connection_t,
        contiguous: uint8_t,
        cmap: xcb_colormap_t,
        colors: uint16_t,
        reds: uint16_t,
        greens: uint16_t,
        blues: uint16_t,
    ) -> xcb_alloc_color_planes_cookie_t;
    pub fn xcb_alloc_color_planes_pixels(R: *const xcb_alloc_color_planes_reply_t)
        -> *mut uint32_t;
    pub fn xcb_alloc_color_planes_pixels_length(
        R: *const xcb_alloc_color_planes_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_alloc_color_planes_pixels_end(
        R: *const xcb_alloc_color_planes_reply_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_alloc_color_planes_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_planes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_color_planes_reply_t;
    pub fn xcb_free_colors_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        pixels_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_free_colors_checked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        plane_mask: uint32_t,
        pixels_len: uint32_t,
        pixels: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_free_colors(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        plane_mask: uint32_t,
        pixels_len: uint32_t,
        pixels: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_coloritem_next(i: *mut xcb_coloritem_iterator_t);
    pub fn xcb_coloritem_end(i: xcb_coloritem_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_store_colors_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        items_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_store_colors_checked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        items_len: uint32_t,
        items: *const xcb_coloritem_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_store_colors(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        items_len: uint32_t,
        items: *const xcb_coloritem_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_store_named_color_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_store_named_color_checked(
        c: *mut xcb_connection_t,
        flags: uint8_t,
        cmap: xcb_colormap_t,
        pixel: uint32_t,
        name_len: uint16_t,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
    pub fn xcb_store_named_color(
        c: *mut xcb_connection_t,
        flags: uint8_t,
        cmap: xcb_colormap_t,
        pixel: uint32_t,
        name_len: uint16_t,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
    pub fn xcb_rgb_next(i: *mut xcb_rgb_iterator_t);
    pub fn xcb_rgb_end(i: xcb_rgb_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_query_colors_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        pixels_len: uint32_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_query_colors(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        pixels_len: uint32_t,
        pixels: *const uint32_t,
    ) -> xcb_query_colors_cookie_t;
    pub fn xcb_query_colors_unchecked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        pixels_len: uint32_t,
        pixels: *const uint32_t,
    ) -> xcb_query_colors_cookie_t;
    pub fn xcb_query_colors_colors(R: *const xcb_query_colors_reply_t) -> *mut xcb_rgb_t;
    pub fn xcb_query_colors_colors_length(
        R: *const xcb_query_colors_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_query_colors_colors_iterator(
        R: *const xcb_query_colors_reply_t,
    ) -> xcb_rgb_iterator_t;
    pub fn xcb_query_colors_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_colors_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_colors_reply_t;
    pub fn xcb_lookup_color_sizeof(_buffer: *const ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
    pub fn xcb_lookup_color(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: uint16_t,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_lookup_color_cookie_t;
    pub fn xcb_lookup_color_unchecked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: uint16_t,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_lookup_color_cookie_t;
    pub fn xcb_lookup_color_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_lookup_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_lookup_color_reply_t;
    pub fn xcb_create_cursor_checked(
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source: xcb_pixmap_t,
        mask: xcb_pixmap_t,
        fore_red: uint16_t,
        fore_green: uint16_t,
        fore_blue: uint16_t,
        back_red: uint16_t,
        back_green: uint16_t,
        back_blue: uint16_t,
        x: uint16_t,
        y: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_cursor(
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source: xcb_pixmap_t,
        mask: xcb_pixmap_t,
        fore_red: uint16_t,
        fore_green: uint16_t,
        fore_blue: uint16_t,
        back_red: uint16_t,
        back_green: uint16_t,
        back_blue: uint16_t,
        x: uint16_t,
        y: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_glyph_cursor_checked(
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source_font: xcb_font_t,
        mask_font: xcb_font_t,
        source_char: uint16_t,
        mask_char: uint16_t,
        fore_red: uint16_t,
        fore_green: uint16_t,
        fore_blue: uint16_t,
        back_red: uint16_t,
        back_green: uint16_t,
        back_blue: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_glyph_cursor(
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source_font: xcb_font_t,
        mask_font: xcb_font_t,
        source_char: uint16_t,
        mask_char: uint16_t,
        fore_red: uint16_t,
        fore_green: uint16_t,
        fore_blue: uint16_t,
        back_red: uint16_t,
        back_green: uint16_t,
        back_blue: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_free_cursor_checked(
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_free_cursor(c: *mut xcb_connection_t, cursor: xcb_cursor_t) -> xcb_void_cookie_t;
    pub fn xcb_recolor_cursor_checked(
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        fore_red: uint16_t,
        fore_green: uint16_t,
        fore_blue: uint16_t,
        back_red: uint16_t,
        back_green: uint16_t,
        back_blue: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_recolor_cursor(
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        fore_red: uint16_t,
        fore_green: uint16_t,
        fore_blue: uint16_t,
        back_red: uint16_t,
        back_green: uint16_t,
        back_blue: uint16_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_query_best_size(
        c: *mut xcb_connection_t,
        _class: uint8_t,
        drawable: xcb_drawable_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_query_best_size_cookie_t;
    pub fn xcb_query_best_size_unchecked(
        c: *mut xcb_connection_t,
        _class: uint8_t,
        drawable: xcb_drawable_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_query_best_size_cookie_t;
    pub fn xcb_query_best_size_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_best_size_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_best_size_reply_t;
    pub fn xcb_query_extension_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_query_extension(
        c: *mut xcb_connection_t,
        name_len: uint16_t,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_query_extension_cookie_t;
    pub fn xcb_query_extension_unchecked(
        c: *mut xcb_connection_t,
        name_len: uint16_t,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_query_extension_cookie_t;
    pub fn xcb_query_extension_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_extension_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_extension_reply_t;
    pub fn xcb_list_extensions_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_list_extensions(c: *mut xcb_connection_t) -> xcb_list_extensions_cookie_t;
    pub fn xcb_list_extensions_unchecked(c: *mut xcb_connection_t) -> xcb_list_extensions_cookie_t;
    pub fn xcb_list_extensions_names_length(
        R: *const xcb_list_extensions_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_list_extensions_names_iterator(
        R: *const xcb_list_extensions_reply_t,
    ) -> xcb_str_iterator_t;
    pub fn xcb_list_extensions_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_list_extensions_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_extensions_reply_t;
    pub fn xcb_change_keyboard_mapping_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_change_keyboard_mapping_checked(
        c: *mut xcb_connection_t,
        keycode_count: uint8_t,
        first_keycode: xcb_keycode_t,
        keysyms_per_keycode: uint8_t,
        keysyms: *const xcb_keysym_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_keyboard_mapping(
        c: *mut xcb_connection_t,
        keycode_count: uint8_t,
        first_keycode: xcb_keycode_t,
        keysyms_per_keycode: uint8_t,
        keysyms: *const xcb_keysym_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_get_keyboard_mapping_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_keyboard_mapping(
        c: *mut xcb_connection_t,
        first_keycode: xcb_keycode_t,
        count: uint8_t,
    ) -> xcb_get_keyboard_mapping_cookie_t;
    pub fn xcb_get_keyboard_mapping_unchecked(
        c: *mut xcb_connection_t,
        first_keycode: xcb_keycode_t,
        count: uint8_t,
    ) -> xcb_get_keyboard_mapping_cookie_t;
    pub fn xcb_get_keyboard_mapping_keysyms(
        R: *const xcb_get_keyboard_mapping_reply_t,
    ) -> *mut xcb_keysym_t;
    pub fn xcb_get_keyboard_mapping_keysyms_length(
        R: *const xcb_get_keyboard_mapping_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_keyboard_mapping_keysyms_end(
        R: *const xcb_get_keyboard_mapping_reply_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_get_keyboard_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_keyboard_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_keyboard_mapping_reply_t;
    pub fn xcb_change_keyboard_control_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_change_keyboard_control_checked(
        c: *mut xcb_connection_t,
        value_mask: uint32_t,
        value_list: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_keyboard_control(
        c: *mut xcb_connection_t,
        value_mask: uint32_t,
        value_list: *const uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_get_keyboard_control(c: *mut xcb_connection_t) -> xcb_get_keyboard_control_cookie_t;
    pub fn xcb_get_keyboard_control_unchecked(
        c: *mut xcb_connection_t,
    ) -> xcb_get_keyboard_control_cookie_t;
    pub fn xcb_get_keyboard_control_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_keyboard_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_keyboard_control_reply_t;
    pub fn xcb_bell_checked(c: *mut xcb_connection_t, percent: int8_t) -> xcb_void_cookie_t;
    pub fn xcb_bell(c: *mut xcb_connection_t, percent: int8_t) -> xcb_void_cookie_t;
    pub fn xcb_change_pointer_control_checked(
        c: *mut xcb_connection_t,
        acceleration_numerator: int16_t,
        acceleration_denominator: int16_t,
        threshold: int16_t,
        do_acceleration: uint8_t,
        do_threshold: uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_pointer_control(
        c: *mut xcb_connection_t,
        acceleration_numerator: int16_t,
        acceleration_denominator: int16_t,
        threshold: int16_t,
        do_acceleration: uint8_t,
        do_threshold: uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_get_pointer_control(c: *mut xcb_connection_t) -> xcb_get_pointer_control_cookie_t;
    pub fn xcb_get_pointer_control_unchecked(
        c: *mut xcb_connection_t,
    ) -> xcb_get_pointer_control_cookie_t;
    pub fn xcb_get_pointer_control_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_pointer_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_pointer_control_reply_t;
    pub fn xcb_set_screen_saver_checked(
        c: *mut xcb_connection_t,
        timeout: int16_t,
        interval: int16_t,
        prefer_blanking: uint8_t,
        allow_exposures: uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_screen_saver(
        c: *mut xcb_connection_t,
        timeout: int16_t,
        interval: int16_t,
        prefer_blanking: uint8_t,
        allow_exposures: uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_get_screen_saver(c: *mut xcb_connection_t) -> xcb_get_screen_saver_cookie_t;
    pub fn xcb_get_screen_saver_unchecked(
        c: *mut xcb_connection_t,
    ) -> xcb_get_screen_saver_cookie_t;
    pub fn xcb_get_screen_saver_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_screen_saver_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_screen_saver_reply_t;
    pub fn xcb_change_hosts_sizeof(_buffer: *const ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
    pub fn xcb_change_hosts_checked(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        family: uint8_t,
        address_len: uint16_t,
        address: *const uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_hosts(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        family: uint8_t,
        address_len: uint16_t,
        address: *const uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_host_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_host_address(R: *const xcb_host_t) -> *mut uint8_t;
    pub fn xcb_host_address_length(R: *const xcb_host_t) -> ::std::os::raw::c_int;
    pub fn xcb_host_address_end(R: *const xcb_host_t) -> xcb_generic_iterator_t;
    pub fn xcb_host_next(i: *mut xcb_host_iterator_t);
    pub fn xcb_host_end(i: xcb_host_iterator_t) -> xcb_generic_iterator_t;
    pub fn xcb_list_hosts_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn xcb_list_hosts(c: *mut xcb_connection_t) -> xcb_list_hosts_cookie_t;
    pub fn xcb_list_hosts_unchecked(c: *mut xcb_connection_t) -> xcb_list_hosts_cookie_t;
    pub fn xcb_list_hosts_hosts_length(R: *const xcb_list_hosts_reply_t) -> ::std::os::raw::c_int;
    pub fn xcb_list_hosts_hosts_iterator(R: *const xcb_list_hosts_reply_t) -> xcb_host_iterator_t;
    pub fn xcb_list_hosts_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_list_hosts_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_hosts_reply_t;
    pub fn xcb_set_access_control_checked(
        c: *mut xcb_connection_t,
        mode: uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_access_control(c: *mut xcb_connection_t, mode: uint8_t) -> xcb_void_cookie_t;
    pub fn xcb_set_close_down_mode_checked(
        c: *mut xcb_connection_t,
        mode: uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_close_down_mode(c: *mut xcb_connection_t, mode: uint8_t) -> xcb_void_cookie_t;
    pub fn xcb_kill_client_checked(
        c: *mut xcb_connection_t,
        resource: uint32_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_kill_client(c: *mut xcb_connection_t, resource: uint32_t) -> xcb_void_cookie_t;
    pub fn xcb_rotate_properties_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_rotate_properties_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        atoms_len: uint16_t,
        delta: int16_t,
        atoms: *const xcb_atom_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_rotate_properties(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        atoms_len: uint16_t,
        delta: int16_t,
        atoms: *const xcb_atom_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_force_screen_saver_checked(
        c: *mut xcb_connection_t,
        mode: uint8_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_force_screen_saver(c: *mut xcb_connection_t, mode: uint8_t) -> xcb_void_cookie_t;
    pub fn xcb_set_pointer_mapping_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_set_pointer_mapping(
        c: *mut xcb_connection_t,
        map_len: uint8_t,
        map: *const uint8_t,
    ) -> xcb_set_pointer_mapping_cookie_t;
    pub fn xcb_set_pointer_mapping_unchecked(
        c: *mut xcb_connection_t,
        map_len: uint8_t,
        map: *const uint8_t,
    ) -> xcb_set_pointer_mapping_cookie_t;
    pub fn xcb_set_pointer_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_set_pointer_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_set_pointer_mapping_reply_t;
    pub fn xcb_get_pointer_mapping_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_pointer_mapping(c: *mut xcb_connection_t) -> xcb_get_pointer_mapping_cookie_t;
    pub fn xcb_get_pointer_mapping_unchecked(
        c: *mut xcb_connection_t,
    ) -> xcb_get_pointer_mapping_cookie_t;
    pub fn xcb_get_pointer_mapping_map(R: *const xcb_get_pointer_mapping_reply_t) -> *mut uint8_t;
    pub fn xcb_get_pointer_mapping_map_length(
        R: *const xcb_get_pointer_mapping_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_pointer_mapping_map_end(
        R: *const xcb_get_pointer_mapping_reply_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_get_pointer_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_pointer_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_pointer_mapping_reply_t;
    pub fn xcb_set_modifier_mapping_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_set_modifier_mapping(
        c: *mut xcb_connection_t,
        keycodes_per_modifier: uint8_t,
        keycodes: *const xcb_keycode_t,
    ) -> xcb_set_modifier_mapping_cookie_t;
    pub fn xcb_set_modifier_mapping_unchecked(
        c: *mut xcb_connection_t,
        keycodes_per_modifier: uint8_t,
        keycodes: *const xcb_keycode_t,
    ) -> xcb_set_modifier_mapping_cookie_t;
    pub fn xcb_set_modifier_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_set_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_set_modifier_mapping_reply_t;
    pub fn xcb_get_modifier_mapping_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_modifier_mapping(c: *mut xcb_connection_t) -> xcb_get_modifier_mapping_cookie_t;
    pub fn xcb_get_modifier_mapping_unchecked(
        c: *mut xcb_connection_t,
    ) -> xcb_get_modifier_mapping_cookie_t;
    pub fn xcb_get_modifier_mapping_keycodes(
        R: *const xcb_get_modifier_mapping_reply_t,
    ) -> *mut xcb_keycode_t;
    pub fn xcb_get_modifier_mapping_keycodes_length(
        R: *const xcb_get_modifier_mapping_reply_t,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_get_modifier_mapping_keycodes_end(
        R: *const xcb_get_modifier_mapping_reply_t,
    ) -> xcb_generic_iterator_t;
    pub fn xcb_get_modifier_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_modifier_mapping_reply_t;
    pub fn xcb_no_operation_checked(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    pub fn xcb_no_operation(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    pub fn xcb_flush(c: *mut xcb_connection_t) -> ::std::os::raw::c_int;
    pub fn xcb_get_maximum_request_length(c: *mut xcb_connection_t) -> uint32_t;
    pub fn xcb_prefetch_maximum_request_length(c: *mut xcb_connection_t);
    pub fn xcb_wait_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    pub fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    pub fn xcb_poll_for_queued_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    pub fn xcb_poll_for_special_event(
        c: *mut xcb_connection_t,
        se: *mut xcb_special_event_t,
    ) -> *mut xcb_generic_event_t;
    pub fn xcb_wait_for_special_event(
        c: *mut xcb_connection_t,
        se: *mut xcb_special_event_t,
    ) -> *mut xcb_generic_event_t;
    pub fn xcb_register_for_special_xge(
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
        eid: uint32_t,
        stamp: *mut uint32_t,
    ) -> *mut xcb_special_event_t;
    pub fn xcb_unregister_for_special_event(c: *mut xcb_connection_t, se: *mut xcb_special_event_t);
    pub fn xcb_request_check(
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t;
    pub fn xcb_discard_reply(c: *mut xcb_connection_t, sequence: ::std::os::raw::c_uint);
    pub fn xcb_discard_reply64(c: *mut xcb_connection_t, sequence: uint64_t);
    pub fn xcb_get_extension_data(
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
    ) -> *const xcb_query_extension_reply_t;
    pub fn xcb_prefetch_extension_data(c: *mut xcb_connection_t, ext: *mut xcb_extension_t);
    pub fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
    pub fn xcb_get_file_descriptor(c: *mut xcb_connection_t) -> ::std::os::raw::c_int;
    pub fn xcb_connection_has_error(c: *mut xcb_connection_t) -> ::std::os::raw::c_int;
    pub fn xcb_connect_to_fd(
        fd: ::std::os::raw::c_int,
        auth_info: *mut xcb_auth_info_t,
    ) -> *mut xcb_connection_t;
    pub fn xcb_disconnect(c: *mut xcb_connection_t);
    pub fn xcb_parse_display(
        name: *const ::std::os::raw::c_char,
        host: *mut *mut ::std::os::raw::c_char,
        display: *mut ::std::os::raw::c_int,
        screen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn xcb_connect(
        displayname: *const ::std::os::raw::c_char,
        screenp: *mut ::std::os::raw::c_int,
    ) -> *mut xcb_connection_t;
    pub fn xcb_connect_to_display_with_auth_info(
        display: *const ::std::os::raw::c_char,
        auth: *mut xcb_auth_info_t,
        screen: *mut ::std::os::raw::c_int,
    ) -> *mut xcb_connection_t;
    pub fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
}
