use std::fs;

pub fn get_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .flat_map(|line| line.split(','))
        .map(|s| s.to_string())
        .collect()
}
