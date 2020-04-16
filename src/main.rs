// pub use cellular_automaton_prg::automaton;
// pub use cellular_automaton_prg::automaton::rules::parsers::parse_rules as parse;
// pub use cellular_automaton_prg::automaton::rules::Rules;
// pub use cellular_automaton_prg::automaton::Automaton;
// pub use cellular_automaton_prg::automaton::{grid::Grid, neighborhood};
// pub use cellular_automaton_prg::utils::coordinates_counter;
// pub use exitfailure::ExitFailure;
// use ndarray::{ArrayD, IxDyn};
// use std::fs;

// fn main() -> Result<(), ExitFailure> {
//     // let path = std::path::Path::new("src/config/rule110.json");

//     // let data = fs::read_to_string(path).expect("Unable to read file");

//     // let data: types::OneDimRules = parse(&data)?;

//     // for rule in data.rules {
//     //     println!("{:?}", rule.neighborhood);
//     // }

//     // Create a three-dimensional f64 array, initialized with zeros
//     let mut dims = Vec::new();
//     dims.push(5);
//     dims.push(5);
//     dims.push(5);
//     dims.push(5);

//     let mut temperature: ArrayD<u32> = ArrayD::zeros(IxDyn(&dims));
//     temperature[[4, 0, 1, 2]] += 1;
//     temperature[[0, 0, 1, 2]] += 2;
//     temperature[[1, 0, 1, 2]] += 3;
//     temperature[[4, 1, 1, 2]] += 4;
//     temperature[[0, 1, 1, 2]] += 5;
//     temperature[[1, 1, 1, 2]] += 6;
//     temperature[[4, 2, 1, 2]] += 7;
//     temperature[[0, 2, 1, 2]] += 8;
//     temperature[[1, 2, 1, 2]] += 9;

//     println!("{:?}\n", temperature);

//     let mut grid = Grid::new(dims.clone(), temperature);

//     let path = std::path::Path::new("src/config/rule110.json");

//     let data = fs::read_to_string(path).expect("Unable to read file");

//     // let mut config_schema = parse(&data)?;
//     // let mut rules =
//     // let automaton = Automaton::new(grid, );
//     let mut point = vec![0, 1, 1, 2];

//     let neighborhood = neighborhood::get_neighborhood(&mut grid, &mut point);
//     println!("neighborhood: {:?}", neighborhood);

//     // Increase the temperature in this location
//     // let point = [2, 1];
//     //temperature[point] += 1;
//     // temperature[[0, 1]] += 1;

//     //println!("{:?}\n", temperature);

//     // let mut base = Convert::new(10, 3);
//     // for num in 0..9 {
//     //     let mut vec = base.convert::<u32, u32>(&vec![num]);
//     //     let vec = pad_to_3(&mut vec);
//     //     println!("converted: {:?}", vec);
//     // }

//     // let mut a = vec![1, 2, 3, 4];
//     // let mut b = vec![-2, 2, -1, 2];
//     // let mut dims = [5, 5, 5, 5];

//     // println!("result: {:?}", add_points(&mut a, &b, &dims[..]));

//     // fn add(a: Coord, b: Coord) -> Coord {
//     //     new_coord_from(a.iter().zip(&b).map(|(a, b)| a + b))
//     // }

//     Ok(())
// }

use cellular_automaton_prg::utils::coordinate_utils::string_to_coordinate;

fn main() {
    println!("{:?}", string_to_coordinate(&String::from("(1,2,3)")));
}
