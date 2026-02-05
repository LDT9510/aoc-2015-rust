use anyhow::Context;
use std::str::FromStr;

pub struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Self {
        Coord { x, y }
    }
}

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let comma_idx = s.find(',').context("Bad format, comma not found.")?;
        let (x_str, y_str) = s.split_at(comma_idx);
        let x = x_str.parse()?;
        // we need to skip the comma
        let y = y_str[1..].parse()?;
        Ok(Coord::new(x, y))
    }
}
