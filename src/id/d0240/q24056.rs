// BOJ 24056 [Selection Sort 6]
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

    let mut n = next::<usize>(&mut it);
    let mut v = (0..n as u32).map(|_| next::<u32>(&mut it)).collect::<Vec<_>>();
    let mut w = (0..n as u32).map(|_| next::<u32>(&mut it)).collect::<Vec<_>>();
    while n > 0 && v[n-1] == w[n-1] { n -= 1; }
    if n == 0 { println!("1"); return; }

    w = w[..n].iter().map(|&x| x).collect::<Vec<_>>();
    let mut k = None;
    for i in 1..n {
        if w[i-1] == w[i] {
            k = Some(w[i]);
            break;
        }
    }
    if let Some(k) = k {
        if k <= v[n-1] {
            println!("0");
            return;
        }
    }

    w.dedup();
    if w.len() == n {
        v = v[..n].iter().map(|&x| x).collect::<Vec<_>>();
        v.sort_unstable();
        println!("{}", if v == w { 1 } else { 0 });
    } else {
        v = v[..n-1].iter().map(|&x| x).collect::<Vec<_>>();
        v.sort_unstable();
        println!("{}", if v == w { 1 } else { 0 });
    }
}
