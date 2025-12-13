use day_12::utils::get_input;

fn main() {
    let (blocks, grids) = get_input("src/input.txt");
    let mut result = 0;

    for ((w, h), requirements) in grids.iter() {
        let area = w * h;
        let mut required_space = 0;
        for (i, count) in requirements.iter().enumerate() {
            required_space += blocks[i].iter().flatten().filter(|&&v| v).count() as u64 * *count;  // Thank you Eric
        }
        if required_space <= area {
            result += 1;
        }
    }

    println!("{}", result);
}