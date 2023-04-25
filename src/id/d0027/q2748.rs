// BOJ 2748 [Fibonacci Number 2]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let n: usize = read(&mut si).trim().parse().unwrap();
    let mut fx: i64 = 0;
    let mut fy: i64 = 1;
    for _ in 0..n {
        let fz = fx + fy;
        fx = fy;
        fy = fz;
    }
    writeln!(so, "{}", fx).unwrap();
}
