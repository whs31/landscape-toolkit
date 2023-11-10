use lazy_static::lazy_static;
use std::sync::{Mutex};
use std::collections::HashMap;
use std::env;
use crate::tiles::tile::Tile;
use crate::tiles::tile_storage::tile_key::TileKey;

pub mod tile_key;

lazy_static!
{
    pub static ref STORAGE: Mutex<TileStorage> = Mutex::new(TileStorage::new());
}

pub struct TileStorage
{
    storage_path: String,
    storage: HashMap<TileKey, Tile>
}

impl TileStorage
{
    pub fn new() -> Self
    {
        gdal::DriverManager::register_all();
        TileStorage {
            storage_path: env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap() + "/elevations",
            storage: HashMap::new()
        }
    }

    pub fn load(&mut self, latitude: i8, longitude: i16) -> bool
    {
        let key = TileKey::from_int(latitude, longitude);
        let is_present = self.storage.contains_key(&key);
        if !is_present
        {
            // mutex
            if !self.storage.contains_key(&key)
            {
                self.storage.insert(key, Tile::new(&self.storage_path, latitude, longitude));
            }
        }

        return self.storage.contains_key(&key)
    }

    pub fn elevation(&self, latitude: f64, longitude: f64) -> Result<f32, &str>
    {
        let key = TileKey::from_f64(latitude, longitude);
        if !self.storage.contains_key(&key) { return Err("Tile not found"); }

        let a = self.storage.get(&key);
        let ret: f32 = match match a {
            Some(x) => x,
            None => return Err("Null tile in storage"),
        }.elevation(latitude, longitude) {
            Ok(x) => x,
            Err(err) => return Err(err),
        };

        Ok(ret)
    }
}

