#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct TileKey
{
    latitude: i8,
    longitude: i16
}

impl TileKey
{
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        TileKey {
            latitude: 0,
            longitude: 0
        }
    }

    pub fn from_int(latitude: i8, longitude: i16) -> Self
    {
        TileKey {
            latitude,
            longitude
        }
    }

    pub fn from_f64(latitude: f64, longitude: f64) -> Self
    {
        TileKey {
            latitude: latitude.floor() as i8,
            longitude: longitude.floor() as i16
        }
    }
}

