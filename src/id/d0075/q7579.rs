// BOJ 7579 [Apps]
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
    let (n, m) = (next::<usize>(&mut it), next::<i32>(&mut it));

    let mut v = vec![];
    let mut c = vec![];
    (0..n).for_each(|_| { v.push(next::<i32>(&mut it)); });
    (0..n).for_each(|_| { c.push(next::<i32>(&mut it)); });
    let csum = c.iter().sum::<i32>() as usize;

    let mut dp = vec![0; csum+1];
    for i in 0..n {
        for j in (0..=csum).rev() {
            if j + c[i] as usize > csum { continue; }
            dp[j + c[i] as usize] = dp[j + c[i] as usize].max(dp[j] + v[i]);
        }
    }

    for c in 0..=csum {
        if dp[c] >= m {
            writeln!(so, "{}", c).unwrap();
            break;
        }
    }
}
