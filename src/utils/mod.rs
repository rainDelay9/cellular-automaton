pub mod coordinates_iterator;

use std::str::FromStr;

//transforms a string representation of a vector to the actual vector
pub fn string_to_vector<T>(string: &String) -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let res: Vec<T> = (&string[1..string.len() - 1])
        .split(",")
        .map(|s| s.to_string().parse::<T>().unwrap())
        .collect();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        string_to_vector::<usize>(&String::from("(1,2,3)"));
    }
}
