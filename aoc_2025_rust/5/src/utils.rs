use std::fs::read_to_string;

pub fn get_input(filename: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = Vec::<(u64, u64)>::new();
    let mut ids = Vec::<u64>::new();

    let mut doing_ranges = true;

    for line in read_to_string(filename).unwrap().lines() {
        let line = line.trim();

        if line.is_empty() {
            doing_ranges = false;
            continue;
        }

        if doing_ranges {
            let mut parts = line.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            ranges.push((start, end));
        } else {
            let id: u64 = line.parse().unwrap();
            ids.push(id);
        }
    }

    (ranges, ids)
}