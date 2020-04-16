use crate::automaton::neighborhood::get_neighborhood;
pub use ndarray::{iter::IndexedIter as Iterator, ArrayD, Dim, IxDyn};

#[derive(Debug, Clone)]
pub struct Grid {
    dims: Vec<usize>,
    grid: ArrayD<u32>,
}

impl Grid {
    pub fn new(_dims: Vec<usize>, _grid: ArrayD<u32>) -> Self {
        Self {
            dims: _dims,
            grid: _grid,
        }
    }

    pub fn dims(&self) -> &[usize] {
        &self.dims[..]
    }

    pub fn grid(&self) -> ArrayD<u32> {
        self.grid.clone()
    }

    pub fn get_point_value(&self, point: &[usize]) -> u32 {
        match self.grid.get(IxDyn(point)) {
            Some(val) => return *val,
            None => panic!("cannot get point {:?}", point),
        }
    }

    pub fn neighborhood(&self, point: Vec<usize>) -> Vec<u32> {
        get_neighborhood(&self, &point)
    }

    pub fn set_point(&mut self, point: &[usize], value: u32) {
        self.grid[IxDyn(point)] = value;
    }

    pub fn iter(&self) -> Iterator<u32, IxDyn> {
        self.grid.indexed_iter()
    }
}
