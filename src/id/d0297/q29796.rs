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

use std::collections::VecDeque;
const MAX: usize = 202;
const INF: i32 = 0x3f3f3f3f;
fn dinic_bfs(lvl: &mut Vec<i32>,
             adj: &Vec<Vec<usize>>, cap: &Vec<Vec<i32>>, flo: &Vec<Vec<i32>>,
             src: usize, snk: usize) -> bool {
    lvl.iter_mut().for_each(|x| *x = -1);
    lvl[src] = 0;
    let mut q = VecDeque::new();
    q.push_back(src);
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if lvl[v] == -1 && cap[u][v] - flo[u][v] > 0 {
                lvl[v] = lvl[u] + 1;
                q.push_back(v);
            }
        }
    }
    lvl[snk] != -1
}
fn dinic_dfs(flo: &mut Vec<Vec<i32>>, wrk: &mut Vec<usize>,
             adj: &Vec<Vec<usize>>, cap: &Vec<Vec<i32>>, lvl: &Vec<i32>,
             cur: usize, snk: usize, f: i32) -> Option<i32> {
    if cur == snk { return Some(f); }
    while wrk[cur] < adj[cur].len() {
        let nxt = adj[cur][wrk[cur]];
        if lvl[nxt] == lvl[cur] + 1 && cap[cur][nxt] - flo[cur][nxt] > 0 {
            if let Some(df) = dinic_dfs(flo, wrk, adj, cap, lvl, nxt, snk, f.min(cap[cur][nxt] - flo[cur][nxt])) {
                flo[cur][nxt] += df;
                flo[nxt][cur] -= df;
                return Some(df);
            }
        }
        wrk[cur] += 1;
    }
    None
}
fn dinic(flo: &mut Vec<Vec<i32>>, lvl: &mut Vec<i32>, wrk: &mut Vec<usize>,
         adj: &Vec<Vec<usize>>, cap: &Vec<Vec<i32>>, src: usize, snk: usize) -> i32 {
    let mut ret = 0;
    while dinic_bfs(lvl, adj, cap, flo, src, snk) {
        wrk.iter_mut().for_each(|x| *x = 0);
        while let Some(f) = dinic_dfs(flo, wrk, adj, cap, lvl, src, snk, INF) {
            ret += f;
        }
    }
    ret
}

const SRC: usize = 0;
const SNK: usize = 201;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();

    let mut g = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        g[u].push(v);
        g[v].push(u);
    }
    let r = next::<usize>(&mut it);
    let b = (0..r).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();

    let mut adj = vec![vec![]; MAX];
    let mut cap = vec![vec![0; MAX]; MAX];
    for i in 1..=n {
        adj[SRC].push(i);
        adj[i].push(SRC);
        cap[SRC][i] = a[i-1];

        adj[i].push(n+i);
        adj[n+i].push(i);
        cap[i][n+i] = INF;
        for &v in &g[i] {
            adj[i].push(n+v);
            adj[n+v].push(i);
            cap[i][n+v] = INF;
        }
    }

    let (mut s, mut e) = (1, a.iter().sum::<i32>()+1);
    while s < e {
        let mid = (s + e) / 2;
        let mut adj = adj.clone();
        let mut cap = cap.clone();
        for &v in &b {
            adj[n+v].push(SNK);
            adj[SNK].push(n+v);
            cap[n+v][SNK] = mid as i32;
        }
        let mut flo = vec![vec![0; MAX]; MAX];
        let mut lvl = vec![0; MAX];
        let mut wrk = vec![0; MAX];
        let f = dinic(&mut flo, &mut lvl, &mut wrk, &adj, &cap, SRC, SNK);
        if f == mid * r as i32 {
            s = mid + 1;
        } else {
            e = mid;
        }
    }
    writeln!(so, "{}", s - 1).unwrap();
}