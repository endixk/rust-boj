// BOJ 2675 [Repeat String]
// Supported by GitHub Copilot

use std::io::stdin;

pub fn main() {
    let mut t = String::new();
    stdin().read_line(&mut t).unwrap();
    let t = t.trim().parse::<i16>().unwrap();

    for _ in 0..t {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();

        let r = iter.next().unwrap().parse::<i8>().unwrap();
        let s = iter.next().unwrap();

        for c in s.chars() {
            for _ in 0..r {
                print!("{}", c);
            }
        }
        println!();
    }
}