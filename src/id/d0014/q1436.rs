// BOJ 1436 [Movie Director]
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
    let d = 666;
    let mut v = Vec::new();
    for x in 0..10000 {
        v.push(x*1000 + d);
        v.push(d*10000 + x);
    }
    for x in 0..1000 {
        for y in 0..10 {
            v.push(x*10000 + d*10 + y);
            v.push(y*1000000 + d*1000 + x);
        }
    }
    for x in 0..100 {
        for y in 0..100 {
            v.push(x*100000 + d*100 + y);
        }
    }

    v.sort();
    v.dedup();
    writeln!(so, "{}", v[next::<usize>(&mut it)-1]).unwrap();
}
