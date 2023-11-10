use std::vec::Vec;
use gdal;

pub struct Tile
{
    tile_latitude: f64,
    tile_longitude: f64,
    latitude_size: f64,
    longitude_size: f64,
    tile_size_x: i32,
    tile_size_y: i32,
    tile_data: Vec<u8>
}

impl Tile
{
    pub fn new(path: &String, latitude: i8, longitude: i16) -> Self
    {
        let mut ret = Tile {
            tile_latitude: 0.0,
            tile_longitude: 0.0,
            latitude_size: 0.0,
            longitude_size: 0.0,
            tile_size_x: 0,
            tile_size_y: 0,
            tile_data: vec!()
        };

        let mut f = path.clone() + "/";
        if latitude >= 0
        {
            if longitude < 0 { f += "0/"; }
            else { f += "1/"; }
        }
        else
        {
            if longitude < 0 { f += "2/"; }
            else { f+= "3/"; }
        }
        f += format!("{}/{}.tif", latitude.abs(), longitude.abs()).as_str();

        let dataset = gdal::Dataset::open(f);


        ret
    }

    // pub fn elevation(&self, latitude: f64, longitude: f64) -> Result<f32, &str>
    // {
    //
    // }
}