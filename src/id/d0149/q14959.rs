// BOJ 14959 [Slot Machines]
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

fn pi(s: &[u32]) -> Vec<usize> {
    let l = s.len();
    let mut p = vec![0; l];

    let mut j = 0;
    for i in 1..l {
        while j > 0 && s[i] != s[j] { j = p[j-1]; }
        if s[i] == s[j] { j += 1; p[i] = j; }
    }

    p
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = (0..n).map(|_| next::<u32>(&mut it)).collect::<Vec<_>>();
    v.reverse();

    let pi = pi(&v);
    let (mut mk, mut mp) = (n, n);
    for (i, x) in pi.iter().enumerate() {
        let k = n - i - 1;
        let p = i - *x + 1;
        if k + p < mk + mp {
            mk = k;
            mp = p;
        }
    }
    println!("{} {}", mk, mp);
}
