// BOJ 28472 [Minimax Tree]
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

fn dfs(depth: &mut [usize], par: &mut [usize], adj: &Vec<Vec<usize>>, u: usize, d: usize) {
    depth[u] = d;
    for &v in adj[u].iter() {
        if v != par[u] {
            par[v] = u;
            dfs(depth, par, adj, v, d+1);
        }
    }
}
fn go(dp: &mut [i32], adj: &Vec<Vec<usize>>, depth: &[usize], par: &[usize], u: usize) -> i32 {
    if dp[u] != -1 { return dp[u]; }
    let mut ans = 0;
    if depth[u] & 1 == 0 {
        for &v in adj[u].iter() {
            if v != par[u] {
                ans = ans.max(go(dp, adj, depth, par, v));
            }
        }
    } else {
        ans = i32::MAX;
        for &v in adj[u].iter() {
            if v != par[u] {
                ans = ans.min(go(dp, adj, depth, par, v));
            }
        }
    }
    dp[u] = ans;
    ans
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; n+1];
    for _ in 1..n {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut depth = vec![0; n+1];
    let mut par = vec![0; n+1];
    dfs(&mut depth, &mut par, &adj, r, 0);

    let mut dp = vec![-1; n+1];
    for _ in 0..next(&mut it) {
        let (k, t) = (next::<usize>(&mut it), next::<i32>(&mut it));
        dp[k] = t;
    }

    for _ in 0..next(&mut it) {
        writeln!(so, "{}", go(&mut dp, &adj, &depth, &par, next::<usize>(&mut it))).ok();
    }
}
