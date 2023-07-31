// BOJ 1946 [New Recruits]
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

    for _ in 0..next(&mut it) {
        let n = next::<usize>(&mut it);
        let mut v = vec![0; n];
        for _ in 0..n {
            v[next::<usize>(&mut it) - 1] = next::<i32>(&mut it);
        }

        let (mut cnt, mut min) = (1, v[0]);
        for &x in v.iter().skip(1) {
            if x < min { cnt += 1; min = x; }
        }
        println!("{}", cnt);
    }
}
