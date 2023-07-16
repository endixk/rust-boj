// BOJ 18324 [Race]
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

fn go(k: usize, x: usize, i: usize) -> Option<usize> {
    let m = if i > x {
        (i*(i+1) + i*(i-1) - x*(x-1)) / 2
    } else {
        i*(i+1) / 2
    };

    if k >= m {
        return if i > x {
            Some(2*i - x + (k-m+i-1)/i)
        } else {
            Some(i + (k-m+i-1)/i)
        }
    } else { None }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (k, n) = (next::<usize>(&mut it), next::<usize>(&mut it));
    for _ in 0..n {
        let x = next::<usize>(&mut it);
        let (mut l, mut r) = (1, 1_000_000_000);
        while l < r {
            let m = (l+r) / 2;
            if go(k, x, m).is_some() { l = m+1; }
            else { r = m; }
        }
        writeln!(so, "{}", go(k, x, l-1).unwrap()).unwrap();
    }
}
