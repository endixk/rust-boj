// BOJ 10250 [ACM Hotel]
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

    for _ in 0..read(&mut si).parse().unwrap() {
        let v: Vec<i32> = read(&mut si)
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        writeln!(so, "{}{:02}", (v[2] - 1) % v[0] + 1, (v[2] - 1) / v[0] + 1).unwrap();
    }
}
