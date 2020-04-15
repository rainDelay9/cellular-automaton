pub use ndarray::{ArrayD, IxDyn};

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

    pub fn get_dims(&self) -> &[usize] {
        &self.dims[..]
    }

    pub fn get_point_value(&self, point: &[usize]) -> u32 {
        match self.grid.get(IxDyn(point)) {
            Some(val) => return *val,
            None => panic!("cannot get point {:?}", point),
        }
    }

    pub fn set_point(&mut self, point: &[usize]) {
        self.grid[IxDyn(point)] = 1;
    }
}
