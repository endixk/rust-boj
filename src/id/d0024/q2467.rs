// BOJ 2467 [Solutions]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();

    let mut ans = (0 as usize, n-1);
    let mut sol = v[ans.0] + v[ans.1];
    let (mut l, mut r) = (0 as usize, n-1);
    while l < r {
        let sum = v[l] + v[r];
        if sum.abs() < sol.abs() {
            sol = sum;
            ans = (l, r);
        }
        if sum < 0 {
            l += 1;
        } else {
            r -= 1;
        }
    }

    writeln!(so, "{} {}", v[ans.0], v[ans.1]).unwrap();
}
