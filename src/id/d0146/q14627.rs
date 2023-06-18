// BOJ 14627 [Padak]
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

fn get(v: &Vec<u64>, k: u64) -> u64 {
    v.iter().fold(0, |acc, &x| acc + x / k)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (s, c) = (next::<usize>(&mut it), next::<u64>(&mut it));
    let v = (0..s).map(|_| next::<u64>(&mut it)).collect::<Vec<_>>();

    let (mut l, mut r) = (1, *v.iter().max().unwrap());
    while l <= r {
        let m = (l + r) / 2;
        if get(&v, m) >= c {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    println!("{}", v.iter().sum::<u64>() - r * c);
}
