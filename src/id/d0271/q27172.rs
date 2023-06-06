// BOJ 27172 [The Game of Division]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

    let n = next::<usize>(&mut it);
    let x = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let mut v = [false; 1000001];
    x.iter().for_each(|&x| v[x] = true);

    let mut score = [0; 1000001];
    for i in 1..=1000000 {
        if v[i] {
            for j in (i<<1..=1000000).step_by(i) {
                if v[j] {
                    score[i] += 1;
                    score[j] -= 1;
                }
            }
        }
    }

    x.iter().for_each(|&x| write!(so, "{} ", score[x]).unwrap());
}
