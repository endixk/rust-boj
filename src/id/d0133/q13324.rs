// BOJ 13324 [Sequence 2]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
use std::collections::BinaryHeap;

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

    let mut pq = BinaryHeap::new();
    let mut t = 0;
    let mut v = Vec::new();
    for _ in 0..next(&mut it) {
        t += 1;
        let a = next::<i64>(&mut it);
        if let Some(&x) = pq.peek() {
            if x + t > a {
                pq.pop();
                pq.push(a - t);
            }
        }
        pq.push(a - t);
        v.push(*pq.peek().unwrap() + t);
    }

    v.iter().for_each(|&x| write!(so, "{} ", x).unwrap());
    writeln!(so).unwrap();
    for i in (0..v.len()-1).rev() {
        if v[i] >= v[i+1] { v[i] = v[i+1] - 1; }
    }
    v.iter().for_each(|&x| write!(so, "{} ", x).unwrap());
}
