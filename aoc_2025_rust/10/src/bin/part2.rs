use day_10::utils::get_input;

fn main() {
    let input = get_input("src/test_input.txt");
    let mut total = 0;

    fn press(state: &Vec<usize>, button: &Vec<usize>) -> Vec<usize> {
        let mut output = state.clone();

        for idx in button.iter() {
            output[*idx] += 1;
        }

        output
    }

    'main: for (_, buttons, target) in input.iter() {
        let mut queue: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
        queue.push((vec![0; target.len()], Vec::<usize>::new()));
        
        loop {
            let mut new_queue: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
            for (state, pressed) in queue.iter() {
                'inner: for (button_idx, button) in buttons.iter().enumerate() {
                    let new_state = press(&state, &button);
                    let mut new_pressed = pressed.clone();
                    new_pressed.push(button_idx);

                    for (cur_val, target_val) in new_state.iter().zip(target.iter()) {
                        if cur_val > target_val {
                            continue 'inner;
                        }
                    }
    
                    if new_state == *target {
                        total += new_pressed.len();
                        continue 'main;
                    }
    
                    new_queue.push((new_state.clone(), new_pressed));
                }
            }
            queue = new_queue;
        }
    }

    println!("{}", total);
}