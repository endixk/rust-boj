// BOJ 28014 [Pushing Towers]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read<T>(si: &mut T) -> String where T: Read {
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
    let h = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let (mut c, mut ans) = (h[0], 1);
    for &x in h.iter().skip(1) {
        if x >= c {
            ans += 1;
        }
        c = x;
    }
    writeln!(so, "{}", ans).ok();
}

