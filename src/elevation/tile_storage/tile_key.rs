use crate::elevation::quarter::Quarter;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct TileKey
{
    pub latitude: i8,
    pub longitude: i16
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

    pub fn quarter(&self) -> Quarter
    {
        if self.latitude >= 0 && self.longitude < 0 { return Quarter::TopLeft }
        if self.latitude >= 0 && self.longitude >= 0 { return Quarter::TopRight }
        if self.latitude < 0 && self.longitude < 0 { return Quarter::BottomLeft }

        Quarter::BottomRight
    }

    pub fn quarter_as_u8(&self) -> u8
    {
        match self.quarter() {
            Quarter::TopLeft => 0,
            Quarter::TopRight => 1,
            Quarter::BottomLeft => 2,
            Quarter::BottomRight => 3
        }
    }
}

