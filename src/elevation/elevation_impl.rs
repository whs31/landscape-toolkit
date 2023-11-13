use std::path::MAIN_SEPARATOR;
use std::sync::{MutexGuard};
use log::{debug, trace, warn};
use nav_types::WGS84;
use num_derive::FromPrimitive;
use crate::elevation::ElevationError;
use crate::elevation::tile_storage::{STORAGE, TileStorage};
use crate::elevation::tile_storage::tile_key::TileKey;
use crate::elevation::utils::FILE_EXTENSION;

#[derive(FromPrimitive)]
pub enum PreloadMode
{
    PreloadTile = 1,
    NoPreload = 2
}

pub fn elevation_at(coordinate: (f64, f64), preload_mode: PreloadMode) -> Result<f32, ElevationError>
{
    match preload_mode {
        PreloadMode::PreloadTile => elevation_at_preloaded(coordinate),
        PreloadMode::NoPreload => elevation_at_no_preload(coordinate)
    }
}

fn elevation_at_no_preload(coordinate: (f64, f64)) -> Result<f32, ElevationError>
{
    let lock = STORAGE.lock().unwrap();
    let fl = (coordinate.0.floor() as i16, coordinate.1.floor() as i16);
    let tile_size: (usize, usize) = (WGS84::from_degrees_and_meters((fl.0) as f64, (fl.1) as f64, 0.0)
                                         .distance(&WGS84::from_degrees_and_meters((fl.0 + 1) as f64, (fl.1) as f64, 0.0)) as usize,
                                     WGS84::from_degrees_and_meters((fl.0) as f64, (fl.1) as f64, 0.0)
                                         .distance(&WGS84::from_degrees_and_meters((fl.0) as f64, (fl.1 + 1) as f64, 0.0)) as usize);
    let path = match lock.get(TileKey::from_int(fl.0 as i8, fl.1)) {
        Ok(x) => x,
        Err(e) => {
            warn!("No such tile present in storage: {:?}, error code: {:?}", fl, e);
            return Err(e);
        }
    };
    let sz = match imagesize::size(&path) {
        Ok(x) => x,
        Err(_) => {
            warn!("Failed to fetch image size");
            return Err(ElevationError::ImageSizeError);
        }
    };
    let image_size: (usize, usize) = (sz.width, sz.height);

    let request_coord = WGS84::from_degrees_and_meters(coordinate.0, coordinate.1, 0.0f64);
    let distance_2d = (request_coord.distance(&WGS84::from_degrees_and_meters(coordinate.0, (fl.1) as f64, 0.0)),
                       request_coord.distance(&WGS84::from_degrees_and_meters((fl.0) as f64, coordinate.1, 0.0)));

    let distance_normalized = (distance_2d.0 / (tile_size.1 as f64), distance_2d.1 / (tile_size.0 as f64));

    let pixel_coords = ((distance_normalized.0 * image_size.0 as f64) as usize, (distance_normalized.1 * image_size.1 as f64) as usize);
    trace!("Pixel coordinates: {:?}", &pixel_coords);

    let value = lock
        .get_tiff(TileKey::from_f64(coordinate.0, coordinate.1))?
        .get_pixel(pixel_coords.1, pixel_coords.0);

    debug!("Elevation at {:?}: {} meters", coordinate, value);

    Ok(value as f32)
}

fn elevation_at_preloaded(coordinate: (f64, f64)) -> Result<f32, ElevationError>
{
    let mut lock = STORAGE.lock().unwrap();
    let fl = (coordinate.0.floor() as i16, coordinate.1.floor() as i16);
    let key = TileKey::from_int(fl.0 as i8, fl.1);
    let path = match lock.is_available(key) {
        true => lock.get(key).unwrap(),
        false => load_tile(key, &mut lock)?
    };
    let tile_size: (usize, usize) = (WGS84::from_degrees_and_meters((fl.0) as f64, (fl.1) as f64, 0.0)
                                         .distance(&WGS84::from_degrees_and_meters((fl.0 + 1) as f64, (fl.1) as f64, 0.0)) as usize,
                                     WGS84::from_degrees_and_meters((fl.0) as f64, (fl.1) as f64, 0.0)
                                         .distance(&WGS84::from_degrees_and_meters((fl.0) as f64, (fl.1 + 1) as f64, 0.0)) as usize);

    let sz = match imagesize::size(&path) {
        Ok(x) => x,
        Err(_) => {
            warn!("Failed to fetch image size");
            return Err(ElevationError::ImageSizeError);
        }
    };
    let image_size: (usize, usize) = (sz.width, sz.height);

    let request_coord = WGS84::from_degrees_and_meters(coordinate.0, coordinate.1, 0.0f64);
    let distance_2d = (request_coord.distance(&WGS84::from_degrees_and_meters(coordinate.0, (fl.1) as f64, 0.0)),
                       request_coord.distance(&WGS84::from_degrees_and_meters((fl.0) as f64, coordinate.1, 0.0)));

    let distance_normalized = (distance_2d.0 / (tile_size.1 as f64), distance_2d.1 / (tile_size.0 as f64));

    let pixel_coords = ((distance_normalized.0 * image_size.0 as f64) as usize, (distance_normalized.1 * image_size.1 as f64) as usize);
    trace!("Pixel coordinates: {:?}", &pixel_coords);

    let value = lock
        .get_tiff(TileKey::from_f64(coordinate.0, coordinate.1))?
        .get_pixel(pixel_coords.1, pixel_coords.0);

    debug!("Elevation at {:?}: {} meters", coordinate, value);

    Ok(value as f32)
}

fn load_tile(key: TileKey, lock: &mut MutexGuard<TileStorage>) -> Result<String, ElevationError>
{
    debug!("Loading tile from key {:?}", key.clone());
    let top_level = lock
        .directory_path
        .clone();
    let quarter = key
        .quarter_as_u8()
        .to_string();
    let path = format!("{}{}{}{}{}{}{}.{}", top_level, MAIN_SEPARATOR,
                       quarter, MAIN_SEPARATOR, key.latitude.abs(), MAIN_SEPARATOR,
                       key.longitude.abs(), FILE_EXTENSION);
    debug!("Searching path for tile {:?} is {}", key.clone(), &path);
    match lock.make_available(key, path) {
        Ok(_) => {},
        Err(e) => { return Err(e) }
    };
    
    lock.get(key)
}