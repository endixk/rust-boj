// BOJ 1233 [Bovine Bones]
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
    let (s1, s2, s3) = (next(&mut it), next(&mut it), next(&mut it));

    let mut cnt = vec![0; s1+s2+s3+1];
    for i in 1..=s1 {
        for j in 1..=s2 {
            for k in 1..=s3 {
                cnt[i+j+k] += 1;
            }
        }
    }

    let mut ans = 0;
    let mut max = 0;
    for i in 1..=s1+s2+s3 {
        if cnt[i] > max {
            max = cnt[i];
            ans = i;
        }
    }

    writeln!(so, "{}", ans).unwrap();
}
