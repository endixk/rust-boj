// BOJ 20920 [Vocabularies]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
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

use std::collections::HashMap;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut map = HashMap::new();
    for _ in 0..n {
        let s = next::<String>(&mut it);
        if s.len() >= m { *map.entry(s).or_insert(0) += 1; }
    }
    let mut v = map.into_iter().collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| {
        b.1.cmp(&a.1).then(b.0.len().cmp(&a.0.len())).then(a.0.cmp(&b.0))
    });
    for (s, _) in v { writeln!(so, "{}", s).unwrap(); }
}
