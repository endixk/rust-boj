// BOJ 10942 [Palindrome?]
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

fn solve(pal: &mut [i8], a: &Vec<i32>, n: usize, s: usize, e: usize) -> i8 {
    if pal[s*n+e] < 0 {
        pal[s*n+e] = if solve(pal, a, n, s+1, e-1) == 1 && a[s] == a[e] { 1 } else { 0 };
    }
    pal[s*n+e]
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let mut pal = [-1 as i8; 4000004];
    (0..n).for_each(|i| pal[i*n+i] = 1);
    (0..n-1).for_each(|i| pal[i*n+i+1] = if a[i] == a[i+1] { 1 } else { 0 });

    let m = next::<usize>(&mut it);
    (0..m).for_each(|_| {
        writeln!(so, "{}", solve(&mut pal, &a, n, next::<usize>(&mut it)-1, next::<usize>(&mut it)-1)).unwrap();
    });
}
