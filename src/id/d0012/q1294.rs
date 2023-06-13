// BOJ 1294 [String Decoration]
// Supported by GitHub Copilot

use std::io::{self, Read};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

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

#[derive(Eq, PartialEq)]
struct Decor {
    s: String,
}
impl PartialOrd for Decor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.s.starts_with(&other.s) {
            Some(Ordering::Greater)
        } else if other.s.starts_with(&self.s) {
            Some(Ordering::Less)
        } else {
            Some(other.s.cmp(&self.s))
        }
    }
}
impl Ord for Decor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut pq = BinaryHeap::new();
    (0..n).map(|_| next::<String>(&mut it)).for_each(|s| pq.push(Decor { s }));

    let mut ans = String::new();
    while let Some(s) = pq.pop() {
        ans.push(s.s.chars().next().unwrap());
        if s.s.len() > 1 {
            pq.push(Decor { s: s.s[1..].to_string() });
        }
    }
    println!("{}", ans);
}
