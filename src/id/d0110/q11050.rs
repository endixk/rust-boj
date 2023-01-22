// BOJ 11050 [Binomial Coefficient 1]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn recursive_binomial(n: i32, k: i32) -> i32 {
    if k == 0 || k == n {
        1
    } else {
        recursive_binomial(n - 1, k - 1) + recursive_binomial(n - 1, k)
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let v: Vec<i32> = read(&mut si).split_whitespace().map(|x| x.parse().unwrap()).collect();
    writeln!(so, "{}", recursive_binomial(v[0], v[1])).unwrap();
}
