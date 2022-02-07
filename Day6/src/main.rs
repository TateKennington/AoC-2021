use std::fs::read_to_string;

fn main() {
    const DAYS: u64 = 256;
    let input = read_to_string("in").unwrap();
    let mut pop = [0; 7];
    let mut b_pop = [0; 7];
    input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .for_each(|x: u64| pop[x as usize] += 1);

    for i in 0..DAYS {
        let day = (i % 7) as usize;
        let next = ((i + 2) % 7) as usize;
        b_pop[next] += pop[day];
        pop[day] += b_pop[day];
        b_pop[day] = 0;
    }

    let ans: u64 = pop.iter().sum::<u64>() + b_pop.iter().sum::<u64>();
    println!("{}", ans);
}
