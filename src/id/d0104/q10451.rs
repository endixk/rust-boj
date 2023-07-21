// BOJ 10451 [Permutation Cycle]
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

    for _ in 0..next(&mut it) {
        let n = next::<usize>(&mut it);
        let v = (0..n).map(|_| next::<usize>(&mut it)-1).collect::<Vec<_>>();
        let mut vis = vec![false; n];
        let mut ans = 0;
        for i in 0..n {
            if !vis[i] {
                let mut j = i;
                while !vis[j] {
                    vis[j] = true;
                    j = v[j];
                }
                ans += 1;
            }
        }
        writeln!(so, "{}", ans).unwrap();
    }
}
