use std::fs::read_to_string;

fn main() {
    let input = read_to_string("day1.in").unwrap();
    /* let ans = input
    .lines()
    .map(|line| line.trim().parse::<i32>().unwrap())
    .collect::<Vec<_>>()
    .windows(2)
    .filter(|pair| pair[1] > pair[0])
    .count(); */
    let ans = input
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|triple| triple.iter().sum::<i32>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count();
    println!("{}", ans);
}
