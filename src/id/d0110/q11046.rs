// BOJ 11046 [Palindrome??]
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

fn manacher(v: Vec<u32>, n: usize) -> Vec<usize> {
    let mut a = vec![0; n];
    let (mut r, mut p) = (0, 0);
    for i in 0..n {
        if i <= r {
            a[i] = a[2 * p - i].min(r - i);
        }
        while i > a[i] && i + a[i] + 1 < n && v[i - a[i] - 1] == v[i + a[i] + 1] {
            a[i] += 1;
        }
        if r < i + a[i] {
            r = i + a[i];
            p = i;
        }
    }
    a
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = vec![0; 2 * n + 1];
    (0..n).for_each(|i| { v[2 * i + 1] = next::<u32>(&mut it); });

    let a = manacher(v, 2 * n + 1);
    for _ in 0..next(&mut it) {
        let (l, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
        writeln!(so, "{}", if a[l + r - 1] >= r - l { 1 } else { 0 }).ok();
    }
}
