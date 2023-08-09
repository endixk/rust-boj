// BOJ 28139 [Average]
// Supported by GitHub Copilot

use std::io::{self, Read};
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

    let n = next::<usize>(&mut it);
    let mut sum = 0.0;
    let mut v = vec![(0, 0); n];
    for i in 0..n {
        (v[i].0, v[i].1) = (next::<i32>(&mut it), next::<i32>(&mut it));
        for j in 0..i {
            sum += (((v[i].0 - v[j].0).pow(2) + (v[i].1 - v[j].1).pow(2)) as f64).sqrt();
        }
    }
    let n = n as f64;
    println!("{:.9}", sum * 2.0 / n);
}
