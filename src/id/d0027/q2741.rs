// BOJ 2741 [Print N]
// Supported by GitHub Copilot

use std::io::{self, Write};

pub fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    for i in 1..=n {
        writeln!(out, "{}", i).unwrap();
    }
}
