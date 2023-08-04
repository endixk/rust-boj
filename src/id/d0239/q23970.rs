// BOJ 23970 [Bubble Sort 3]
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
    let mut v = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let w = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let mut d = (0..n).fold(0, |acc, i| acc + (v[i] - w[i]).abs());
    if d == 0 {
        writeln!(so, "1").unwrap();
        return;
    }

    for x in (0..n).rev() {
        for i in 0..x {
            if v[i] > v[i+1] {
                d += (v[i+1] - w[i]).abs() + (v[i] - w[i+1]).abs() - (v[i] - w[i]).abs() - (v[i+1] - w[i+1]).abs();
                if d == 0 {
                    writeln!(so, "1").unwrap();
                    return;
                }
                v.swap(i, i+1);
            }
        }
    }
    writeln!(so, "0").unwrap();
}

