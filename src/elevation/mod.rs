mod tile_storage;
mod fs_identity;
mod quarter;
mod utils;
mod scan;
mod elevation_impl;

#[derive(Debug)]
pub enum ElevationError
{
    InvalidQuarterDirectorySpecifier,
    KeyAlreadyAvailable,
    InvalidDirectoryPath,
    InvalidDirectoryHierarchy,
    InvalidFileExtension,
    NoSuchTile,
    ImageSizeError,
    LibraryError
}

pub fn init_logger()
{
    pretty_env_logger::init();
}

pub fn scan_relative_directory(relative_directory: &str) -> Result<(), ElevationError>
{
    scan::scan_relative_directory(relative_directory)
}

pub fn scan_directory(directory: &String) -> Result<(), ElevationError>
{
    scan::scan_directory(directory)
}

pub fn elevation_at(coordinate: (f64, f64)) -> Result<f32, ElevationError>
{
    elevation_impl::elevation_at(coordinate)
}

#[cfg(test)]
mod tests
{
    use std::path::MAIN_SEPARATOR;
    use log::warn;
    use crate::elevation;

    #[test]
    fn test_scan_relative_directory()
    {
        elevation::init_logger();
        let result = elevation::scan_relative_directory(format!("testdata{}elevations", MAIN_SEPARATOR).as_str());
        let a = elevation::elevation_at((60.0, 30.0)).unwrap();
        let b = elevation::elevation_at((60.9, 30.9)).unwrap();
        let c = elevation::elevation_at((60.5, 30.5)).unwrap();
        let d = elevation::elevation_at((50.5, 39.5));

        assert!(result.is_ok());
        assert!(a >= -1.0 && a <= 1.0);
        assert!(b >= 2.0 && b <= 4.0);
        assert!(c >= 60.0 && c <= 67.0);
        assert!(d.is_err());
    }
}