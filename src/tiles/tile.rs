use std::mem;
use std::vec::Vec;
use gdal;
use gdal::raster::ResampleAlg;
use byteorder::{ByteOrder, LittleEndian};

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
        if dataset.is_err()
        {
            return ret
        }
        let adf_geo_transform = dataset
            .as_ref()
            .unwrap()
            .geo_transform()
            .unwrap();
        ret.latitude_size = adf_geo_transform[5];
        ret.longitude_size = adf_geo_transform[1];
        ret.tile_latitude = adf_geo_transform[3] + ret.latitude_size / 2.0;
        ret.tile_longitude = adf_geo_transform[0] + ret.longitude_size / 2.0;

        if latitude + 1 == ret.tile_latitude.floor() as i8
            && longitude == ret.tile_longitude.ceil() as i16
        {
            let raster_band = dataset
                .as_ref()
                .unwrap()
                .rasterband(1)
                .unwrap();
            ret.tile_size_x = raster_band.x_size() as i32;
            ret.tile_size_y = raster_band.y_size() as i32;

            let tile_data_size = ret.tile_size_x * ret.tile_size_y * mem::size_of::<u16>() as i32;
            ret.tile_data = vec![0x00; tile_data_size as usize];

            let raster_size = (ret.tile_size_x as usize, ret.tile_size_y as usize);
            let raster_io = raster_band.read_as::<u8>((0, 0), raster_size,
                                                      raster_size, Some(ResampleAlg::Bilinear));
            ret.tile_data = raster_io.unwrap().data;
        }

        ret
    }

    pub fn elevation(&self, latitude: f64, longitude: f64) -> Result<f32, &str>
    {
        if self.tile_data.is_empty() { return Err("Tile not found") }

        let index = ((latitude - self.tile_latitude) / self.latitude_size).round() as usize * self.tile_size_x as usize
                          + ((longitude - self.tile_longitude) / self.longitude_size).round() as usize * mem::size_of::<i16>();
        let slice = self.tile_data.get((index + 1)..(index + 1 + mem::size_of::<i16>()));
        if slice.is_none() { return Err("Failed to read data from tile buffer") }
        let ret = LittleEndian::read_i16(&slice.unwrap());

        Ok(ret as f32)
    }
}