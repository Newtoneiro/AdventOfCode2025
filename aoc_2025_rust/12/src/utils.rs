use std::fs::read_to_string;

pub fn get_input(filename: &str) -> (Vec<Vec<Vec<bool>>>, Vec<((u64, u64), Vec<u64>)>) {
    let content = read_to_string(filename).unwrap();
    let mut grids = Vec::new();
    let mut sequences = Vec::new();

    let mut lines = content.lines().peekable();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(colon_pos) = line.find(':') {
            let header = &line[..colon_pos];
            if header.parse::<u64>().is_ok() {
                let mut grid = Vec::new();
                while let Some(&next_line) = lines.peek() {
                    let next_line = next_line.trim();
                    if next_line.is_empty() {
                        lines.next();
                        continue;
                    }
                    if next_line.chars().all(|c| c == '#' || c == '.') {
                        let row: Vec<bool> = next_line.chars().map(|c| c == '#').collect();
                        grid.push(row);
                        lines.next();
                    } else {
                        break;
                    }
                }
                grids.push(grid);
                continue;
            }
        }

        if let Some(colon_pos) = line.find(':') {
            let size_part = &line[..colon_pos].trim();
            let data_part = &line[colon_pos + 1..].trim();

            if let Some(x_pos) = size_part.find('x') {
                if let (Ok(width), Ok(height)) = (
                    size_part[..x_pos].parse::<u64>(),
                    size_part[x_pos + 1..].parse::<u64>(),
                ) {
                    let indices: Vec<u64> = data_part
                        .split_whitespace()
                        .filter_map(|s| s.parse::<u64>().ok())
                        .collect();
                    sequences.push(((width, height), indices));
                }
            }
        }
    }

    (grids, sequences)
}
