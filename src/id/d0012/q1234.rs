// BOJ 1234 [Christmas Tree]
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

fn fact(n: usize) -> usize {
    let mut ret = 1;
    for i in 2..=n { ret *= i }
    ret
}

fn solve(level: usize, lmax: usize, r: usize, g: usize, b: usize, dp: &mut Vec<usize>) -> usize {
    if level > lmax { return 1 }
    let x = r << 14 | g << 7 | b;
    if dp[x] > 0 { return dp[x] }

    let mut ret = 0;
    if r >= level { ret += solve(level + 1, lmax, r - level, g, b, dp) }
    if g >= level { ret += solve(level + 1, lmax, r, g - level, b, dp) }
    if b >= level { ret += solve(level + 1, lmax, r, g, b - level, dp) }
    if level % 2 == 0 {
        let v = level >> 1;
        if r >= v && g >= v { ret += solve(level + 1, lmax, r - v, g - v, b, dp) * fact(level) / fact(v) / fact(v) }
        if r >= v && b >= v { ret += solve(level + 1, lmax, r - v, g, b - v, dp) * fact(level) / fact(v) / fact(v) }
        if g >= v && b >= v { ret += solve(level + 1, lmax, r, g - v, b - v, dp) * fact(level) / fact(v) / fact(v) }
    }
    if level % 3 == 0 {
        let v = level / 3;
        if r >= v && g >= v && b >= v { ret += solve(level + 1, lmax, r - v, g - v, b - v, dp) * fact(level) / fact(v) / fact(v) / fact(v) }
    }

    dp[x] = ret;
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    writeln!(so, "{}", solve(1, next(&mut it), next(&mut it), next(&mut it), next(&mut it), &mut vec![0; 1 << 21])).ok();
}
