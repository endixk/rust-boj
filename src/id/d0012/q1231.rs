// BOJ 1231 [Stock Market]
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
    let (c, d, mut m) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    const MAX: usize = 500000;

    let mut market = [0; 555];
    (0..c).for_each(|i| {
        (0..d).for_each(|j| {
            market[i*d+j] = next::<usize>(&mut it);
        })
    });

    let mut dp = [0; MAX+1];
    for j in 1..d {
        for p in 0..=m { dp[p] = p; }
        for i in 0..c {
            for p in market[i*d+j-1]..=m {
                if dp[p] < dp[p - market[i*d+j-1]] + market[i*d+j] {
                    dp[p] = dp[p - market[i*d+j-1]] + market[i*d+j];
                }
            }
        }
        m = dp[m];
    }

    writeln!(so, "{}", m).unwrap();
}
