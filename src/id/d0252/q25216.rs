// BOJ 25216 [Farming Routes]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
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

fn go(adj: &Vec<Vec<usize>>, a: &[i64], x: &[i64], y: &[i64], c: &[i64], n: usize, t: i64) -> i64 {
    let mut dp = vec![0; n];
    let mut ap = vec![0; n];
    for k in (0..t).rev() {
        let mut tdp = vec![0; n];
        let mut tap = vec![0; n];
        for u in 0..n {
            tdp[u] = c[u];
            tap[u] = (a[u] + x[u] * k - 1) / y[u] + 1;
            let (mut mv, mut mc) = (0, 0);
            for &v in &adj[u] {
                if mv < dp[v] {
                    mv = dp[v];
                    mc = ap[v];
                } else if mv == dp[v] {
                    mc = mc.min(ap[v]);
                }
            }
            if mv > 0 {
                tdp[u] += mv;
                tap[u] = tap[u].max(mc);
            }
        }
        dp = tdp;
        ap = tap;
    }
    ap[0]
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, t) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i64>(&mut it));
    let (mut a, mut x, mut y, mut c) = (
        Vec::new(), Vec::new(), Vec::new(), Vec::new()
    );
    for _ in 0..n {
        a.push(next::<i64>(&mut it));
        x.push(next::<i64>(&mut it));
        y.push(next::<i64>(&mut it));
        c.push(next::<i64>(&mut it));
    }
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let (u, v) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1);
        adj[u].push(v);
    }

    writeln!(so, "{}", go(&adj, &a, &x, &y, &c, n, t)).ok();

    Ok(())
}