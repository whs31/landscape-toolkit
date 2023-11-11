use crate::elevation::ElevationError;

#[derive(Debug, PartialEq)]
pub enum Quarter
{
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3
}

pub fn quarter_from_directory(dir_name: &String) -> Result<Quarter, ElevationError>
{
    if dir_name.len() != 1 { return Err(ElevationError::InvalidQuarterDirectorySpecifier); }
    let as_int = dir_name.parse::<u8>().unwrap();
    match as_int {
        0 => { Ok(Quarter::TopLeft) }
        1 => { Ok(Quarter::TopRight) }
        2 => { Ok(Quarter::BottomLeft) }
        3 => { Ok(Quarter::BottomRight) }
        _ => { Err(ElevationError::InvalidQuarterDirectorySpecifier) }
    }
}

pub fn quarter_signs(quarter: &Quarter) -> (i8, i16)
{
    match quarter {
        Quarter::TopLeft => { (1, -1) }
        Quarter::TopRight => { (1, 1) }
        Quarter::BottomLeft => { (-1, -1) }
        Quarter::BottomRight => { (-1, 1) }
    }
}