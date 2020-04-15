use ndarray::{ArrayD, Dim, IxDyn, NdIndex};

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

    pub fn get_point(&self, point: &[usize]) -> u32 {
        match self.space.get(IxDyn(point)) {
            Some(val) => return *val,
            // None => panic!("could not get point!"),
            None => println!("cannot get point {:?}", point),
        }
        0
    }
}
