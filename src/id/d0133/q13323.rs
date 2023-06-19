// BOJ 13323 [Sequence 1]
// Supported by GitHub Copilot

use std::io::{self, Read};
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
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut pq = BinaryHeap::new();
    let mut ans = 0;
    let mut t = 0;
    for _ in 0..next(&mut it) {
        t += 1;
        let a = next::<i64>(&mut it);
        if let Some(&x) = pq.peek() {
            if x + t > a {
                ans += x + t - a;
                pq.pop();
                pq.push(a - t);
            }
        }
        pq.push(a - t);
    }
    println!("{}", ans);
}
