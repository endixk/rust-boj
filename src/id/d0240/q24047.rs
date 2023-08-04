// BOJ 24047 [Bubble Sort 5]
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

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut v = (0..n).map(|_| next::<u32>(&mut it)).collect::<Vec<_>>();
    let mut q = (0..n-1).collect::<VecDeque<_>>();
    let mut cnt = 0;
    while let Some(i) = q.pop_front() {
        if v[i] > v[i+1] {
            v.swap(i, i+1);
            if i > 0 { q.push_back(i-1); }
            cnt += 1;
        }
        if cnt == k {
            v.iter().for_each(|&x| write!(so, "{} ", x).unwrap());
            writeln!(so).unwrap();
            return;
        }
    }
    writeln!(so, "-1").unwrap();
}
