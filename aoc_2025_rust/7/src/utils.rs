use std::fs::read_to_string;

pub fn get_input(filename: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let row: Vec<char> = line.to_string().chars().collect();
        result.push(row);
    }

    result
}