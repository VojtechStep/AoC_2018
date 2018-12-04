use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();
    let lines = input
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    'loo: for (i, line) in lines.iter().enumerate() {
        for other_line in &lines[i + 1..] {
            let common = line
                .as_bytes()
                .iter()
                .zip(other_line.as_bytes().iter())
                .filter_map(|(char, other_char)| {
                    if char == other_char {
                        Some(char.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if common.len() == line.len() - 1 {
                println!(
                    "{}, {} : {}",
                    line,
                    other_line,
                    String::from_utf8(common).unwrap()
                );
                break 'loo;
            }
        }
    }
}
