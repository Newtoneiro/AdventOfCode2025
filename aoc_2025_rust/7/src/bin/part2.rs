use day_7::utils::get_input;
use std::collections::HashMap;


fn main() {
    fn flatten(grid: &Vec<Vec<char>>, y: usize) -> Vec<char> {
        grid.iter().skip(y - 1).flatten().copied().collect()
    }

    fn mark_bean(grid: &mut Vec<Vec<char>>, pos: (usize, usize)) {
        grid[pos.1][pos.0] = '|';
    }

    let mut grid = get_input("src/input.txt");

    let start: (usize, usize) = (grid[0].iter().position(|n| *n == 'S').unwrap() , 0);
    mark_bean(&mut grid, start);

    fn count_paths(
        grid: &mut Vec<Vec<char>>,
        cur_y: usize,
        cache: &mut HashMap<(usize, Vec<char>), u64>,
    ) -> u64 {
        if cur_y == grid.len() { return 1 }

        let key = (cur_y, flatten(grid, cur_y));
        if let Some(&v) = cache.get(&key) {
            return v;
        }

        let mut paths = 0;
        for x in 0..grid[cur_y].len() {
            let prev_ch = grid[cur_y - 1][x];
            let cur_ch  = grid[cur_y][x];
    
            if prev_ch != '|' { continue; }

            match cur_ch {
                '.' => {
                    mark_bean(grid, (x, cur_y));
                    paths += count_paths(grid, cur_y + 1, cache);
                }
                '^' => {
                    let mut grid1 = grid.clone();
                    let mut grid2 = grid.clone();
                    
                    mark_bean(&mut grid1, (x - 1, cur_y));
                    mark_bean(&mut grid2, (x + 1, cur_y));

                    paths += count_paths(&mut grid1, cur_y + 1, cache);
                    paths += count_paths(&mut grid2, cur_y + 1, cache);
                }
                _ => {}
            }
        }

        cache.insert(key, paths);
        paths
    }

    let mut cache: HashMap<(usize, Vec<char>), u64> = HashMap::new();
    println!("{}", count_paths(&mut grid, 1, &mut cache));
}