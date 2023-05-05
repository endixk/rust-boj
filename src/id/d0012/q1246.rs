// BOJ 1246 [Going Once, Going Twice, Gone!]
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
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<i32>(&mut it), next::<i32>(&mut it));
    let mut p = (0..m).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    p.sort_unstable();

    let mut ans = (0, 0);
    for i in 0..m {
        let v = p[i as usize] * if m - i > n { n } else { m - i };
        if ans.1 < v {
            ans = (p[i as usize], v);
        }
    }
    writeln!(so, "{} {}", ans.0, ans.1).ok();
}
