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
                cur_val = (cur_val - value) % 100;
            }
            'R' => {
                cur_val = (cur_val + value) % 100;
            }
            _ => ()
        }

        if cur_val == 0 {
            counter += 1;
        }
    }

    println!("{}", counter);
}