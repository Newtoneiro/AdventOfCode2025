use day_11::utils::get_input;

fn main() {
    let routes = get_input("src/input.txt");
    let mut result = 0;
    
    let mut active_roads = Vec::<Vec<String>>::new();
    active_roads.push(vec!["you".to_string()]);

    loop {
        let mut new_roads = Vec::<Vec<String>>::new();
        for road in active_roads {
            for next in routes.get(road.last().unwrap()).unwrap() {
                if next == "out" {
                    result += 1;
                } else {
                    let mut new_road = road.clone();
                    new_road.push(next.clone());
                    new_roads.push(new_road);
                }
            }
        }
        if new_roads.len() == 0 {
            break;
        }

        active_roads = new_roads;
    }

    println!("{}", result);
}