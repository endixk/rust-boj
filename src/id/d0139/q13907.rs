// BOJ 13907 [Taxes]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
use std::collections::VecDeque;

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

const INF: i32 = 0x3f3f3f3f;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, k) = (
        next::<usize>(&mut it),
        next::<usize>(&mut it),
        next::<usize>(&mut it));
    let (s, d) = (
        next::<usize>(&mut it) - 1,
        next::<usize>(&mut it) - 1);

    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let (u, v, w) = (
            next::<usize>(&mut it) - 1,
            next::<usize>(&mut it) - 1,
            next::<i32>(&mut it));
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let mut dp = vec![vec![INF; n]; n+1];
    let mut gd = vec![INF; n];
    dp[0][s] = 0;
    let mut q = VecDeque::new();
    q.push_back(s);
    for i in 0..n {
        let z = q.len();
        let mut inq = vec![false; n];
        for _ in 0..z {
            let u = q.pop_front().unwrap();
            for &(v, w) in adj[u].iter() {
                if dp[i+1][v] > dp[i][u] + w {
                    dp[i+1][v] = dp[i][u] + w;
                    if !inq[v] && gd[v] > dp[i+1][v] {
                        gd[v] = dp[i+1][v];
                        inq[v] = true;
                        q.push_back(v);
                    }
                }
            }
        }
    }

    let mut v = vec![0];
    (0..k).for_each(|i| {
        let x = v[i] + next::<i32>(&mut it);
        v.push(x);
    });

    let mut to = n;
    for t in v {
        let (mut ans, mut nto) = (INF, n);
        for i in 0..=to {
            if ans > dp[i][d] + t * i as i32 {
                ans = dp[i][d] + t * i as i32;
                nto = i;
            }
        }
        writeln!(so, "{}", ans).unwrap();
        to = nto;
    }
}
