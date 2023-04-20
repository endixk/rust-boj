// BOJ 12852 [Down to One 2]
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

    let n = next(&mut read(&mut si).split_ascii_whitespace());
    let mut memo = [0; 1000001];
    let mut path = [0; 1000001];

    for i in 2..=n {
        memo[i] = memo[i - 1] + 1;
        path[i] = i - 1;
        if i % 2 == 0 {
            if memo[i] > memo[i / 2] + 1 {
                memo[i] = memo[i / 2] + 1;
                path[i] = i / 2;
            }
        }
        if i % 3 == 0 {
            if memo[i] > memo[i / 3] + 1 {
                memo[i] = memo[i / 3] + 1;
                path[i] = i / 3;
            }
        }
    }

    writeln!(so, "{}", memo[n]).unwrap();
    let mut i = n;
    while i > 0 {
        write!(so, "{} ", i).unwrap();
        i = path[i];
    }
    writeln!(so).unwrap();
}
