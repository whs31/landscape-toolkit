use std::ffi::{c_char, CStr};
use crate::elevation;

#[repr(C)]
pub struct LEDResult
{
    pub result: f32,
    pub valid: bool
}

#[no_mangle]
#[allow(dead_code)]
pub extern "C" fn led_init_logger()
{
    elevation::init_logger();
}

#[no_mangle]
#[allow(dead_code)]
pub fn led_load_relative_directory(path: *const c_char) -> bool
{
    match elevation::scan_relative_directory(c_char_to_string(path).as_str()) {
        Ok(_) => true,
        Err(_) => false
    }
}

#[no_mangle]
#[allow(dead_code)]
pub fn led_load_directory(path: *const c_char) -> bool
{
    match elevation::scan_directory(&c_char_to_string(path)) {
        Ok(_) => true,
        Err(_) => false
    }
}

#[no_mangle]
#[allow(dead_code)]
pub fn led_elevation_at(latitude: f64, longitude: f64) -> LEDResult
{
    match elevation::elevation_at((latitude, longitude)) {
        Ok(x) => LEDResult {
            result: x,
            valid: true
        },
        Err(_) => LEDResult {
            result: 0.0,
            valid: false
        }
    }
}

fn c_char_to_string(ptr: *const c_char) -> String
{
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map(|s| s.to_owned())
        .unwrap()
}

