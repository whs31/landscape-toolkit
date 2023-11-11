use log::{trace, warn};
use nav_types::WGS84;
use crate::elevation::ElevationError;
use crate::elevation::tile_storage::STORAGE;
use crate::elevation::tile_storage::tile_key::TileKey;

pub fn elevation_at(coordinate: (f64, f64)) -> Result<f32, ElevationError>
{
    let fl = (coordinate.0.floor() as i16, coordinate.1.floor() as i16);
    let tile_size: (usize, usize) = (WGS84::from_degrees_and_meters((fl.0) as f64, (fl.1) as f64, 0.0)
                                         .distance(&WGS84::from_degrees_and_meters((fl.0 + 1) as f64, (fl.1) as f64, 0.0)) as usize,
                                     WGS84::from_degrees_and_meters((fl.0) as f64, (fl.1) as f64, 0.0)
                                         .distance(&WGS84::from_degrees_and_meters((fl.0) as f64, (fl.1 + 1) as f64, 0.0)) as usize);
    let path = match STORAGE
        .lock()
        .unwrap()
        .get(TileKey::from_int(fl.0 as i8, fl.1)) {
        Ok(x) => x,
        Err(e) => { warn!("No such tile present in storage: {:?}, error code: {:?}", fl, e); return Err(e); }
    };
    let sz = match imagesize::size(&path) {
        Ok(x) => x,
        Err(_) => { warn!("Failed to fetch image size"); return Err(ElevationError::ImageSizeError); }
    };
    let image_size: (usize, usize) = (sz.width, sz.height);
    trace!("Image size: {:?}", &image_size);

    let request_coord = WGS84::from_degrees_and_meters(coordinate.0, coordinate.1, 0.0f64);
    let distance_2d = (request_coord.distance(&WGS84::from_degrees_and_meters(coordinate.0, (fl.1) as f64, 0.0)),
                                request_coord.distance(&WGS84::from_degrees_and_meters((fl.0) as f64, coordinate.1, 0.0)));

    let distance_normalized = (distance_2d.0 / (tile_size.1 as f64), distance_2d.1 / (tile_size.0 as f64));
    trace!("Image normalized distance from BL corner: {:?}", &distance_normalized);

    let pixel_coords = ((distance_normalized.0 * image_size.0 as f64) as usize, (distance_normalized.1 * image_size.1 as f64) as usize);
    trace!("Pixel coordinates: {:?}", &pixel_coords);

    let value = STORAGE
        .lock()
        .unwrap()
        .get_tiff(TileKey::from_f64(coordinate.0, coordinate.1))?
        .get_pixel(pixel_coords.1, pixel_coords.0);

    Ok(value as f32)
}