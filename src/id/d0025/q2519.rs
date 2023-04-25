// BOJ 2519 [Sticks]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

const MAX: usize = 6001;

fn dfs_tarjan(
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
            ret = ret.min(dfs_tarjan(scc, adj, ids, vis, stk, cnt, nxt));
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
        scc_cur.sort();
        scc.push(scc_cur);
    }

    ret
}

struct Line {
    sx: i64,
    sy: i64,
    ex: i64,
    ey: i64,
}

fn intersect(l1: &Line, l2: &Line) -> bool {
    let a = (l1.sx - l1.ex) * (l2.sy - l1.ey) - (l1.sy - l1.ey) * (l2.sx - l1.ex);
    let b = (l1.sx - l1.ex) * (l2.ey - l1.ey) - (l1.sy - l1.ey) * (l2.ex - l1.ex);
    let c = (l2.sx - l2.ex) * (l1.sy - l2.ey) - (l2.sy - l2.ey) * (l1.sx - l2.ex);
    let d = (l2.sx - l2.ex) * (l1.ey - l2.ey) - (l2.sy - l2.ey) * (l1.ex - l2.ex);
    a * b <= 0 && c * d <= 0
}

// generates a 2-SAT graph from line segments
fn generate_graph(lines: &Vec<Line>, n: usize) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); MAX];

    // line intersections
    for i in 1..n {
        for j in i+1..=n {
            if intersect(&lines[i], &lines[j]) {
                // i -> ~j, j -> ~i
                adj[i].push(n+j);
                adj[j].push(n+i);
            }
        }
    }

    // from same individual
    for i in 1..=n/3 {
        let x = i * 3 - 2;
        let y = i * 3 - 1;
        let z = i * 3;

        // ~x -> y, ~x -> z, ~y -> x, ~y -> z, ~z -> x, ~z -> y
        adj[n+x].push(y);
        adj[n+x].push(z);
        adj[n+y].push(x);
        adj[n+y].push(z);
        adj[n+z].push(x);
        adj[n+z].push(y);
    }

    adj
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n: usize = it.by_ref().next().unwrap().parse().unwrap();
    let n = n * 3;

    // get lines
    let mut lines = vec![Line { sx: 0, sy: 0, ex: 0, ey: 0 }];
    for _ in 0..n {
        let sx: i64 = it.by_ref().next().unwrap().parse().unwrap();
        let sy: i64 = it.by_ref().next().unwrap().parse().unwrap();
        let ex: i64 = it.by_ref().next().unwrap().parse().unwrap();
        let ey: i64 = it.by_ref().next().unwrap().parse().unwrap();
        lines.push(Line { sx, sy, ex, ey });
    }

    // generate graph
    let adj = generate_graph(&lines, n);

    // find SCC
    let mut scc = Vec::new();
    let mut ids = vec![0; MAX];
    let mut vis = vec![false; MAX];
    let mut stk = Vec::new();
    let mut cnt = 0;
    for i in 1..=2*n {
        if ids[i] == 0 {
            dfs_tarjan(&mut scc, &adj, &mut ids, &mut vis, &mut stk, &mut cnt, i);
        }
    }

    // check satisfiability
    for cc in scc.iter() {
        for &v in cc.iter() {
            if v > n { continue; }
            if cc.binary_search(&(v + n)).is_ok() {
                writeln!(so, "-1").unwrap();
                return;
            }
        }
    }

    // assign booleans
    let mut ans = Vec::new();
    let mut vis = vec![false; MAX];
    for cc in scc.iter().rev() {
        if vis[cc[0]] { continue; }
        for &v in cc.iter() {
            if v > n {
                vis[v - n] = true;
            } else {
                ans.push(v);
                vis[v + n] = true;
            }
            vis[v] = true;
        }
    }

    writeln!(so, "{}", ans.len()).unwrap();
    for v in ans.iter() {
        write!(so, "{} ", v).unwrap();
    }
}
