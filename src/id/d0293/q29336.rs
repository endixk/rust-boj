// BOJ 29336 [Emergency]
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

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    a.sort_unstable();

    let mut qsum = 0;
    for i in 0..m {
        let (t, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
        while qsum < q {
            if a.is_empty() { break; }
            qsum += a.pop().unwrap() + t;
        }
        if qsum < q && a.is_empty() {
            writeln!(so, "-1").ok();
            return;
        }
        if i == m-1 {
            while let Some(x) = a.pop() {
                qsum += x + t;
            }
        }
    }
    writeln!(so, "{}", qsum).ok();
}
