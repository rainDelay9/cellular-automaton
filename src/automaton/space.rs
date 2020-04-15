pub use ndarray::{ArrayD, IxDyn};

pub struct Space {
    dims: Vec<usize>,
    space: ArrayD<u32>,
}

impl Space {
    pub fn new(_dims: Vec<usize>, _space: ArrayD<u32>) -> Self {
        Space {
            dims: _dims,
            space: _space,
        }
    }

    pub fn get_dims(&self) -> &[usize] {
        &self.dims[..]
    }

    pub fn get_point_value(&self, point: &[usize]) -> u32 {
        match self.space.get(IxDyn(point)) {
            Some(val) => return *val,
            None => panic!("cannot get point {:?}", point),
        }
    }

    pub fn set_point(&mut self, point: &[usize]) {
        self.space[IxDyn(point)] = 1;
    }
}
