use std::{env, fs};
use std::fs::DirEntry;
use log::{debug, info};
use crate::elevation::Quarter::{BottomLeft, BottomRight, TopLeft, TopRight};

#[derive(Debug)]
pub enum ElevationError
{
    InvalidQuarterDirectorySpecifier
    //UnknownError
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
        let quarter_identity = FSObjectIdentity::from_dir_entry(path.as_ref().unwrap());
        let quarter = get_quarter_from_directory(&quarter_identity.name)?;
        debug!("Quarter directory: {:?}, Path: {}", quarter, &quarter_identity.path);

        let q_dir = fs::read_dir(&quarter_identity.path).unwrap();
        for q_path in q_dir
        {
            let latitude_identity = FSObjectIdentity::from_dir_entry(&q_path.as_ref().unwrap());
            debug!("Latitude directory: {}", latitude_identity.name);
        }
    }
    Ok(())
}

struct FSObjectIdentity
{
    name: String,
    path: String
}

impl FSObjectIdentity
{
    fn from_dir_entry(entry: &DirEntry) -> Self
    {
        FSObjectIdentity {
            name: entry.file_name().into_string().unwrap(),
            path: entry.path().into_os_string().into_string().unwrap()
        }
    }
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