// BOJ 1263 [Scheduling]
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
    let mut v = (0..n).map(|_| (next::<i32>(&mut it), next::<i32>(&mut it))).collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

    let mut t = v[0].1 - v[0].0;
    for &(a, b) in v.iter().skip(1) {
        t = t.min(b);
        t -= a;
        if t < 0 { break; }
    }
    writeln!(so, "{}", if t < 0 { -1 } else { t }).ok();
}
