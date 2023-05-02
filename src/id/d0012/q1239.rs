// BOJ 1239 [Pie Chart]
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

static mut T: u8 = 0;
fn solve(p: &[u8]) {
    let mut s = 0;
    let mut t = 0;
    let mut v = Vec::new();
    for &x in p {
        t += x;
        if t > 50 {
            s += if v.contains(&(t - 50)) { 1 } else { 0 };
        } else {
            v.push(t);
        }
    }
    unsafe { T = T.max(s); }
}
fn permutation(p: &mut Vec<u8>, v: &mut [bool], a: &[u8], n: usize) {
    if n == p.len() {
        solve(p);
    } else {
        for i in 0..n {
            if !v[i] {
                v[i] = true;
                p.push(a[i]);
                permutation(p, v, a, n);
                p.pop();
                v[i] = false;
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n: usize = next(&mut it);
    let a = (0..n).map(|_| next(&mut it)).collect::<Vec<u8>>();
    let mut v = vec![false; n];
    let mut p = Vec::new();
    permutation(&mut p, &mut v, &a, n);
    writeln!(so, "{}", unsafe { T }).unwrap();
}
