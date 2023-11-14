use std::env;
use std::ffi::{c_char, c_double, c_float, c_int, CStr, CString};
use num_traits::{FromPrimitive, ToPrimitive};
use once_cell::sync::Lazy;
use crate::elevation;

#[repr(C)]
pub struct LEDResult
{
    pub result: c_float,
    pub valid: bool
}

#[repr(C)]
pub struct LEDVersion
{
    pub major: c_int,
    pub minor: c_int,
    pub patch: c_int
}

#[no_mangle]
#[allow(dead_code)]
pub extern fn led_version() -> LEDVersion
{
    LEDVersion {
        major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
        minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap()
    }
}

static BINARY_DIRECTORY: Lazy<String> = Lazy::new(|| { env::current_dir()
    .unwrap()
    .into_os_string()
    .into_string()
    .unwrap() });

#[no_mangle]
#[allow(dead_code)]
pub extern fn led_binary_directory() -> *const c_char
{
    let c_str = CString::new(BINARY_DIRECTORY.clone()).unwrap();
    c_str.into_raw() as *const _
}

pub unsafe extern fn led_free_string(ptr: *const c_char)
{
    let _ = CString::from_raw(ptr as *mut _);
}

#[no_mangle]
#[allow(dead_code)]
pub extern "C" fn led_init_logger() -> bool
{
    elevation::init_logger()
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
pub fn led_set_relative_directory(path: *const c_char) -> bool
{
    match elevation::set_relative_directory(c_char_to_string(path).as_str()) {
        Ok(_) => true,
        Err(_) => false
    }
}

#[no_mangle]
#[allow(dead_code)]
pub fn led_set_directory(path: *const c_char) -> bool
{
    match elevation::set_directory(&c_char_to_string(path)) {
        Ok(_) => true,
        Err(_) => false
    }
}

#[no_mangle]
#[allow(dead_code)]
pub fn led_elevation_at(latitude: c_double, longitude: c_double, preload_mode: c_int) -> LEDResult
{
    match elevation::elevation_at((latitude as f64, longitude as f64), FromPrimitive::from_i32(preload_mode
        .to_i32()
        .unwrap())
        .unwrap()) {
        Ok(x) => LEDResult {
            result: x as c_float,
            valid: true
        },
        Err(_) => LEDResult {
            result: 0.0,
            valid: false
        }
    }
}

#[no_mangle]
#[allow(dead_code)]
pub fn led_elevation_at_as_int(latitude: c_double, longitude: c_double, preload_mode: c_int) -> c_int
{
    match elevation::elevation_at((latitude as f64, longitude as f64), FromPrimitive::from_i32(preload_mode
        .to_i32()
        .unwrap())
        .unwrap()) {
        Ok(x) => x as c_int,
        Err(_) => -404
    }
}

fn c_char_to_string(ptr: *const c_char) -> String
{
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map(|s| s.to_owned())
        .unwrap()
}

#[cfg(test)]
mod tests
{
    use std::ffi::CString;
    use std::path::MAIN_SEPARATOR;
    use crate::ffi_exports;
    use crate::ffi_exports::c_char_to_string;

    #[test]
    fn test_ffi_no_preload()
    {
        let path = CString::new(format!("testdata{}elevations", MAIN_SEPARATOR)
            .as_str()
            .to_owned())
            .unwrap()
            .into_raw()
            .cast_const();
        let result = ffi_exports::led_set_relative_directory(path);
        let a = ffi_exports::led_elevation_at(60.0, 30.0, 1).result;
        let b = ffi_exports::led_elevation_at(60.9, 30.9, 1).result;
        let c = ffi_exports::led_elevation_at(60.5, 30.5, 1).result;
        let d = ffi_exports::led_elevation_at(50.5, 39.5, 1);

        assert!(result);
        assert!(a >= -1.0 && a <= 1.0);
        assert!(b >= 2.0 && b <= 4.0);
        assert!(c >= 60.0 && c <= 67.0);
        assert!(d.valid == false && d.result <= 1.0 && d.result >= -1.0);
    }

    #[test]
    fn test_ffi_functions()
    {
        let path = CString::new(format!("testdata{}elevations", MAIN_SEPARATOR)
            .as_str()
            .to_owned())
            .unwrap()
            .into_raw()
            .cast_const();
        let result = ffi_exports::led_load_relative_directory(path);
        let a = ffi_exports::led_elevation_at(60.0, 30.0, 2).result;
        let b = ffi_exports::led_elevation_at(60.9, 30.9, 2).result;
        let c = ffi_exports::led_elevation_at(60.5, 30.5, 2).result;
        let d = ffi_exports::led_elevation_at(50.5, 39.5, 2);

        assert!(result);
        assert!(a >= -1.0 && a <= 1.0);
        assert!(b >= 2.0 && b <= 4.0);
        assert!(c >= 60.0 && c <= 67.0);
        assert!(d.valid == false && d.result <= 1.0 && d.result >= -1.0);
    }

    #[test]
    fn test_miscellaneous()
    {
        let str = c_char_to_string(ffi_exports::led_binary_directory());
        assert!(!ffi_exports::led_binary_directory().is_null());
        assert!(!str.is_empty());
    }
}