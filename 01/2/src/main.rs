use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();
    let mut freqs = HashSet::new();
    let mut sum: i64 = 0;
    freqs.insert(sum);
    let changes = input
        .lock()
        .lines()
        .map(|line| line.unwrap().trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    loop {
        for change in &changes {
            sum += change;
            if freqs.contains(&sum) {
                println!("{}", sum);
                return;
            }
            freqs.insert(sum);
        }
    }
}
