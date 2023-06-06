// BOJ 5467 [Type Printer]
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

fn pref(a: &String, b: &String) -> u8 {
    let mut l = 0;
    let (a, b) = (a.as_bytes(), b.as_bytes());
    for i in 0..a.len().min(b.len()) {
        if a[i] == b[i] {
            l += 1;
        } else {
            break;
        }
    }
    l
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let (mut lmax, mut loc) = (0, 0);
    let mut v = Vec::new();
    for i in 0..n {
        let s = next::<String>(&mut it);
        if lmax < s.len() {
            lmax = s.len();
            loc = i;
        }
        v.push(s);
    }

    let pivot = v[loc].clone();
    v.sort_unstable_by(|a, b| {
        pref(a, &pivot).cmp(&pref(b, &pivot))
            .then(a.cmp(b))
    });

    let mut op = vec![];
    for c in v[0].chars() { op.push(c); }
    op.push('P');
    for i in 1..v.len() {
        let p = pref(&v[i - 1], &v[i]) as usize;
        for _ in p..v[i-1].len() { op.push('-'); }
        for c in v[i].chars().skip(p) { op.push(c); }
        op.push('P');
    }

    writeln!(so, "{}", op.len()).ok();
    for c in op { write!(so, "{} ", c).ok(); }
}
