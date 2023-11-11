pub mod tile_key;

use std::collections::HashMap;
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
    available: HashMap<TileKey, String>
}

impl TileStorage
{
    fn new() -> Self
    {
        TileStorage {
            directory_path: std::env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
            available: HashMap::new()
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

    pub fn make_available(&mut self, key: TileKey, path: String) -> Result<(), ElevationError>
    {
        match self.available.insert(key, path) {
            None => { Ok(()) }
            Some(_) => { Err(ElevationError::KeyAlreadyAvailable) }
        }
    }

    pub fn is_available(&self, key: TileKey) -> bool { self.available.contains_key(&key) }
    pub fn get(&self, key: TileKey) -> Result<String, ElevationError>
    {
        match self.is_available(key) {
            true => { Ok(self.available.get(&key).unwrap().clone()) }
            false => { Err(ElevationError::NoSuchTile) }
        }
    }
}