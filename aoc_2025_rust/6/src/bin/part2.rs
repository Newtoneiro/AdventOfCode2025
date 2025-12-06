use day_6::utils::get_input_2;


fn main() {
    let (matrix, operations) = get_input_2("src/input.txt");
    let mut total: u64 = 0;
    
    for (i, op) in operations.iter().enumerate() {
        let col: Vec<String> = matrix.iter().map(|row| row[i].clone()).collect();
        let mut subtotal = if *op == '*' { 1 } else { 0 };

        for k in (0..col[0].len()).rev() {
            let num: u64 = col.iter()
                .map(|col| col.chars().nth(k).unwrap())
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap_or(0);
            match op {
                '*' => subtotal *= num,
                '+' => subtotal += num,
                _ => ()
            };
        }
        total += subtotal;
    }

    println!("{}", total);
}