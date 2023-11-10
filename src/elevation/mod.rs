use std::{env, fs};
use log::{debug, info};
use crate::elevation::Quarter::{BottomLeft, BottomRight, TopLeft, TopRight};

#[derive(Debug)]
pub enum ElevationError
{
    InvalidQuarterDirectorySpecifier,
    UnknownError
}

#[derive(Debug, PartialEq)]
enum Quarter
{
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3
}

pub fn init()
{
    pretty_env_logger::init();
}

pub fn scan_relative_directory(relative_directory: &str) -> Result<(), ElevationError>
{
    info!("Scanning relative directory: {}", &relative_directory);
    let abs_path = format!(
        "{}{}{}", env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
        std::path::MAIN_SEPARATOR,
        relative_directory
    );

    return scan_directory(&abs_path)
}

pub fn scan_directory(directory: &String) -> Result<(), ElevationError>
{
    info!("Scanning absolute directory: {}", &directory);
    let paths = fs::read_dir(directory).unwrap();
    for path in paths
    {
        let quarter_path = path
            .as_ref()
            .unwrap()
            .path()
            .into_os_string()
            .into_string()
            .unwrap();
        let quarter_name = path
            .as_ref()
            .unwrap()
            .file_name()
            .into_string()
            .unwrap();
        let quarter = get_quarter_from_directory(&quarter_name)?;
        debug!("Quarter: {:?}, Path: {}", quarter, quarter_path);
    }
    Ok(())
}

fn get_quarter_from_directory(dir_name: &String) -> Result<Quarter, ElevationError>
{
    if dir_name.len() != 1 { return Err(ElevationError::InvalidQuarterDirectorySpecifier); }
    let as_int = dir_name.parse::<u8>().unwrap();
    return match as_int {
        0 => { Ok(TopLeft) }
        1 => { Ok(TopRight) }
        2 => { Ok(BottomLeft) }
        3 => { Ok(BottomRight) }
        _ => { Err(ElevationError::InvalidQuarterDirectorySpecifier) }
    }
}