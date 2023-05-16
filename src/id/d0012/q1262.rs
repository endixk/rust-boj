// BOJ 1262 [ASCII Diamondi]
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

fn mht(x1: usize, y1: usize, x2: usize, y2: usize) -> usize { // Manhattan distance
    (if x1 > x2 { x1 - x2 } else { x2 - x1 }) + (if y1 > y2 { y1 - y2 } else { y2 - y1 })
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, r1, c1, r2, c2) = (
        next::<usize>(&mut it),
        next::<usize>(&mut it), next::<usize>(&mut it),
        next::<usize>(&mut it), next::<usize>(&mut it)
    );
    for r in r1..=r2 {
        let mut s = String::new();
        for c in c1..=c2 {
            let d = mht(r % (2*n-1), c % (2*n-1), n-1, n-1);
            if d < n { s.push(((d % 26) as u8 + b'a') as char); }
            else { s.push('.'); }
        }
        writeln!(so, "{}", s).ok();
    }
}
