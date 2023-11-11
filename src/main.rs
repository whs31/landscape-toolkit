mod elevation;
mod ffi_exports;

use std::ffi::{c_char, CString};
use log::{error, info, warn};
use std::io;

fn main() {
    ffi_exports::led_init_logger();
    let b1 = ffi_exports::led_load_relative_directory(unsafe { match CString::new("elevations") {
        Ok(x) => x,
        Err(_) => panic!("TEST CASE PANIC"),
    }.as_ptr() as *const c_char });
    if !b1 { error!("Can't load relative directory (FFI)!"); return; }
    let elevations = [
        ffi_exports::led_elevation_at(61.0, 31.0),
        ffi_exports::led_elevation_at(60.0, 30.0),
        ffi_exports::led_elevation_at(60.5, 30.5),
        ffi_exports::led_elevation_at(39.0, 30.5)
    ];
    for e in elevations {
        if !e.valid {
            warn!("Invalid coordinate!");
            continue;
        }
        info!("Elevation value: {} meters", e.result);
    }

    let mut c = String::new();
    info!("Press any key to exit.");
    io::stdin().read_line(&mut c).expect("End of the test program");
}
