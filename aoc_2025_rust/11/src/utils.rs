use std::fs::read_to_string;
use std::collections::HashMap;

pub fn get_input(filename: &str) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::<String, Vec<String>>::new();

    for line in read_to_string(filename).unwrap().lines() {
        let line = line.to_string(); // keep it alive
        let mut parts = line.split(':');

        let key = parts.next().unwrap().trim().to_string();
        let vals: Vec<String> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|tok| tok.to_string())
            .collect();

        result.insert(key, vals);
    }

    result
}