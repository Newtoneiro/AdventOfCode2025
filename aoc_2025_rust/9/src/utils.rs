use std::fs::read_to_string;

pub fn get_input(filename: &str) -> Vec<(u64, u64)> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let mut parts = line.split(',').map(|s| s.trim().parse::<u64>().unwrap());

        let a = parts.next().unwrap();
        let b = parts.next().unwrap();

        result.push((a, b));
    }

    result
}