// BOJ 27651 [Bug Cut]
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
    let mut p = vec![0];
    for i in 0..n {
        p.push(p[i] + next::<i64>(&mut it));
    }

    let mut ans = 0;
    let (mut x, mut y) = (1, n);
    for i in 1..n {
        while p[x] <= (p[i] + p[n]) >> 1 { x += 1; }
        while p[y] >= p[n] - p[i] { y -= 1; }
        if x <= y { ans += y - x + 1; }
        else { break; }
    }
    println!("{}", ans);
}
