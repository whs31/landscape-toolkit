use std::{env, fs};
use log::{debug, info, warn};
use crate::elevation::{ElevationError, utils};
use crate::elevation::fs_identity::FSObjectIdentity;
use crate::elevation::quarter::{quarter_from_directory, quarter_signs};
use crate::elevation::tile_storage::STORAGE;
use crate::elevation::tile_storage::tile_key::TileKey;

pub fn scan_directory(directory: &String) -> Result<(), ElevationError>
{
    info!("Scanning absolute directory: {}", &directory);
    let paths = match fs::read_dir(directory) {
        Ok(x) => x,
        Err(_) => { return Err(ElevationError::InvalidDirectoryHierarchy) }
    };
    for path in paths
    {
        let quarter_identity = FSObjectIdentity::from_dir_entry(path.as_ref().unwrap());
        let quarter = quarter_from_directory(&quarter_identity.name)?;
        debug!("Quarter directory: {:?}, Path: {}", quarter, &quarter_identity.path);

        let q_dir = fs::read_dir(&quarter_identity.path).unwrap();
        for q_path in q_dir
        {
            let latitude_identity = FSObjectIdentity::from_dir_entry(&q_path.as_ref().unwrap());
            //debug!("Latitude directory: {}", &latitude_identity.name);

            let lat_dir = match fs::read_dir(&latitude_identity.path) {
                Ok(x) => x,
                Err(_) => { return Err(ElevationError::InvalidDirectoryHierarchy) }
            };
            for lat_path in lat_dir
            {
                let longitude_identity = FSObjectIdentity::from_dir_entry(&lat_path.as_ref().unwrap());
                let signs = quarter_signs(&quarter);
                let lon_trimmed = utils::trim_longitude_path(longitude_identity.name);
                let coords: (i8, i16) = (latitude_identity.name.parse::<i8>().unwrap() * signs.0,
                                         lon_trimmed? * signs.1);
                debug!("Coordinate pair found: {:?}", coords);
                match STORAGE
                    .lock()
                    .unwrap()
                    .make_available(TileKey::from_int(coords.0, coords.1), longitude_identity.path) {
                    Ok(_) => { () }
                    Err(x) => { warn!("{:?}", x) }
                }
            }
        }
    }

    STORAGE.lock().unwrap().set_dir(directory.clone())?;
    Ok(())
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

    scan_directory(&abs_path)
}