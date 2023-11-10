use crate::tiles::Preload;
use crate::tiles::Preload::PreloadTile;

mod tiles;

fn main() {
    let a = tiles::elevation(60.032398, 30.230101, PreloadTile);
    match a
    {
        Some(x) => println!("{}", x),
        None => println!("Error!")
    };
}
