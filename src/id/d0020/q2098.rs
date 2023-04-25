// BOJ 2098 [Traveling Salesman Problem]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

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

const INF: i32 = 0x3f3f3f3f;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n: usize = next(&mut it);

    let mut arr = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            arr[i][j] = next(&mut it);
        }
    }

    let mut dp = vec![vec![INF; 1<<n]; n];
    let mut q = VecDeque::new();
    dp[0][1] = 0;
    q.push_back((0, 1));
    while let Some((src, mask)) = q.pop_front() {
        for dst in 0..n {
            if mask & (1<<dst) != 0 { continue; }
            if arr[src][dst] == 0 { continue; }
            let next = mask | (1<<dst);
            if dp[dst][next] > dp[src][mask] + arr[src][dst] {
                dp[dst][next] = dp[src][mask] + arr[src][dst];
                q.push_back((dst, next));
            }
        }
    }

    let mut ans = INF;
    for i in 0..n {
        if arr[i][0] == 0 { continue; }
        ans = ans.min(dp[i][(1<<n)-1] + arr[i][0]);
    }
    writeln!(so, "{}", ans).unwrap();
}
