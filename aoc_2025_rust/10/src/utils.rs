use std::fs::read_to_string;

pub fn get_input(filename: &str) -> Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let s = line.to_string();
        let first = s.find(char::is_whitespace).unwrap();
        let last = s.rfind(char::is_whitespace).unwrap();

        let part1 = &s[..first];
        let part2 = &s[first + 1 .. last];
        let part3 = &s[last + 1 ..];

        let part1: Vec<bool> = part1.replace(&['[', ']'], "").chars()
            .map(|ch| ch == '#')
            .collect();
        
        let part2: Vec<Vec<usize>> = part2
            .split_whitespace()
            .map(|tok| {
                tok.trim_matches(&['(', ')'][..])
                    .split(',')
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .collect();

        let part3: Vec<usize> = part3
            .replace(&['{', '}'], "")
            .split(',')
            .filter_map(|tok| tok.trim().parse::<usize>().ok())
    .collect();

        result.push((part1, part2, part3));
    }

    result
}