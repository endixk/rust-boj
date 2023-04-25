// BOJ 1654 [Cutting Cables]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn count(wires: &Vec<u32>, len: u32) -> u32 {
    wires.iter().map(|&wire| wire / len).sum()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (k, n) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let wires = (0..k).map(|_| next::<u32>(&mut it)).collect::<Vec<_>>();

    let (mut lo, mut hi) = (1, wires.iter().max().unwrap() + 1);
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if count(&wires, mid) >= n as u32 { lo = mid }
        else { hi = mid; }
    }

    writeln!(so, "{}", lo).unwrap();
}
