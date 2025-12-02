use day_1::utils::get_input;

fn main() {
    let input = get_input("src/input.txt");
    let mut counter: u64 = 0;
    let mut cur_val: i32 = 50;

    for action in input {
        let direction: char = action.chars().next().unwrap();
        let value: i32 = action[1..].parse().unwrap();

        match direction {
            'L' => {
                for _ in 0..value {
                    cur_val -= 1;
                    if cur_val == 0 {
                        counter += 1;
                    }
                    if cur_val < 0 {
                        cur_val = 99;
                    }
                }
            }
            'R' => {
                for _ in 0..value {
                    cur_val += 1;
                    if cur_val == 100 {
                        cur_val = 0;
                        counter += 1;
                    }
                }
            }
            _ => ()
        }
    }

    println!("{}", counter);
}