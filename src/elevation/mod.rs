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
    InvalidFileExtension,
    NoSuchTile,
    LibraryError
}

pub fn init_logger()
{
    pretty_env_logger::init();
}

pub fn elevation_at(coordinate: (f64, f64)) -> Result<f32, ElevationError>
{
    elevation_impl::elevation_at(coordinate)
}

pub fn scan_relative_directory(relative_directory: &str) -> Result<(), ElevationError>
{
    scan::scan_relative_directory(relative_directory)
}

pub fn scan_directory(directory: &String) -> Result<(), ElevationError>
{
    scan::scan_directory(directory)
}

