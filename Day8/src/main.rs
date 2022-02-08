use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let mut ans = 0;
    let input = read_to_string("in").unwrap();
    input.lines().for_each(|line| {
        let mut split = line.trim().split(" | ");
        let signal = split.next().unwrap();
        let output = split.next().unwrap();

        let mut unique = HashMap::new();
        signal.split(' ').for_each(|line| {
            let set: HashSet<_> = line.trim().chars().collect();
            if line.len() == 2 {
                unique.insert(1, set);
            } else if line.len() == 3 {
                unique.insert(7, set);
            } else if line.len() == 4 {
                unique.insert(4, set);
            } else if line.len() == 7 {
                unique.insert(8, set);
            }
        });

        let mut curr = 0;
        output.split(' ').for_each(|line| {
            curr *= 10;
            let set: HashSet<_> = line.trim().chars().collect();
            let overlap = [
                unique.get(&1).unwrap().intersection(&set).count(),
                unique.get(&4).unwrap().intersection(&set).count(),
                unique.get(&7).unwrap().intersection(&set).count(),
            ];

            match (line.len(), overlap) {
                (2, _) => curr += 1,
                (5, [1, 2, 2]) => curr += 2,
                (5, [2, 3, 3]) => curr += 3,
                (4, _) => curr += 4,
                (5, [1, 3, 2]) => curr += 5,
                (6, [1, 3, 2]) => curr += 6,
                (3, _) => curr += 7,
                (7, _) => curr += 8,
                (6, [2, 4, 3]) => curr += 9,
                _ => (),
            }
        });
        ans += curr;
        println!("{}", curr);
    });

    println!("{}", ans);
}
