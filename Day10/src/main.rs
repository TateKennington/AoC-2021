use std::fs::read_to_string;

fn main() {
    let input = read_to_string("in").unwrap();
    let mut scores = Vec::new();
    let ans: i32 = input
        .lines()
        .map(|line| {
            let line = line.trim();
            let mut stack = Vec::new();
            for c in line.chars() {
                match (c, stack.last()) {
                    (a, Some(&b)) if closes(b, a) => {
                        stack.pop();
                    }
                    (')', _) => return 3,
                    (']', _) => return 57,
                    ('}', _) => return 1197,
                    ('>', _) => return 25137,
                    _ => stack.push(c),
                }
            }
            if !stack.is_empty() {
                scores.push(stack.iter().rev().fold(0u64, |acc, &c| {
                    acc * 5
                        + match c {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => unreachable!(),
                        }
                }))
            }
            0
        })
        .sum();

    println!("{}", ans);
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}

fn closes(a: char, b: char) -> bool {
    matches!((a, b), ('(', ')') | ('{', '}') | ('[', ']') | ('<', '>'))
}
