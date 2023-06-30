// BOJ 2535 [AOI]
// Supported by GitHub Copilot

use std::io::{self, Read};

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
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut v = (0..next(&mut it)).map(|_| {
        (next::<u32>(&mut it), next::<u32>(&mut it), next::<u32>(&mut it))
    }).collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| b.2.cmp(&a.2));

    println!("{} {}", v[0].0, v[0].1);
    println!("{} {}", v[1].0, v[1].1);
    let n = if v[0].0 == v[1].0 { v[0].0 } else { 1001 };
    let mut c = 2;
    while v[c].0 == n { c += 1; }
    println!("{} {}", v[c].0, v[c].1);
}
