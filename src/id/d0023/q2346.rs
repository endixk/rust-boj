// BOJ 2346 [Popping Balloons]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = Vec::new();
    for i in 0..n {
        let c = next::<i32>(&mut it);
        v.push((c, i+1, (i+n-1)%n, (i+1)%n));
    }

    let mut i = 0;
    for _ in 0..n {
        write!(so, "{} ", v[i].1).unwrap();
        let (l, r) = (v[i].2, v[i].3);
        v[l].3 = r; v[r].2 = l;
        let c = v[i].0;
        if c > 0 { for _ in 0..c { i = v[i].3; } }
        else { for _ in 0..-c { i = v[i].2; } }
    }
}
