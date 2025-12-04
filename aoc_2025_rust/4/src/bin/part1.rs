use day_4::utils::get_input;

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    let grid = get_input("src/input.txt");

    let count_neighbors = |x: u32, y: u32| -> u32 {
        let mut result: u32 = 0;

        for (dx, dy) in &NEIGHBORS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= grid[0].len() as i32 ||
               ny < 0 || ny >= grid.len() as i32 {
                continue;
            }
            result += grid[ny as usize][nx as usize] as u32;
        }

        result
    };

    let mut result = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, field) in row.iter().enumerate() {
            match field {
                1 => {
                    result = if count_neighbors(x.try_into().unwrap(), y.try_into().unwrap()) < 4 { result + 1 } else { result };
                },
                _ => (),
            }
        }
    } 

    println!("{}", result);
}