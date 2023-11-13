use const_format::concatcp;
use crate::elevation::ElevationError;

pub const FILE_EXTENSION: &'static str = "tif";

pub fn trim_longitude_path(path: String) -> Result<i16, ElevationError>
{
    match path.strip_suffix(concatcp!(".", FILE_EXTENSION)) {
        None => { Err(ElevationError::InvalidFileExtension) }
        Some(x) => { Ok(x.parse::<i16>().unwrap()) }
    }
}