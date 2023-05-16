// BOJ 1269 [Symmetric Difference]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

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

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut set = HashSet::new();
    let mut cnt = 0;
    (0..n).for_each(|_| { set.insert(next::<i32>(&mut it)); });
    (0..m).for_each(|_| { if set.contains(&next::<i32>(&mut it)) { cnt += 1; } });

    writeln!(so, "{}", n + m - (cnt << 1)).unwrap();
}
