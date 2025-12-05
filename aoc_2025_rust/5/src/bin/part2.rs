use day_5::utils::get_input;
use std::cmp;

fn main() {
    let (ranges, _ids) = get_input("src/input.txt");
    let mut parsed_ranges: Vec<(u64, u64)> = Vec::new();

    fn overlaps(a: &(u64, u64), b: &(u64, u64)) -> bool {
        a.0 <= b.1 && b.0 <= a.1
    }

    fn remove_indexes<T>(v: &mut Vec<T>, mut idxs: Vec<usize>) {
        idxs.sort_unstable_by(|a, b| b.cmp(a));
        for idx in idxs {
            v.remove(idx);
        }
    }

    for raw_range in ranges.iter() {
        let mut new_range: (u64, u64) = *raw_range;
        let mut ranges_to_combine: Vec<usize> = Vec::new();
        for (i, parsed_range) in parsed_ranges.iter().enumerate() {
            if overlaps(&new_range, parsed_range) {
                new_range.0 = cmp::min(new_range.0, parsed_range.0);
                new_range.1 = cmp::max(new_range.1, parsed_range.1);
                ranges_to_combine.push(i);
            }
        }

        remove_indexes(&mut parsed_ranges, ranges_to_combine);
        parsed_ranges.push(new_range);
    }

    let mut result = 0;

    for (start, end) in &parsed_ranges {
        result += end - start + 1
    }

    println!("{}", result);
}