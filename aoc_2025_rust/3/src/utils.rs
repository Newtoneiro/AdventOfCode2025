use std::fs::read_to_string;

pub fn get_input(filename: &str) -> Vec<Vec<u32>> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let mut row = Vec::<u32>::new();
        for ch in line.to_string().chars() {
            row.push(ch.to_digit(10).unwrap())
        }
        result.push(row)
    }

    result
}