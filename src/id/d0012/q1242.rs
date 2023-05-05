// BOJ 1242 [Picnic]
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

fn solve(n: usize, k: usize, m: usize) -> usize {
    if n == 1 || k % n == m { return 1; }
    let nxt = (m + n - 1) % (n - 1);
    let nxt = if nxt == 0 { n - 1 } else { nxt };
    solve(n-1, k, nxt) + 1
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (mut n, k, mut m) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));

    let mut ans = 1;
    while n > 0 {
        let x = if k % n == 0 { n } else { k % n };
        if m == x { break; }
        m = if m > x { m - x } else { m + n - x };
        n -= 1;
        ans += 1;
    }
    writeln!(so, "{}", ans).ok();
}
