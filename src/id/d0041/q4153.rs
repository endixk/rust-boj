// BOJ 4153 [Right Triangle]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    loop {
        si.read_line(&mut buf).unwrap();
        if buf.trim() == "0 0 0" { break }

        let mut v: Vec<i32> = buf.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        v.sort();
        match v[0].pow(2) + v[1].pow(2) == v[2].pow(2) {
            true => writeln!(so, "right").unwrap(),
            false => writeln!(so, "wrong").unwrap()
        }

        buf.clear();
    }
}
