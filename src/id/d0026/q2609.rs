// BOJ 2609 [GCD and LCM]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    si.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();

    writeln!(so, "{}", gcd(a, b)).unwrap();
    writeln!(so, "{}", a * b / gcd(a, b)).unwrap();
}
