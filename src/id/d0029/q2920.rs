// BOJ 2920 [Scales]
// Supported by GitHub Copilot

use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let scale: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut asc = false;
    let mut dsc = false;
    let mut x = scale[0];
    for s in scale[1..].iter() {
        match x < *s {
            true => asc = true,
            false => dsc = true,
        }
        x = *s;
    }

    match (asc, dsc) {
        (true, true) => println!("mixed"),
        (true, false) => println!("ascending"),
        (false, true) => println!("descending"),
        _ => println!(),
    }
}
