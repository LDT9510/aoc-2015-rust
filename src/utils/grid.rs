use super::coord_2d::Coord;
use crate::utils::geom::{PositionalLine, PositionalRectangle};
use std::ops::{Index, IndexMut};

#[inline]
fn coord_to_grid_index(c: Coord, grid_width: usize) -> usize {
    (c.y as usize * grid_width) + c.x as usize
}

#[inline]
fn grid_index_to_coord(index: usize, grid_width: usize) -> Coord {
    Coord::new((index % grid_width) as i64, (index / grid_width) as i64)
}

pub struct GridElement<T> {
    pub position: Coord,
    pub value: T,
}

impl<T> GridElement<T> {
    fn new(index: usize, value: T, grid_width: usize) -> Self {
        GridElement {
            position: grid_index_to_coord(index, grid_width),
            value,
        }
    }
}

pub struct MutableGridElement<'grid, T> {
    pub position: Coord,
    pub value: &'grid mut T,
}

impl<'grid, T> MutableGridElement<'grid, T> {
    fn new(index: usize, value: &'grid mut T, grid_width: usize) -> Self {
        MutableGridElement {
            position: grid_index_to_coord(index, grid_width),
            value,
        }
    }
}

// TODO: this is a square grid, make it dimensionally generic
pub struct Grid<T: Default, const SIZE: usize> {
    _array: [T; SIZE],
    _width: usize,
}

impl<T: Default + Copy, const SIZE: usize> Grid<T, SIZE> {
    pub fn new_empty() -> Self {
        Grid {
            _array: [T::default(); SIZE],
            _width: SIZE.isqrt(),
        }
    }

    pub fn new_boxed() -> Box<Grid<T, SIZE>> {
        // this hack is necessary because Rust lacks placement new (creating a type directly on a
        // memory location), since Grid is stack allocated, a massive one may cause a stack
        // overflow, so this is required to construct it directly on the heap
        let layout = std::alloc::Layout::new::<Grid<T, SIZE>>();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut Grid<T, SIZE> };
        let mut grid = unsafe { Box::from_raw(ptr) };

        grid._array.fill(T::default());
        grid._width = SIZE.isqrt();

        grid
    }

    pub fn iter(&self) -> impl Iterator<Item = GridElement<T>> {
        self._array
            .iter()
            .enumerate()
            .map(|(i, value)| GridElement::new(i, *value, self._width))
    }

    pub fn iter_mut(&'_ mut self) -> impl Iterator<Item = MutableGridElement<'_, T>> {
        self._array
            .iter_mut()
            .enumerate()
            .map(|(i, value)| MutableGridElement::new(i, value, self._width))
    }

    pub fn apply_in_rectangle<F: FnMut(&mut T)>(&mut self, rect: PositionalRectangle, mut func: F) {
        for line in rect.iter_horizontal_lines() {
            self.apply_in_horizontal_line(line, &mut func);
        }
    }

    pub fn apply_in_horizontal_line<F: FnMut(&mut T)>(
        &mut self,
        line: PositionalLine,
        func: &mut F,
    ) {
        let begin_idx = coord_to_grid_index(line.start, self._width);
        let end_idx = coord_to_grid_index(line.end, self._width);
        self._array[begin_idx..=end_idx].iter_mut().for_each(func);
    }

    pub fn fill_in_rectangle(&mut self, rect: PositionalRectangle, value: T) {
        rect.iter_horizontal_lines()
            .for_each(|l| self.fill_in_horizontal_line(l, value));
    }

    pub fn fill_in_horizontal_line(&mut self, line: PositionalLine, value: T) {
        let begin_idx = coord_to_grid_index(line.start, self._width);
        let end_idx = coord_to_grid_index(line.end, self._width);
        self._array[begin_idx..=end_idx].fill(value);
    }
}

impl<T: Default + Copy, const SIZE: usize> Index<Coord> for Grid<T, SIZE> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self._array[coord_to_grid_index(index, self._width)]
    }
}

impl<T: Default + Copy, const SIZE: usize> IndexMut<Coord> for Grid<T, SIZE> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self._array[coord_to_grid_index(index, self._width)]
    }
}

impl<const SIZE: usize> Grid<bool, SIZE> {
    pub fn toggle_in_rectangle(&mut self, rect: PositionalRectangle) {
        rect.iter_horizontal_lines()
            .for_each(|l| self.toggle_in_horizontal_line(l));
    }

    pub fn toggle_in_horizontal_line(&mut self, line: PositionalLine) {
        let begin_idx = coord_to_grid_index(line.start, self._width);
        let end_idx = coord_to_grid_index(line.end, self._width);
        self._array[begin_idx..=end_idx]
            .iter_mut()
            .for_each(|b| *b = !*b);
    }

    pub fn count_on(&self) -> usize {
        self._array.iter().filter(|b| **b).count()
    }
    pub fn count_off(&self) -> usize {
        self._array.iter().filter(|b| !**b).count()
    }
}
