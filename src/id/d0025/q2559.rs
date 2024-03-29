// BOJ 2559 [Sequence]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let mut win = a.iter().take(k).sum::<i32>();
    let mut ans = win;
    for i in k..n {
        win += a[i] - a[i-k];
        ans = ans.max(win);
    }
    writeln!(so, "{}", ans).unwrap();
}
