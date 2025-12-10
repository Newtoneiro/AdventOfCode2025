use day_10::utils::get_input;

fn main() {
    let input = get_input("src/input.txt");
    let mut total = 0;

    fn press(state: &Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
        let mut output = state.clone();

        for idx in button.iter() {
            output[*idx] = !output[*idx];
        }

        output
    }

    'main: for (target, buttons, _) in input.iter() {
        let mut queue: Vec<(Vec<bool>, Vec<usize>)> = Vec::new();
        queue.push((vec![false; target.len()], Vec::<usize>::new()));
        
        loop {
            let mut new_queue: Vec<(Vec<bool>, Vec<usize>)> = Vec::new();
            for (state, pressed) in queue.iter() {
                for (button_idx, button) in buttons.iter().enumerate() {
                    let new_state = press(&state, &button);
                    let mut new_pressed = pressed.clone();
                    new_pressed.push(button_idx);
    
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