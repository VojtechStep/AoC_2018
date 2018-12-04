use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();
    let mut twice = 0;
    let mut thrice = 0;
    for line in input.lock().lines() {
        let mut counts = [0; 26];
        let line = line.unwrap();
        for char in line.as_bytes() {
            if char >= &b'a' && char <= &b'z' {
                counts[(char - b'a') as usize] += 1;
            }
        }
        let mut found_twice = false;
        let mut found_thrice = false;
        for &count in counts.iter() {
            if !found_twice && count == 2 {
                twice += 1;
                found_twice = true;
            }
            if !found_thrice && count == 3 {
                thrice += 1;
                found_thrice = true;
            }
        }
    }
    println!("{} * {} = {}", twice, thrice, twice * thrice);
}
