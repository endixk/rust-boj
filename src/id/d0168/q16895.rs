// BOJ 16895 [Nim Game 3]
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

    let n = next::<usize>(&mut it);
    let p = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..n {
        let nim = p.iter().fold(0, |acc, &x| acc ^ x);
        for k in 0..p[i] {
            if (nim ^ p[i] ^ k) == 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
