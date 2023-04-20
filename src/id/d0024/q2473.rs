// BOJ 2473 [Three Solutions]
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
    let mut v = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    v.sort();

    let mut ans: (usize, usize, usize) = (0, 1, 2);
    let mut sol = v[ans.0] + v[ans.1] + v[ans.2];
    for x in 0..n {
        let (mut l, mut r) = (x+1, n-1);
        while l < r {
            let sum = v[x] + v[l] + v[r];
            if sum.abs() < sol.abs() {
                sol = sum;
                ans = (x, l, r);
            }
            if sum < 0 {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }

    writeln!(so, "{} {} {}", v[ans.0], v[ans.1], v[ans.2]).unwrap();
}
