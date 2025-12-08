use day_8::utils::get_input;
use std::collections::HashSet;


fn main() {
    let input = get_input("src/input.txt");

    fn get_distance(first: (i64, i64, i64), second: (i64, i64, i64)) -> f64 {
        (((first.0 - second.0).pow(2) + (first.1 - second.1).pow(2) + (first.2 - second.2).pow(2)) as f64).sqrt()
    }

    fn remove_indexes<T>(v: &mut Vec<T>, mut idxs: Vec<usize>) {
        idxs.sort_unstable_by(|a, b| b.cmp(a));
        for idx in idxs {
            v.remove(idx);
        }
    }

    let mut distances: Vec::<((usize, usize), f64)> = Vec::new();
    for (i, first) in input.iter().enumerate() {
        for (j, second) in input.iter().enumerate().skip(i + 1) {
            distances.push(((i, j), get_distance(*first, *second)));
        }
    }
    distances.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());

    let mut groups: Vec<HashSet<usize>> = Vec::new();
    for (i, j) in distances.iter().map(|(pair, _)| pair) {
        let mut new_group: HashSet<usize> = HashSet::new();
        let mut groups_to_combine: Vec<usize> = Vec::new();
        new_group.insert(*i);
        new_group.insert(*j);
        
        for (n, group) in groups.iter().enumerate() {
            if group.contains(i) || group.contains(j) {
                new_group.extend(group);
                groups_to_combine.push(n);
            }
        }
        
        remove_indexes(&mut groups, groups_to_combine);
        groups.push(new_group);

        if groups.len() == 1 && groups[0].len() == input.len() {
            println!("{}", input[*i].0 * input[*j].0);
            break;
        }
    }
}