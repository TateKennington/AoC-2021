use std::fs::read_to_string;

fn main() {
    let input = read_to_string("in").unwrap();
    let mut nums: Vec<i32> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();
    nums.sort_unstable();

    let med = if nums.len() & 1 == 0 {
        (nums[nums.len() / 2] + nums[nums.len() / 2 - 1]) / 2
    } else {
        nums[nums.len() / 2]
    };

    let ans: i32 = nums.iter().map(|&x| (x - med).abs()).sum();
    println!("{}", ans);

    let avg: i32 = nums.iter().sum::<i32>() / nums.len() as i32;
    let ans: i32 = nums
        .iter()
        .map(|&x| {
            let dist = (x - avg).abs();
            dist * (dist + 1) / 2
        })
        .sum();
    println!("{}", ans);

    let avg: i32 = avg + 1;
    let ans: i32 = nums
        .iter()
        .map(|&x| {
            let dist = (x - avg).abs();
            dist * (dist + 1) / 2
        })
        .sum();
    println!("{}", ans);
}
