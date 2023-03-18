// BOJ 1932 [Number Triangle]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = it.by_ref().next().unwrap().parse::<usize>().unwrap();
    let v: Vec<i32> = it.map(|x| x.parse().unwrap()).collect();

    let mut dp = vec![0; v.len()];
    for i in 1..=n {
        dp[v.len() - i] = v[v.len() - i];
    }
    for i in (1..n).rev() {
        for j in 0..i {
            let loc = i * (i + 1) / 2 - 1 - j;
            dp[loc] = (v[loc] + dp[loc + i]).max(v[loc] + dp[loc + i + 1]);
        }
    }

    writeln!(so, "{}", dp[0]).unwrap();
}
