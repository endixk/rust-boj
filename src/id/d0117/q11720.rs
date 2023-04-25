// BOJ 11720 [Sum of Numbers]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    si.read_line(&mut String::new()).unwrap();
    let mut buf = String::new();
    si.read_line(&mut buf).unwrap();

    let mut s = 0;
    for c in buf.trim().chars() {
        s += c.to_digit(10).unwrap();
    }
    writeln!(so, "{}", s).unwrap();
}
