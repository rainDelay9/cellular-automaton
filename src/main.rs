pub use cellular_automaton_prg::automaton::rules::parsers::{parse_rules as parse, types};
pub use cellular_automaton_prg::automaton::{neighborhood, space::Space};
use convert_base::Convert;
pub use exitfailure::ExitFailure;
use ndarray::{ArrayD, IxDyn};
use std::fs;

fn main() -> Result<(), ExitFailure> {
    // let path = std::path::Path::new("src/config/rule110.json");

    // let data = fs::read_to_string(path).expect("Unable to read file");

    // let data: types::OneDimRules = parse(&data)?;

    // for rule in data.rules {
    //     println!("{:?}", rule.neighborhood);
    // }

    // Create a three-dimensional f64 array, initialized with zeros
    let mut dims = Vec::new();
    dims.push(5);
    dims.push(5);

    let mut temperature: ArrayD<u32> = ArrayD::zeros(IxDyn(&dims));
    temperature[[4, 0]] += 1;
    temperature[[0, 0]] += 2;
    temperature[[1, 0]] += 3;
    temperature[[4, 1]] += 4;
    temperature[[0, 1]] += 5;
    temperature[[1, 1]] += 6;
    temperature[[4, 2]] += 7;
    temperature[[0, 2]] += 8;
    temperature[[1, 2]] += 9;

    println!("{:?}\n", temperature);

    let mut space = Space::new(dims.clone(), temperature);
    let mut point = vec![0, 1];

    let neighborhood = neighborhood::get_neighborhood(&mut space, &mut point);
    println!("neighborhood: {:?}", neighborhood);

    // Increase the temperature in this location
    // let point = [2, 1];
    //temperature[point] += 1;
    // temperature[[0, 1]] += 1;

    //println!("{:?}\n", temperature);

    // let mut base = Convert::new(10, 3);
    // for num in 0..9 {
    //     let mut vec = base.convert::<u32, u32>(&vec![num]);
    //     let vec = pad_to_3(&mut vec);
    //     println!("converted: {:?}", vec);
    // }

    let mut a = vec![1, 2, 3, 4];
    let mut b = vec![-2, 2, -1, 2];
    let mut dims = [5, 5, 5, 5];

    println!("result: {:?}", add_points(&mut a, &b, &dims[..]));

    // fn add(a: Coord, b: Coord) -> Coord {
    //     new_coord_from(a.iter().zip(&b).map(|(a, b)| a + b))
    // }

    Ok(())
}

pub fn add_points(a: &mut Vec<u32>, b: &Vec<i32>, dims: &[u32]) -> Vec<i32> {
    let mut result = vec![0; a.len()];
    for (rref, val) in result
        .iter_mut()
        .zip(a.iter().zip(b).map(|(a, b)| *a as i32 + b))
    {
        *rref = val;
    }
    for (rref, val) in result.iter_mut().zip(dims.iter()) {
        *rref = (*rref + *val as i32) % *val as i32;
    }
    result
}

fn pad_to_3(vec: &mut Vec<u32>) -> Vec<i32> {
    let mut res_vec: Vec<i32> = Vec::new();
    for i in 0..vec.len() {
        res_vec.push(vec[i] as i32 - 1);
    }
    while res_vec.len() < 3 {
        res_vec.push(-1);
    }
    res_vec
}
