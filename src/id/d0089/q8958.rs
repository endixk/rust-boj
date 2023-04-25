// BOJ 8958 [OX Quiz]
// Supported by GitHub Copilot

use std::io::{self, Write};

pub fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();

    for _ in 0..t.trim().parse::<i32>().unwrap() {
        let mut tot = 0;
        let mut cur = 0;
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        for c in s.trim().chars() {
            match c {
                'O' => cur += 1,
                _ => cur = 0,
            }
            tot += cur;
        }
        writeln!(out, "{}", tot).unwrap();
    }
}
