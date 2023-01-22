// BOJ 2751 [Sort the Numbers 2]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    si.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        si.read_line(&mut buf).unwrap();
        v.push(buf.trim().parse::<i32>().unwrap());
    }

    v.sort();
    for i in v {
        writeln!(so, "{}", i).unwrap();
    }
}
