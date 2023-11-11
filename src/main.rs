mod elevation;

use log::info;
use std::io;

fn main() {
    elevation::init();
    elevation::scan_relative_directory("elevations").expect("Incorrectly handled error:");

    let mut c = String::new();
    info!("Press any key to exit.");
    io::stdin().read_line(&mut c).expect("End of the test program");
}
