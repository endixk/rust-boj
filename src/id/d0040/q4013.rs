// BOJ 4013 [ATM]
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

fn tarjan(
    scc: &mut Vec<Vec<usize>>,
    adj: &Vec<Vec<usize>>,
    ids: &mut Vec<usize>,
    vis: &mut Vec<bool>,
    stk: &mut Vec<usize>,
    cnt: &mut usize,
    cur: usize) -> usize {

    *cnt += 1;
    let mut ret = *cnt;
    vis[cur] = true;
    ids[cur] = *cnt;
    stk.push(cur);

    for &nxt in &adj[cur] {
        if ids[nxt] == 0 {
            ret = ret.min(tarjan(scc, adj, ids, vis, stk, cnt, nxt));
        } else if vis[nxt] {
            ret = ret.min(ids[nxt]);
        }
    }

    if ret == ids[cur] {
        let mut scc_cur = Vec::new();
        loop {
            let top = stk.pop().unwrap();
            scc_cur.push(top);
            vis[top] = false;
            if top == cur { break; }
        }
        // scc_cur.sort_unstable();
        scc.push(scc_cur);
    }

    ret
}

fn dfs(sdj: &Vec<Vec<usize>>, vis: &mut Vec<bool>, cur: usize) {
    vis[cur] = true;
    for &nxt in &sdj[cur] {
        if !vis[nxt] {
            dfs(sdj, vis, nxt);
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; n+1];
    for _ in 0..m {
        adj[next::<usize>(&mut it)].push(next::<usize>(&mut it));
    }

    let mut scc = Vec::new();
    let mut ids = vec![0; n+1];
    let mut vis = vec![false; n+1];
    let mut stk = Vec::new();
    let mut cnt = 0;
    for i in 1..=n {
        if ids[i] == 0 {
            tarjan(&mut scc, &adj, &mut ids, &mut vis, &mut stk, &mut cnt, i);
        }
    }

    let mut sid = vec![0; n+1];
    for (i, v) in scc.iter().enumerate() {
        for &j in v {
            sid[j] = i;
        }
    }

    let mut bal = vec![0; scc.len()];
    for i in 1..=n {
        bal[sid[i]] += next::<i32>(&mut it);
    }

    let src = next::<usize>(&mut it);
    let mut res = vec![false; scc.len()];
    for _ in 0..next(&mut it) {
        res[sid[next::<usize>(&mut it)]] = true;
    }

    let mut sdj = vec![vec![]; scc.len()];
    for i in 0..scc.len() {
        for &j in &scc[i] {
            for &k in &adj[j] {
                if sid[k] != i {
                    sdj[i].push(sid[k]);
                }
            }
        }
        sdj[i].sort_unstable();
        sdj[i].dedup();
    }

    let mut vis = vec![false; scc.len()];
    dfs(&sdj, &mut vis, sid[src]);
    for i in 0..scc.len() {
        if !vis[i] { bal[i] = 0; }
    }

    let mut dp = vec![0; scc.len()];
    let mut ans = 0;
    for i in (0..scc.len()).rev() {
        dp[i] += bal[i];
        if res[i] { ans = ans.max(dp[i]); }
        for &j in &sdj[i] {
            dp[j] = dp[j].max(dp[i]);
        }
    }

    writeln!(so, "{}", ans)?;

    Ok(())
}
