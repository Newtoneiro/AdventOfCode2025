use day_6::utils::get_input;

fn main() {
    let (matrix, operations) = get_input("src/input.txt");
    let mut total: u64 = 0;
    
    for (i, op) in operations.iter().enumerate() {
        let col = matrix.iter().map(|row| row[i]);
        let result = match op {
            '*' => col.fold(1, |a, b| a * b),
            '+' => col.fold(0, |a, b| a + b),
            _ => 0_u64
        };
        total += result;
    }

    println!("{}", total);
}