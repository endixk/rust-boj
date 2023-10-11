// BOJ 2213 [Independent Set on a Tree]
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

fn dfs_pick(pick: &mut Vec<i32>, pass: &mut Vec<i32>,
            adj: &Vec<Vec<usize>>, val: &Vec<i32>, cur: usize, par: usize) -> i32 {
    if pick[cur] != -1 { return pick[cur]; }
    let mut ret = val[cur];
    for &nxt in &adj[cur] {
        if nxt == par { continue; }
        ret += dfs_pass(pick, pass, adj, val, nxt, cur);
    }
    pick[cur] = ret;
    ret
}
fn dfs_pass(pick: &mut Vec<i32>, pass: &mut Vec<i32>,
            adj: &Vec<Vec<usize>>, val: &Vec<i32>, cur: usize, par: usize) -> i32 {
    if pass[cur] != -1 { return pass[cur]; }
    let mut ret = 0;
    for &nxt in &adj[cur] {
        if nxt == par { continue; }
        ret += dfs_pick(pick, pass, adj, val, nxt, cur).max(dfs_pass(pick, pass, adj, val, nxt, cur));
    }
    pass[cur] = ret;
    ret
}
fn dfs_track(ans: &mut Vec<usize>, pick: &Vec<i32>, pass: &Vec<i32>,
             adj: &Vec<Vec<usize>>, cur: usize, par: usize, free: bool) {
    if free && pick[cur] > pass[cur] {
        ans.push(cur+1);
        for &nxt in &adj[cur] {
            if nxt == par { continue; }
            dfs_track(ans, pick, pass, adj, nxt, cur, false);
        }
    } else {
        for &nxt in &adj[cur] {
            if nxt == par { continue; }
            dfs_track(ans, pick, pass, adj, nxt, cur, true);
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let val = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let mut adj = vec![vec![]; n];
    for _ in 0..n-1 {
        let (u, v) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut pick = vec![-1; n];
    let mut pass = vec![-1; n];
    let mut ans = vec![];
    dfs_pick(&mut pick, &mut pass, &adj, &val, 0, 0);
    dfs_pass(&mut pick, &mut pass, &adj, &val, 0, 0);
    dfs_track(&mut ans, &pick, &pass, &adj, 0, 0, true);

    ans.sort_unstable();
    writeln!(so, "{}", pick[0].max(pass[0])).ok();
    for &x in &ans { write!(so, "{} ", x).ok(); }

    Ok(())
}
