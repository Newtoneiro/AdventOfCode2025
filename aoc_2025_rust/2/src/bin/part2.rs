use day_2::utils::get_input;
use std::collections::HashSet;

fn proper_divisors(n: usize) -> Vec<usize> {
    (1..n)
        .filter(|i| n % i == 0)
        .collect()
}

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
            for divisor in proper_divisors(i) {
                let bottom = 10u64.pow(divisor as u32 - 1);
                let upper = 10u64.pow(divisor as u32) - 1;
                for digit_part in bottom..=upper {
                    let full_digit_str = digit_part.to_string().repeat((i / divisor) as usize);
                    let full_digit: u64 = full_digit_str.parse().unwrap(); 
                    if full_digit >= low && full_digit <= high {
                        invalid_ids.insert(full_digit);
                    } 
                }
            }
        }
    }
    println!("{}", invalid_ids.iter().sum::<u64>());
}