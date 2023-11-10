mod elevation;

use log::info;
use std::io;
use byteorder::ReadBytesExt;

fn main() {
    elevation::init();
    elevation::scan_relative_directory("elevations").expect("Failure");

    let mut c = String::new();
    info!("Press any key to exit.");
    io::stdin().read_line(&mut c).expect("End of the test program");
}
