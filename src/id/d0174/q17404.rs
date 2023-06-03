// BOJ 17404 [RGB Avenue 2]
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

fn solve(dp: (u32, u32, u32), v: &Vec<(u32, u32, u32)>, n: usize) -> (u32, u32, u32) {
    let mut dp = dp;
    for i in 1..n {
        dp = (
            v[i].0 + dp.1.min(dp.2),
            v[i].1 + dp.0.min(dp.2),
            v[i].2 + dp.0.min(dp.1),
        );
    }
    dp
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| {
        let r = next::<u32>(&mut it);
        let g = next::<u32>(&mut it);
        let b = next::<u32>(&mut it);
        (r, g, b)
    }).collect::<Vec<_>>();

    const INF: u32 = 0x3f3f3f3f;
    let mut ans = INF;

    // first house is red
    let dp = solve((v[0].0, INF, INF), &v, n);
    ans = ans.min(dp.1.min(dp.2));

    // first house is green
    let dp = solve((INF, v[0].1, INF), &v, n);
    ans = ans.min(dp.0.min(dp.2));

    // first house is blue
    let dp = solve((INF, INF, v[0].2), &v, n);
    ans = ans.min(dp.0.min(dp.1));

    println!("{}", ans);
}
