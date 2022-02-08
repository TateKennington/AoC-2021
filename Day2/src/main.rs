use std::fs::read_to_string;

fn main() {
    let input = read_to_string("in").unwrap();
    /* let ans = input
    .lines()
    .map(|line| {
        let mut split = line.trim().split_whitespace();
        let command = split.next().unwrap();
        let amount: i32 = split.next().unwrap().parse().unwrap();
        match command {
            "forward" => (amount, 0),
            "down" => (0, amount),
            "up" => (0, -amount),
            _ => unreachable!(),
        }
    })
    .fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy)); */
    let ans = input
        .lines()
        .map(|line| {
            let mut split = line.trim().split_whitespace();
            let command = split.next().unwrap();
            let amount: i32 = split.next().unwrap().parse().unwrap();
            match command {
                "forward" => (amount, 0),
                "down" => (0, amount),
                "up" => (0, -amount),
                _ => unreachable!(),
            }
        })
        .fold((0, 0, 0), |(x, y, aim), (dx, daim)| {
            (x + dx, y + aim * dx, aim + daim)
        });
    println!("{}", ans.0 * ans.1);
}
