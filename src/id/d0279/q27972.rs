// BOJ 27972 [Musical Note]
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
    let v = (0..n).map(|_| next::<u32>(&mut it)).collect::<Vec<_>>();
    let (mut p, mut c, mut a, mut d) = (v[0], 1, 0, true);
    for x in v {
        if d && p < x { c += 1; }
        else if d && p > x { d = !d; c = 2; }
        else if !d && p > x { c += 1; }
        else if !d && p < x { d = !d; c = 2; }
        if c > a { a = c; }
        p = x;
    }
    writeln!(so, "{}", a).unwrap();
}
