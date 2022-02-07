use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("in").unwrap();
    let mut counts: Vec<u32> = Vec::new();
    let mut total = 0;
    input.lines().for_each(|line| {
        total += 1;
        let line = line.trim();
        if counts.is_empty() {
            counts.resize(line.len(), 0);
        }

        line.chars().enumerate().for_each(|(index, char)| {
            if char == '1' {
                counts[index] += 1;
            }
        });
    });

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    counts.iter().for_each(|&count| {
        gamma <<= 1;
        epsilon <<= 1;
        if count >= (total / 2) {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    });

    println!("{}", gamma * epsilon);

    let mut lines: HashSet<_> = input.lines().map(|line| line.trim()).collect();
    for i in 0..counts.len() {
        let count = lines.iter().filter(|&&line| &line[i..i + 1] == "1").count();
        if count >= lines.len() / 2 + lines.len() % 2 {
            lines.retain(|&line| &line[i..i + 1] == "1");
        } else {
            lines.retain(|&line| &line[i..i + 1] == "0");
        }

        if lines.len() == 1 {
            break;
        }
    }
    let mut oxygen = 0;
    lines.iter().next().unwrap().chars().for_each(|char| {
        oxygen <<= 1;
        if char == '1' {
            oxygen += 1;
        }
    });

    let mut lines: HashSet<_> = input.lines().map(|line| line.trim()).collect();
    for i in 0..counts.len() {
        let count = lines.iter().filter(|&&line| &line[i..i + 1] == "1").count();
        if count >= lines.len() / 2 + lines.len() % 2 {
            lines = lines
                .iter()
                .filter(|&&line| &line[i..i + 1] == "0")
                .cloned()
                .collect()
        } else {
            lines = lines
                .iter()
                .filter(|&&line| &line[i..i + 1] == "1")
                .cloned()
                .collect()
        }

        if lines.len() == 1 {
            break;
        }
    }
    let mut carbon = 0;
    lines.iter().next().unwrap().chars().for_each(|char| {
        carbon <<= 1;
        if char == '1' {
            carbon += 1;
        }
    });
    println!("{}", oxygen * carbon);
}
