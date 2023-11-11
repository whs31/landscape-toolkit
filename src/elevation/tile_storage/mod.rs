pub mod tile_key;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use geotiff_rs::GeoTiff;
use tile_key::TileKey;
use lazy_static::lazy_static;
use log::warn;

use crate::elevation::ElevationError;

lazy_static!
{
    pub static ref STORAGE: Mutex<TileStorage> = Mutex::new(TileStorage::new());
}

pub struct TileStorage
{
    directory_path: String,
    available: HashMap<TileKey, String>,
    storage: HashMap<TileKey, GeoTiff>
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
            available: HashMap::new(),
            storage: HashMap::new()
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
        let _ = self.load(key, path.clone())?;
        match self.available.insert(key, path) {
            None => { Ok(()) }
            Some(_) => { Err(ElevationError::KeyAlreadyAvailable) }
        }
    }

    pub fn is_available(&self, key: TileKey) -> bool { self.available.contains_key(&key) }
    pub fn get(&self, key: TileKey) -> Result<String, ElevationError>
    {
        match self.is_available(key) {
            true => { Ok(self.available.get(&key).unwrap().clone()) },
            false => { Err(ElevationError::NoSuchTile) }
        }
    }

    pub fn get_tiff(&self, key: TileKey) -> Result<&GeoTiff, ElevationError>
    {
        match self.is_available(key) {
            true => { Ok(self.storage.get(&key).unwrap()) },
            false => { Err(ElevationError::NoSuchTile) }
        }
    }

    fn load(&mut self, key: TileKey, path: String) -> Result<(), ElevationError>
    {
        match self.storage.insert(key, match GeoTiff::from_file(&path) {
            Ok(x) => x,
            Err(_) => { warn!("Failed to open tiff file: {}", &path); return Err(ElevationError::LibraryError); },
        }) {
            None => { Ok(()) }
            Some(_) => { Err(ElevationError::KeyAlreadyAvailable) }
        }
    }
}