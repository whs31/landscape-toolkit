mod elevation;

use log::{info, warn};
use std::io;
use crate::elevation::elevation_at;

fn main() {
    elevation::init_logger();
    elevation::scan_relative_directory("elevations").expect("Incorrectly handled error:");

    info!("ELEVATION: {} meters", elevation_at((61.0, 31.0)).expect("PANIC"));
    info!("ELEVATION: {} meters", elevation_at((60.0, 30.0)).expect("PANIC"));
    info!("ELEVATION: {} meters", elevation_at((60.5, 30.5)).expect("PANIC"));

    let mut c = String::new();
    info!("Press any key to exit.");
    io::stdin().read_line(&mut c).expect("End of the test program");
}
