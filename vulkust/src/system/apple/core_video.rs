use super::super::super::libc;

pub type CVDisplayLinkRef = *mut libc::c_void;
pub type CVReturn = i32;
pub type CVOptionFlags = u64;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CVSMPTETime
{
    subframes: i16,
    subframe_divisor: i16,
    counter: u32,
    type_: u32,
    flags: u32,
    hours: i16,
    minutes: i16,
    seconds: i16,
    frames: i16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CVTimeStamp {
    version: u32,
    video_time_scale: i32,
    video_time: i64,
    host_time: u64,
    rate_scalar: f64,
    video_refresh_period: i64,
    smpte_time: CVSMPTETime,
    flags: u64,
    reserved: u64,
}

type CVDisplayLinkOutputCallback = 
    extern "C" fn (
        display_link: CVDisplayLinkRef, 
        in_now: *const CVTimeStamp, 
        in_output_time: *const CVTimeStamp, 
        flags_in: CVOptionFlags,
        flags_out: *mut CVOptionFlags, 
        display_link_context: *mut libc::c_void);

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {
    pub fn CVDisplayLinkCreateWithActiveCGDisplays(d: *mut CVDisplayLinkRef) -> CVReturn;
    pub fn CVDisplayLinkSetOutputCallback(
        d: CVDisplayLinkRef, 
        callback: CVDisplayLinkOutputCallback, 
        user_data: *mut libc::c_void) -> CVReturn;
}