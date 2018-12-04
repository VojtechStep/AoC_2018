use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();
    let lock = input.lock();
    let nums = lock
        .lines()
        .map(|line| line.unwrap().trim().parse::<i64>().unwrap());
    let result = nums.sum::<i64>();
    println!("{}", result);
}
