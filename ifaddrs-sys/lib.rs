#![cfg(target_os = "android")]
extern crate libc;
use libc::*;
#[repr(C)]
#[derive(Debug)]
pub struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut c_char,
    pub ifa_flags: ::c_uint,
    pub ifa_addr: *mut ::sockaddr,
    pub ifa_netmask: *mut ::sockaddr,
    pub ifa_ifu: *mut ::sockaddr,
    pub ifa_data: *mut ::c_void,
}

extern "C" {
    pub fn getifaddrs(ifap: *mut *mut ::ifaddrs) -> ::c_int;
    pub fn freeifaddrs(ifa: *mut ::ifaddrs);
}
