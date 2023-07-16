// BOJ 23351 [Watering]
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

    let (n, k, a, b) = (
        next::<usize>(&mut it),
        next::<usize>(&mut it),
        next::<usize>(&mut it),
        next::<usize>(&mut it)
    );
    let mut v = vec![k; n];
    let (mut d, mut i) = (1, 0);
    loop {
        (0..a).map(|x| i+x).for_each(|i| v[i] += b);
        if v.iter().any(|&x| x <= d) { break; }
        i = (i+a) % n;
        d += 1;
    }
    println!("{}", d);
}
