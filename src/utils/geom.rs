use crate::utils::coord_2d::Coord;

pub struct PositionalLine {
    pub start: Coord,
    pub end: Coord,
}

impl PositionalLine {
    pub fn new(start: Coord, end: Coord) -> Self {
        PositionalLine { start, end }
    }
}

pub struct PositionalRectangle {
    pub top_corner: Coord,
    pub bottom_corner: Coord,
}

impl PositionalRectangle {
    pub fn new(top_corner: Coord, bottom_corner: Coord) -> Self {
        PositionalRectangle {
            top_corner,
            bottom_corner,
        }
    }

    pub fn area(&self) -> usize {
        self.length() * self.width()
    }
    
    pub fn intersection_area(&self, _other: PositionalRectangle) -> Option<usize> {
        todo!()
    }

    pub fn intersects_with(&self, _other: PositionalRectangle) -> bool {
        todo!()
    }

    pub fn length(&self) -> usize {
        ((self.top_corner.x - self.bottom_corner.x).abs() + 1) as usize
    }

    pub fn width(&self) -> usize {
        ((self.top_corner.y - self.bottom_corner.y).abs() + 1) as usize
    }

    pub fn iter_horizontal_lines(&self) -> impl Iterator<Item = PositionalLine> {
        (self.top_corner.y..=self.bottom_corner.y).map(|delta_y| {
            let start = Coord::new(self.top_corner.x, delta_y);
            let end = Coord::new(self.bottom_corner.x, delta_y);
            PositionalLine::new(start, end)
        })
    }
}
