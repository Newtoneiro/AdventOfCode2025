use std::fs::read_to_string;

pub fn get_input(filename: &str) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let mut row = Vec::new();
        for ch in line.chars() {
            if ch == '@' {
                row.push(1);
            } else {
                row.push(0);
            }
        }
        result.push(row);
    }

    result

}