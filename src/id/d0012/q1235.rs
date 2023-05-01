// BOJ 1235 [Student IDs]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
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

    let mut v = (0..next(&mut it)).map(|_|
        next::<String>(&mut it).chars().rev().collect::<String>()
    ).collect::<Vec<_>>();
    v.sort();

    let mut k = 1;
    v.iter().zip(v.iter().skip(1)).for_each(|(a, b)| {
        let mut i = 0;
        while a[i..i+1] == b[i..i+1] {
            i += 1;
        }
        k = k.max(i+1);
    });

    writeln!(so, "{}", k).unwrap();
}
