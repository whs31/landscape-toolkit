use crate::tiles::Preload;
use crate::tiles::Preload::PreloadTile;

mod tiles;

fn main() {
    let a = tiles::elevation(60.0, 30.0, PreloadTile);
    match a
    {
        Some(x) => println!("{}", x),
        None => println!("Error!")
    };
}
