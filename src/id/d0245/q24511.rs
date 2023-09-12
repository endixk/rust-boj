// BOJ 24511 [queuestack]
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

    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| next::<i32>(&mut it) == 0).collect::<Vec<_>>();
    let mut q = std::collections::VecDeque::<i32>::new();
    for i in 0..n {
        let c = next(&mut it);
        if a[i] { q.push_front(c); }
    }
    for _ in 0..next(&mut it) {
        q.push_back(next(&mut it));
        write!(so, "{} ", q.pop_front().unwrap()).ok();
    }
}
