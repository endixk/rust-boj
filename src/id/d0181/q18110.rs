// BOJ 18110 [solved.ac]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    if n == 0 { println!("0"); return; }
    let cut = (n as f64 * 0.15).round() as usize;

    let mut cnt = vec![0; 31];
    (0..n).for_each(|_| cnt[next::<usize>(&mut it)] += 1);

    let (mut c, mut i) = (cut, 0);
    while c > cnt[i] {
        c -= cnt[i];
        cnt[i] = 0;
        i += 1;
    }
    cnt[i] -= c;
    let (mut c, mut i) = (cut, 30);
    while c > cnt[i] {
        c -= cnt[i];
        cnt[i] = 0;
        i -= 1;
    }
    cnt[i] -= c;

    let sum = cnt.iter().enumerate().fold(0, |acc, (i, &x)| acc + i * x);
    println!("{}", (sum as f64 / (n - 2 * cut) as f64).round() as usize);
}
