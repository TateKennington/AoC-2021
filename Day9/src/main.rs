use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let mut ans = 0;
    let mut basins = Vec::new();
    let input = read_to_string("in").unwrap();
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut low = u32::MAX;
            if i > 0 {
                low = low.min(grid[i - 1][j])
            }

            if j > 0 {
                low = low.min(grid[i][j - 1]);
            }

            if i + 1 < grid.len() {
                low = low.min(grid[i + 1][j]);
            }

            if j + 1 < grid[i].len() {
                low = low.min(grid[i][j + 1]);
            }

            if low > grid[i][j] {
                ans += grid[i][j] + 1;

                let mut queue = vec![(i, j)];
                let mut seen = HashSet::new();
                while let Some(curr) = queue.pop() {
                    let (i, j) = curr;
                    seen.insert(curr);
                    if i > 0 && grid[i - 1][j] < 9 && !seen.contains(&(i - 1, j)) {
                        queue.push((i - 1, j))
                    }

                    if j > 0 && grid[i][j - 1] < 9 && !seen.contains(&(i, j - 1)) {
                        queue.push((i, j - 1))
                    }

                    if i + 1 < grid.len() && grid[i + 1][j] < 9 && !seen.contains(&(i + 1, j)) {
                        queue.push((i + 1, j))
                    }

                    if j + 1 < grid[i].len() && grid[i][j + 1] < 9 && !seen.contains(&(i, j + 1)) {
                        queue.push((i, j + 1))
                    }
                }
                basins.push(seen.len());
            }
        }
    }

    basins.sort_unstable();
    println!("{}", ans);
    println!("{}", basins.iter().rev().take(3).product::<usize>());
}
