// BOJ 4134 [Next Prime]
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

fn isp(n: u64) -> bool {
    if n < 2 { return false; }
    for i in 2..n {
        if i*i > n { break; }
        if n % i == 0 { return false; }
    }
    true
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let mut n = next(&mut it);
        while !isp(n) { n += 1; }
        writeln!(so, "{}", n).unwrap();
    }
}
