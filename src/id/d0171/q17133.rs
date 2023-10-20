// BOJ 17133 [Raccoons]
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

use std::collections::VecDeque;
const SRC: usize = 1000;
const SNK: usize = 1001;
const MAX: usize = 1002;
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let (k, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let w = (0..m).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let p = next::<usize>(&mut it);

    let mut adj = vec![vec![]; MAX];
    let mut cap = vec![vec![0; MAX]; MAX];

    for &x in &a { // source -1-> remainders
        if cap[SRC][x] == 0 {
            adj[SRC].push(x);
            adj[x].push(SRC);
        }
        cap[SRC][x] += 1;
    }
    for i in r..k { // remainders -INF-> remainders
        adj[i].push(i - r);
        adj[i - r].push(i);
        cap[i][i - r] = INF;
        adj[i].push(i - r + k);
        adj[i - r + k].push(i);
        cap[i][i - r + k] = INF;
    }
    for i in 0..k { // remainders -1-> remainders
        for &x in &w {
            let j = (i + x) % k;
            if cap[i][j] == 0 {
                adj[i].push(j);
                adj[j].push(i);
            }
            if cap[i+k][j] == 0 {
                adj[i+k].push(j);
                adj[j].push(i+k);
            }
            cap[i][j] += 1;
            cap[i+k][j] += 1;
        }
    }
    // connect sink
    adj[p].push(SNK);
    adj[SNK].push(p);
    cap[p][SNK] = INF;
    adj[p+k].push(SNK);
    adj[SNK].push(p+k);
    cap[p+k][SNK] = INF;

    // flow
    let mut flo = vec![vec![0; MAX]; MAX];
    let mut lvl = vec![0; MAX];
    let mut wrk = vec![0; MAX];
    writeln!(so, "{}", dinic(&mut flo, &mut lvl, &mut wrk, &adj, &cap, SRC, SNK))?;

    Ok(())
}
