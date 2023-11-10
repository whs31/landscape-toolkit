mod tile;
mod tile_storage;

#[derive(PartialEq)]
pub enum Preload
{
    PreloadTile,
    NoPreload
}

pub fn load_tile(latitude: i8, longitude: i16) -> bool
{
    tile_storage::STORAGE.lock().unwrap().load(latitude, longitude)
}

pub fn load_rectangle(min_lat: i8, min_lon: i16, max_lat: i8, max_lon: i16) -> bool
{
    let mut ret = true;
    for lat in min_lat..=max_lat
    {
        for lon in min_lon..=max_lon { ret &= load_tile(lat, lon); }
    }

    return ret
}

pub fn elevation(latitude: f64, longitude: f64, preload: Preload) -> Option<f32>
{
    const COMP_FACTOR: f64 = 0.00001;
    let mut lat = latitude;
    let mut lon = longitude;
    if latitude - latitude.floor() < COMP_FACTOR { lat = latitude.floor(); }
    if latitude.ceil() - latitude < COMP_FACTOR { lat = latitude.ceil(); }
    if longitude - longitude.floor() < COMP_FACTOR { lon = longitude.floor(); }
    if longitude.ceil() - longitude < COMP_FACTOR { lon = longitude.ceil(); }

    if preload == Preload::PreloadTile { load_tile(lat as i8, lon as i16); }
    let binding = tile_storage::STORAGE.lock().unwrap();
    let elevation =  binding.elevation(lat, lon);
    if elevation.is_err()
    {
        None
    }
    else
    {
        Some(elevation.unwrap())
    }
}

