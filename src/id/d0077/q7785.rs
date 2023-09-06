// BOJ 7785 [Easy Work]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
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

    let n = next::<usize>(&mut it);
    let mut set = HashSet::new();
    for _ in 0..n {
        let (s, _) = (next::<String>(&mut it), next::<String>(&mut it));
        if set.contains(&s) { set.remove(&s); } else { set.insert(s); }
    }
    let mut keys = set.into_iter().collect::<Vec<_>>();
    keys.sort_unstable();
    keys.iter().rev().for_each(|s| writeln!(so, "{}", s).unwrap());
}
