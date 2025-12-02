use day_2::utils::get_input;
use std::collections::HashSet;

fn main() {
    let ranges = get_input("src/input.txt");

    let mut invalid_ids: HashSet<u64> = HashSet::new();
    for range in ranges {
        let parts: Vec<&str> = range.split('-').collect();
        let low: u64 = parts[0].parse().unwrap();
        let high: u64 = parts[1].parse().unwrap();
        
        let low_digits = low.to_string().len();
        let high_digits = high.to_string().len();

        for i in low_digits..=high_digits {
            if i % 2 == 0 {
                let n: u32 = (i / 2).try_into().unwrap();
                let bottom = 10u64.pow(n - 1);
                let upper = 10u64.pow(n) - 1;
                for digit_part in bottom..=upper {
                    let full_digit: u64 = format!("{}{}", digit_part, digit_part)
                        .parse::<u64>()
                        .unwrap();
                    if full_digit >= low && full_digit <= high {
                        invalid_ids.insert(full_digit);
                    } 
                }
            }
        }
    }
    println!("{}", invalid_ids.iter().sum::<u64>());
}