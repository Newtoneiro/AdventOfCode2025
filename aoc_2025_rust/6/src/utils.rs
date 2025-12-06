use std::fs::read_to_string;

pub fn get_input(filename: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut matrix = Vec::<Vec<u64>>::new();

    let lines: Vec<String> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    for line in &lines[..lines.len() - 1] {
        let part: Vec<u64> = line
            .split_whitespace()
            .filter_map(|tok| tok.parse::<u64>().ok())
            .collect();
        matrix.push(part);
    }
    let operations: Vec<char> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .filter_map(|tok| tok.parse::<char>().ok())
        .collect();

    (matrix, operations)
}

pub fn get_input_2(filename: &str) -> (Vec<Vec<String>>, Vec<char>) {
    let mut matrix = Vec::<Vec<String>>::new();
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
    let mut operations = Vec::<char>::new();
    let mut gaps = Vec::new();
    
    let mut in_gap = false;
    let mut gap_count: u64 = 0;
    
    for c in lines.last().unwrap().chars() {
        match c {
            '+' | '*' => {
                operations.push(c);
                if in_gap {
                    gaps.push(gap_count);
                    gap_count = 0;
                    in_gap = false;
                }
            }
            ' ' => {
                in_gap = true;
                gap_count += 1;
            }
            _ => {}
        }
    }
    gaps.push(gap_count + 1);
    
    for line in &lines[..lines.len() - 1] {
        let chars: Vec<char> = line.chars().collect();
        let mut part: Vec<String> = Vec::new();

        let mut start = 0;
        for &length in gaps.iter() {
            let width = length as usize;
            let end = start + width;
            let segment: String = chars[start..end.min(chars.len())].iter().collect();

            part.push(segment);
            start = end + 1;
        }

        matrix.push(part);
    }

    (matrix, operations)
}