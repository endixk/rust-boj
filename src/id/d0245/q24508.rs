// BOJ 24508 [Nadori Pang]
// Supported by GitHub Copilot

use std::io::{self, Read};
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

    let (n, k, t) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut sum = 0;
    let mut a = [0; 100001];
    for _ in 0..n {
        let x = next::<usize>(&mut it);
        sum += x;
        a[x] += 1;
    }
    if sum == 0 { println!("YES"); return; }
    if sum % k != 0 { println!("NO"); return; }
    let x = sum / k;

    let (mut i, mut c, mut mv) = (100000, 0, 0);
    loop {
        if a[i] == 0 { i -= 1; continue; }
        mv += i; c += 1; a[i] -= 1;
        if c == x { break; }
    }
    println!("{}", if sum - mv <= t { "YES" } else { "NO" });
}
