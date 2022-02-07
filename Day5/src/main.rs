use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("in").unwrap();
    let mut grid: HashMap<(i32, i32), i32> = HashMap::default();

    for line in input.lines() {
        let mut split = line.trim().split(" -> ");
        let from = split.next().unwrap();
        let to = split.next().unwrap();

        let mut split = from.split(',');
        let x: i32 = split.next().unwrap().parse().unwrap();
        let y: i32 = split.next().unwrap().parse().unwrap();
        let mut from = (x, y);

        let mut split = to.split(',');
        let x: i32 = split.next().unwrap().parse().unwrap();
        let y: i32 = split.next().unwrap().parse().unwrap();
        let to = (x, y);

        while from != to {
            let entry = grid.entry(from).or_insert(0);
            *entry += 1;

            from.0 += match from.0.cmp(&to.0) {
                Ordering::Greater => -1,
                Ordering::Less => 1,
                Ordering::Equal => 0,
            };

            from.1 += match from.1.cmp(&to.1) {
                Ordering::Greater => -1,
                Ordering::Less => 1,
                Ordering::Equal => 0,
            };
        }

        let entry = grid.entry(to).or_insert(0);
        *entry += 1;
    }

    let ans = grid.values().filter(|&&x| x > 1).count();
    println!("{}", ans);
}
