// BOJ 1806 [Subsequence Sum]
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

    let buf = read(&mut si);
    let mut it = buf.split_ascii_whitespace();

    let (n, s) = (next::<usize>(&mut it), next::<i32>(&mut it));
    let arr = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();

    let mut ans = n+1;
    let (mut q, mut l, mut r) = (0, 0, 0);
    while r < n {
        q += arr[r];
        while q >= s {
            ans = ans.min(r - l + 1);
            q -= arr[l];
            l += 1;
        }
        r += 1;
    }

    writeln!(so, "{}", match ans > n {true => 0, false => ans}).unwrap();
}
