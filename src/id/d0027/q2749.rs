// BOJ 2749 [Fibonacci Number 3]
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

    let n: u64 = read(&mut si).trim().parse().unwrap();
    let p: usize = 1_500_000;
    let mut f = vec![0; p];
    f[1] = 1;
    for i in 2..p {
        f[i] = (f[i - 1] + f[i - 2]) % 1_000_000;
    }
    writeln!(so, "{}", f[n as usize % p]).unwrap();
}
