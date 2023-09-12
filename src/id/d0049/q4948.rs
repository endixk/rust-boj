// BOJ 4948 [Bertrand's Postulate]
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

fn sieve(n: usize) -> Vec<bool> {
    let mut p = vec![true; n + 1];
    p[0] = false; p[1] = false;
    for i in 2..=n {
        if p[i] {
            for j in (i * i..=n).step_by(i) {
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

    let p = sieve(250000);
    loop {
        let n = next::<usize>(&mut it);
        if n == 0 { break; }
        writeln!(so, "{}", p[n+1..=2*n].iter().filter(|&x| *x).count()).unwrap();
    }
}
