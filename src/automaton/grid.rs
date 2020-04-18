use crate::automaton::neighborhood::get_neighborhood;
use exitfailure::ExitFailure;
pub use ndarray::{iter::IndexedIter as Iterator, ArrayD, Dim, IxDyn};
use std::fmt;

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

    pub fn get_point_value(&self, point: &[usize]) -> Result<u32, ExitFailure> {
        if self.dims.len() != point.len() {
            return Err(ExitFailure::from(GridError::new(
                "point is of wrong dimensions!",
            )));
        }
        match self.grid.get(IxDyn(point)) {
            Some(val) => return Ok(*val),
            None => return Err(ExitFailure::from(GridError::new("point does not exist"))),
        }
    }

    pub fn neighborhood(&self, point: Vec<usize>) -> Result<Vec<u32>, ExitFailure> {
        if self.dims.len() != point.len() {
            return Err(ExitFailure::from(GridError::new(
                "neighborhood: base point is of wrong dimensions!",
            )));
        }
        Ok(get_neighborhood(&self, &point)?)
    }

    pub fn set_point(&mut self, point: &[usize], value: u32) -> Result<(), ExitFailure> {
        if self.dims.len() != point.len() {
            return Err(ExitFailure::from(GridError::new(
                "added point is of wrong dimensions!",
            )));
        }
        self.grid[IxDyn(point)] = value;
        Ok(())
    }

    pub fn iter(&self) -> Iterator<u32, IxDyn> {
        self.grid.indexed_iter()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

#[derive(Debug)]
struct GridError {
    cause: String,
}

impl GridError {
    pub fn new(cause: &str) -> Self {
        Self {
            cause: cause.to_string(),
        }
    }
}

impl fmt::Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "grid error! {}", self.cause)
    }
}

impl std::error::Error for GridError {}
