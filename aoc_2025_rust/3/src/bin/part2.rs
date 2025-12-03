use day_3::utils::get_input;

fn main() {
    let input = get_input("src/input.txt");
    let mut total: u128 = 0;
    for row in input {
        let mut cur: u128 = 0;
        let mut prev_idx: usize = 0;
        for n in (0..12).rev() {
            let cur_slice = &row[prev_idx..row.len() - n];
            let cur_max = cur_slice.iter().max().unwrap();
            let max_idx = cur_slice.iter().position(|&x| x == *cur_max).unwrap();
            prev_idx += max_idx + 1;
            cur += *cur_max as u128 * 10_u128.pow(n.try_into().unwrap());
        }
        total += cur;
    }
    println!("{}", total);
}