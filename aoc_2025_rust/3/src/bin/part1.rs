use day_3::utils::get_input;

fn main() {
    let input = get_input("src/input.txt");
    let mut total: u64 = 0;
    for row in input {
        let max_1 = row
            .iter()
            .take(row.len() - 1)
            .max()
            .unwrap();
        let idx = row.iter().position(|&x| x == *max_1).unwrap();
        let max_2 = &row[idx + 1..]
            .iter()
            .max()
            .unwrap();
        let n = (*max_1 as u64) * 10 + (**max_2 as u64);
        total += n;
    }
    println!("{}", total);
}