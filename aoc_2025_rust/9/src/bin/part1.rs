use day_9::utils::get_input;

fn main() {
    let input = get_input("src/input.txt");
    
    let mut result = 0;
    for (i, first) in input.iter().enumerate() {
        for second in input.iter().skip(i + 1) {
            let area = (second.0.abs_diff(first.0) + 1) * (second.1.abs_diff(first.1) + 1);
            result = result.max(area);
        }
    }

    println!("{}", result);
}