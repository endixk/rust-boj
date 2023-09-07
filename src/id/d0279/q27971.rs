// BOJ 27971 [Dogs]
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

use std::collections::VecDeque;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, a, b) = (
        next::<usize>(&mut it), next::<usize>(&mut it),
        next::<usize>(&mut it), next::<usize>(&mut it)
    );
    let mut v = vec![true; n+1];
    for _ in 0..m {
        let (l, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
        for i in l..=r { v[i] = false; }
    }

    let mut ans = 1;
    let mut q = VecDeque::new();
    q.push_back(0);
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let u = q.pop_front().unwrap();
            if u+a == n { writeln!(so, "{ans}").ok(); return; }
            if u+b == n { writeln!(so, "{ans}").ok(); return; }
            if u+a < n && v[u+a] {
                v[u+a] = false;
                q.push_back(u+a);
            }
            if u+b < n && v[u+b] {
                v[u+b] = false;
                q.push_back(u+b);
            }
        }
        ans += 1;
    }
    writeln!(so, "-1").ok();
}
