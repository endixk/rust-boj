// BOJ 2143 [Sum of Two Arrays]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

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

fn pref(a: Vec<i64>) -> Vec<i64> {
    let mut p = vec![0];
    for i in 0..a.len() {
        p.push(p[i] + a[i]);
    }
    p
}
fn ps(p: &Vec<i64>, i: usize, j: usize) -> i64 {
    p[j+1] - p[i]
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let t = next::<i64>(&mut it);
    let n = next::<usize>(&mut it);
    let a = pref((0..n).map(|_| next::<i64>(&mut it)).collect());
    let m = next::<usize>(&mut it);
    let b = pref((0..m).map(|_| next::<i64>(&mut it)).collect());

    let mut map = HashMap::new();
    for i in 0..n { for j in i..n {
        let s = ps(&a, i, j);
        *map.entry(s).or_insert(0i64) += 1;
    }}

    let mut ans = 0;
    for i in 0..m { for j in i..m {
        let s = ps(&b, i, j);
        if let Some(&v) = map.get(&(t - s)) {
            ans += v;
        }
    }}
    writeln!(so, "{}", ans).unwrap();
}
