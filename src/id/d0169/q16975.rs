// BOJ 16975 [Sequence and Queries 21]
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

struct FenwickTree {
    n: usize,
    ft: Vec<i64>,
}
impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            ft: vec![0; n+1],
        }
    }
    fn sum(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.ft[i];
            i -= i & (!i + 1);
        }
        sum
    }
    fn add(&mut self, mut i: usize, v: i64) {
        while i <= self.n {
            self.ft[i] += v;
            i += i & (!i + 1);
        }
    }
    fn update(&mut self, l: usize, r: usize, v: i64) {
        self.add(l, v);
        self.add(r+1, -v);
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut ft = FenwickTree::new(n);
    (0..n).for_each(|i| ft.update(i+1, i+1, next::<i64>(&mut it)));

    for _ in 0..next(&mut it) {
        match next::<u8>(&mut it) {
            1 => ft.update(next::<usize>(&mut it),next::<usize>(&mut it),next::<i64>(&mut it)),
            _ => writeln!(so, "{}", ft.sum(next::<usize>(&mut it))).unwrap(),
        }
    }
}
