use std::{env, fs};

#[derive(Debug)]
pub enum ElevationError
{
    UnknownError
}

pub fn init()
{
    pretty_env_logger::init();
}

pub fn scan_relative_directory(relative_directory: &str) -> Result<(), ElevationError>
{
    let abs_path = format!(
        "{}/{}", env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
        relative_directory
    );

    return scan_directory(&abs_path)
}

pub fn scan_directory(directory: &String) -> Result<(), ElevationError>
{
    Ok(())
}