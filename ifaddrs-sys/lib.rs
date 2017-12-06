// Copyright 2017 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

//! ifaddrs-sys
#![doc(html_logo_url = "https://raw.githubusercontent.com/maidsafe/QA/master/Images/
maidsafe_logo.png",
      html_favicon_url = "http://maidsafe.net/img/favicon.ico",
      html_root_url = "http://maidsafe.github.io/ifaddrs")]
// For explanation of lint checks, run `rustc -W help` or see
// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md
#![forbid(exceeding_bitshifts, mutable_transmutes, no_mangle_const_items, unknown_crate_types,
         warnings)]
#![deny(bad_style, deprecated, improper_ctypes, missing_docs, non_shorthand_field_patterns,
       overflowing_literals, plugin_as_library, private_no_mangle_fns, private_no_mangle_statics,
       stable_features, unconditional_recursion, unknown_lints, unsafe_code, unused,
       unused_allocation, unused_attributes, unused_comparisons, unused_features, unused_parens,
       while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
       unused_qualifications, unused_results)]
#![allow(box_pointers, missing_copy_implementations, missing_debug_implementations,
         variant_size_differences)]
#![cfg_attr(feature = "cargo-clippy",
           deny(clippy, unicode_not_nfc, wrong_pub_self_convention, option_unwrap_used))]
#![cfg_attr(feature = "cargo-clippy", allow(use_debug, too_many_arguments))]

#![cfg(target_os = "android")]
extern crate libc;

use libc::*;

#[allow(missing_docs)]
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
