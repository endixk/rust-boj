// BOJ 28427 [Tricknology]
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

const MAX: usize = 500000;
fn sieve(n: usize) -> Vec<bool> {
    let mut p = vec![true; n + 1];
    p[0] = false; p[1] = false;
    for i in 2..=n {
        if p[i] {
            for j in (i*i..=n).step_by(i) {
                p[j] = false;
            }
        }
    }
    p
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let p = sieve(MAX<<1);
    let mut v = vec![0; MAX];
    for i in 2..MAX {
        v[i] = v[i-1] + if p[i<<1|1] { 1 } else { 0 };
    }

    for _ in 0..next(&mut it) {
        let (l, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
        writeln!(so, "{}", v[r-1] - v[l-1]).unwrap();
    }
}
