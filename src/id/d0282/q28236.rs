// BOJ 28236 [Lunch Race]
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

    let (_, _, k) = (next::<usize>(&mut it), next::<i32>(&mut it), next::<usize>(&mut it));
    let (mut ans, mut min) = (0, i32::MAX);
    for i in 1..=k {
        let (f, d) = (next::<i32>(&mut it), next::<i32>(&mut it));
        if f - d < min {
            min = f - d;
            ans = i;
        }
    }
    println!("{}", ans);
}

