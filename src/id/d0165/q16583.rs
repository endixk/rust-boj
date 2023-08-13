// BOJ 16583 [Boomerangs]
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

fn dfs1(adj: &Vec<Vec<usize>>, cur: usize, par: usize,
       tdj: &mut Vec<Vec<usize>>, dmy: &mut Vec<Vec<usize>>,
       vis: &mut Vec<usize>, idx: &mut usize, rep: &mut Vec<usize>, rdx: &mut usize) {
    vis[cur] = *idx;
    *idx += 1;
    for &nxt in &adj[cur] {
        if nxt == par { continue; }
        if vis[nxt] == 0 {
            tdj[cur].push(nxt);
            tdj[nxt].push(cur);
            dfs1(adj, nxt, cur, tdj, dmy, vis, idx, rep, rdx);
        } else if vis[nxt] < vis[cur] {
            rep.push(nxt);
            dmy[cur].push(*rdx);
            *rdx += 1;
        }
    }
}
fn dfs2(tdj: &Vec<Vec<usize>>, rep: &Vec<usize>, cur: usize, par: usize,
        vis: &mut Vec<bool>, col: &mut Vec<bool>, ans: &mut Vec<(usize, usize, usize)>) {
    vis[cur] = true;
    let mut ch = Vec::new();
    for &nxt in &tdj[cur] {
        if nxt == par { continue; }
        dfs2(tdj, rep, nxt, cur, vis, col, ans);
        if !col[nxt] { ch.push(nxt); }
    }
    for i in (1..ch.len()).step_by(2) {
        ans.push((rep[ch[i-1]], rep[cur], rep[ch[i]]));
    }
    if ch.len() % 2 == 1 && par > 0 {
        col[cur] = true;
        ans.push((rep[ch[ch.len()-1]], rep[cur], rep[par]));
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut tdj = vec![vec![]; n+1];
    let mut dmy = vec![vec![]; n+1];
    let mut vis = vec![0; n+1];
    let mut rep = (0..=n).collect::<Vec<_>>();
    let mut idx = 1;
    let mut rdx = n+1;
    for i in 1..=n {
        if vis[i] == 0 {
            dfs1(&adj, i, 0, &mut tdj, &mut dmy, &mut vis, &mut idx, &mut rep, &mut rdx);
        }
    }

    for _ in n+1..rdx { tdj.push(vec![]); }
    for i in 1..=n {
        for &j in &dmy[i] {
            tdj[i].push(j);
            tdj[j].push(i);
        }
    }

    let mut vis = vec![false; rdx];
    let mut col = vec![false; rdx];
    let mut ans = Vec::new();
    for i in 1..rdx {
        if !vis[i] {
            dfs2(&tdj, &rep, i, 0, &mut vis, &mut col, &mut ans);
        }
    }

    writeln!(so, "{}", ans.len()).unwrap();
    for (u, v, w) in ans {
        writeln!(so, "{} {} {}", u, v, w).unwrap();
    }
}
