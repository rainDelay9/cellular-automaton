use super::space::Space;
use convert_base::Convert;

pub fn get_neighborhood(space: &mut Space, point: &mut Vec<u32>) -> Vec<u32> {
    let mut converter = Convert::new(10, 3);
    let dims = space.get_dims();
    let num_of_dims = dims.len();
    let base: u32 = 3;
    let range = base.pow(num_of_dims as u32);
    let mut neighborhood: Vec<u32> = Vec::new();
    for num in 0..range {
        let mut offset = converter.convert::<u32, u32>(&vec![num]);
        let offset = pad_to_n_and_adjust(&mut offset, num_of_dims);
        let neighbor = add_points_on_taurus(point, &offset, &dims[..]);
        neighborhood.push(space.get_point_value(&neighbor[..]))
    }
    neighborhood
}

fn pad_to_n_and_adjust(vec: &mut Vec<u32>, n: usize) -> Vec<i32> {
    let mut res_vec: Vec<i32> = Vec::new();
    for i in 0..vec.len() {
        res_vec.push(vec[i] as i32 - 1);
    }
    while res_vec.len() < n {
        res_vec.push(-1);
    }
    res_vec.reverse();
    res_vec
}

fn add_points_on_taurus(point_a: &mut Vec<u32>, point_b: &Vec<i32>, dims: &[usize]) -> Vec<usize> {
    // check that all sizes are equal
    assert_eq!(point_a.len(), point_b.len());
    assert_eq!(point_a.len(), dims.len());

    let mut zipped = vec![0; point_a.len()];

    // add two input arrays in zip
    for (rref, val) in zipped
        .iter_mut()
        .zip(point_a.iter().zip(point_b).map(|(a, b)| *a as i32 + b))
    {
        *rref = val;
    }

    let mut result: Vec<usize> = Vec::new();
    // zip with dimension to mod result
    for (rref, val) in zipped.iter_mut().zip(dims.iter()) {
        let modulo = *val as i32;
        result.push(((*rref + modulo) % modulo) as usize);
    }
    result
}
