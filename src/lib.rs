#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use std::ffi::{c_void, CStr};

mod bindings;
pub use bindings::*;

impl value_string {
    pub const fn new(value: u32, string: &CStr) -> Self {
        Self {
            value,
            strptr: string.as_ptr(),
        }
    }
}

unsafe impl Sync for value_string {}

impl header_field_info {
    pub const DEFAULT: Self = Self {
        name: 0 as _,
        abbrev: 0 as _,
        type_: 0,
        display: 0,
        strings: 0 as _,
        bitmask: 0,
        blurb: 0 as _,
        id: -1,
        parent: 0,
        ref_type: 0,
        same_name_prev_id: -1,
        same_name_next: 0 as _,
    };

    pub const fn new(
        name: &CStr,
        abbr: &CStr,
        type_: u32,
        display: u32,
        strings: &[value_string],
    ) -> Self {
        Self {
            name: name.as_ptr(),
            abbrev: abbr.as_ptr(),
            type_,
            display: display as i32,
            strings: if strings.is_empty() {
                std::ptr::null()
            } else {
                strings.as_ptr() as *const c_void
            },
            ..Self::DEFAULT
        }
    }
}

impl hf_register_info {
    pub const fn new(
        p_id: *mut ::std::os::raw::c_int,
        name: &CStr,
        abbr: &CStr,
        type_: u32,
        display: u32,
        strings: &[value_string],
    ) -> Self {
        Self {
            p_id,
            hfinfo: header_field_info::new(name, abbr, type_, display, strings),
        }
    }
}

#[cfg(test)]
mod tests {
    use super as sys;
    use std::process::Command;

    // Pre 3.0 this did not exist, and we manually define it in build.rs
    #[test]
    fn check_that_version_is_correctly_defined() {
        let a: u32 = sys::WIRESHARK_VERSION_MAJOR;
        let b: u32 = sys::WIRESHARK_VERSION_MINOR;
        let c: u32 = sys::WIRESHARK_VERSION_MICRO;

        let output = Command::new("pkg-config")
            .args(["--modversion", "wireshark"])
            .output()
            .unwrap()
            .stdout;
        let output = String::from_utf8(output).unwrap();

        assert_eq!(output.trim(), format!("{a}.{b}.{c}"));
    }
}
