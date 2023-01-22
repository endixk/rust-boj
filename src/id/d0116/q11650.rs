// BOJ 11650 [Coordinate Sorting]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut v: Vec<(i32, i32)> = Vec::new();
    for _ in 0..read(&mut si).parse().unwrap() {
        let buf = read(&mut si);
        let mut iter = buf.split_whitespace();
        v.push((
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            )
        );
    }

    v.sort();
    for (x, y) in v {
        writeln!(so, "{} {}", x, y).unwrap();
    }
}
