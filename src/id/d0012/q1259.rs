// BOJ 1259 [Palindromic Numbers]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    while {
        si.read_line(&mut buf).unwrap();
        buf.trim() != "0"
    } {
        writeln!(so, "{}",
                 if buf.trim().chars().rev().collect::<String>() == buf.trim() {
                     "yes"
                 } else {
                     "no"
                 }
        ).unwrap();
        buf.clear();
    }
}
