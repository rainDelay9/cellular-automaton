pub fn string_to_coordinate(string: &String) -> Vec<usize> {
    let string_vec: Vec<usize> = (&string[1..string.len() - 1])
        .split(",")
        .map(|s| s.to_string().parse::<usize>().unwrap())
        .collect();
    string_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        string_to_coordinate(&String::from("(1,2,3)"));
    }
}
