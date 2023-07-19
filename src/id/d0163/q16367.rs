// BOJ 16367 [TV Show Game]
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
        scc_cur.sort_unstable();
        scc.push(scc_cur);
    }

    ret
}

fn node(x: i32, n: usize) -> usize {
    if x > 0 {
        x as usize
    } else {
        -x as usize + n
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (k, n) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; 2*k+1];
    for _ in 0..n {
        let (l1, c1, l2, c2, l3, c3) = (
            next::<i32>(&mut it), next::<char>(&mut it) == 'B',
            next::<i32>(&mut it), next::<char>(&mut it) == 'B',
            next::<i32>(&mut it), next::<char>(&mut it) == 'B');
        let (l1, l2, l3) = (
            if c1 { l1 } else { -l1 },
            if c2 { l2 } else { -l2 },
            if c3 { l3 } else { -l3 });
        adj[node(-l1, k)].push(node(l2, k));
        adj[node(-l1, k)].push(node(l3, k));
        adj[node(-l2, k)].push(node(l1, k));
        adj[node(-l2, k)].push(node(l3, k));
        adj[node(-l3, k)].push(node(l1, k));
        adj[node(-l3, k)].push(node(l2, k));
    }

    let mut scc = Vec::new();
    let mut ids = vec![0; 2*k+1];
    let mut vis = vec![false; 2*k+1];
    let mut stk = Vec::new();
    let mut cnt = 0;
    for i in 1..2*k+1 {
        if ids[i] == 0 {
            tarjan(&mut scc, &adj, &mut ids, &mut vis, &mut stk, &mut cnt, i);
        }
    }

    // check satisfiability
    for cc in scc.iter() {
        for &v in cc.iter() {
            if v > k { continue; }
            if cc.binary_search(&(v+k)).is_ok() {
                println!("-1");
                return;
            }
        }
    }

    // find solution
    let mut ans = vec![false; k+1];
    let mut vis = vec![false; 2*k+1];
    for cc in scc.iter().rev() {
        if vis[cc[0]] { continue; }
        for &v in cc.iter() {
            if v > k {
                ans[v - k] = true;
                vis[v - k] = true;
            } else {
                ans[v] = false;
                vis[v + k] = true;
            }
            vis[v] = true;
        }
    }

    for i in 1..=k {
        write!(so, "{}", if ans[i] { 'B' } else { 'R' }).unwrap();
    }
}
