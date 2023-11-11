pub mod tile_key;

use std::collections::HashSet;
use std::path::Path;
use std::sync::Mutex;
use tile_key::TileKey;
use lazy_static::lazy_static;

use crate::elevation::ElevationError;

lazy_static!
{
    pub static ref STORAGE: Mutex<TileStorage> = Mutex::new(TileStorage::new());
}

pub struct TileStorage
{
    directory_path: String,
    available: HashSet<TileKey>
}

impl TileStorage
{
    pub fn new() -> Self
    {
        TileStorage {
            directory_path: std::env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
            available: HashSet::new()
        }
    }

    pub fn set_dir(&mut self, path: String) -> Result<(), ElevationError>
    {
        let exists = Path::new(&path).exists();
        match exists {
            true => {
                self.directory_path = path;
                Ok(())
            }
            false => {
                Err(ElevationError::InvalidDirectoryPath)
            }
        }
    }

    pub fn make_available(&mut self, key: TileKey) -> Result<(), ElevationError>
    {
        match self.available.insert(key) {
            true => { Ok(()) }
            false => { Err(ElevationError::KeyAlreadyAvailable) }
        }
    }

    pub fn is_available(&self, key: TileKey) -> bool { self.available.contains(&key) }
}