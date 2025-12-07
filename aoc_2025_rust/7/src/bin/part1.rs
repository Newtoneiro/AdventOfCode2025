use day_7::utils::get_input;

fn main() {
    fn mark_bean(grid: &mut Vec<Vec<char>>, pos: (usize, usize)) {
        grid[pos.1][pos.0] = '|';
    }

    let mut grid = get_input("src/input.txt");
    let mut counter = 0;

    let start: (usize, usize) = (grid[0].iter().position(|n| *n == 'S').unwrap() , 0);
    mark_bean(&mut grid, start);

    for y in 1..grid.len() {
        for x in 0..grid[y].len() {
            let prev_ch = grid[y - 1][x];
            let cur_ch  = grid[y][x];

            if prev_ch != '|' { continue; }

            match cur_ch {
                '.' => mark_bean(&mut grid, (x, y)),
                '^' => {
                    mark_bean(&mut grid, (x - 1, y));
                    mark_bean(&mut grid, (x + 1, y));
                    counter += 1;
                }
                _ => {}
            }
        }
    }

    println!("{}", counter);
}