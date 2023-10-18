// BOJ 4196 [Dominos]
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
    sid: &mut Vec<usize>,
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
            ret = ret.min(tarjan(scc, adj, ids, sid, vis, stk, cnt, nxt));
        } else if vis[nxt] {
            ret = ret.min(ids[nxt]);
        }
    }

    if ret == ids[cur] {
        let mut scc_cur = Vec::new();
        loop {
            let top = stk.pop().unwrap();
            scc_cur.push(top);
            sid[top] = scc.len() + 1;
            vis[top] = false;
            if top == cur { break; }
        }
        scc.push(scc_cur);
    }

    ret
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let mut adj = vec![vec![]; n+1];
        for _ in 0..m {
            let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
            adj[u].push(v);
        }

        let mut ids = vec![0; n+1];
        let mut sid = vec![0; n+1];
        let mut vis = vec![false; n+1];
        let mut stk = Vec::new();
        let mut cnt = 0;
        let mut scc = Vec::new();
        for i in 1..=n {
            if ids[i] == 0 {
                tarjan(&mut scc, &adj, &mut ids, &mut sid, &mut vis, &mut stk, &mut cnt, i);
            }
        }

        let mut ind = vec![0; scc.len()];
        for u in 1..=n {
            for &v in &adj[u] {
                if sid[u] != sid[v] {
                    ind[sid[v]] += 1;
                }
            }
        }

        writeln!(so, "{}", ind.iter().filter(|&&x| x == 0).count())?;
    }

    Ok(())
}
