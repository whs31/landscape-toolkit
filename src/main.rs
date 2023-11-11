mod elevation;

use log::{info, warn};
use std::io;

fn main() {
    elevation::init();
    elevation::scan_relative_directory("elevations").expect("Incorrectly handled error:");
    let ele = match elevation::elevation_at((43.5, 39.5)) {
        Ok(x) => x,
        Err(e) => { warn!("elevation_at failed with reason: {:?}", e); 0.0 }
    };
    info!("ELEVATION: {} meters", ele);

    let mut c = String::new();
    info!("Press any key to exit.");
    io::stdin().read_line(&mut c).expect("End of the test program");
}
