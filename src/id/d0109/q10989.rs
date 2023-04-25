// BOJ 10989 [Sort the Numbers 3]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    si.read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    let mut v = vec![0;10001];
    for _ in 0..n {
        buf.clear();
        si.read_line(&mut buf).unwrap();
        v[buf.trim().parse::<usize>().unwrap()] += 1;
    }

    for i in 1..10001 {
        for _ in 0..v[i] {
            writeln!(so, "{}", i).unwrap();
        }
    }
}
