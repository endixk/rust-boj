// BOJ 30690 [Railway Assembly]
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

fn dfs1(d: &mut Vec<i32>, md: &mut Vec<(i32,i32,i32)>, adj: &Vec<Vec<usize>>, cur: usize, par: usize) -> i32 {
    for &nxt in adj[cur].iter() {
        if nxt == par { continue; }
        let x = dfs1(d, md, adj, nxt, cur);
        if x > md[cur].0 { md[cur].2 = md[cur].1; md[cur].1 = md[cur].0; md[cur].0 = x; }
        else if x > md[cur].1 { md[cur].2 = md[cur].1; md[cur].1 = x; }
        else if x > md[cur].2 { md[cur].2 = x; }
    }
    d[cur] = md[cur].0 + 1;
    d[cur]
}
fn dfs2(a: &mut Vec<i32>, ma: &mut Vec<(i32,i32)>, md: &Vec<(i32,i32,i32)>, adj: &Vec<Vec<usize>>, cur: usize, par: usize) -> i32 {
    for &nxt in adj[cur].iter() {
        if nxt == par { continue; }
        let x = dfs2(a, ma, md, adj, nxt, cur);
        if x > ma[cur].0 { ma[cur].1 = ma[cur].0; ma[cur].0 = x; }
        else if x > ma[cur].1 { ma[cur].1 = x; }
    }
    a[cur] = ma[cur].0.max(md[cur].0 + md[cur].1 + 2);
    a[cur]
}

use std::collections::HashMap;
fn go(e: &mut Vec<i32>, f: &mut Vec<i32>, g: &mut Vec<i32>,
      d: &Vec<i32>, md: &Vec<(i32,i32,i32)>,
      a: &Vec<i32>, ma: &Vec<(i32,i32)>,
      map: &HashMap<(usize,usize),usize>,
      par: usize, cur: usize, nxt: usize) {
    let cid = map[&(cur,nxt)];
    let pid = map[&(par,cur)];

    g[cid] = if d[nxt] == md[cur].0 { md[cur].1 + 1 } else { md[cur].0 + 1 };
    g[cid] = g[cid].max(g[pid] + 1);

    let (mut x, mut y) = (md[cur].0, md[cur].1);
    if d[nxt] == x { x = y; y = md[cur].2; }
    else if d[nxt] == y { y = md[cur].2; }
    if x < g[pid] { y = x; x = g[pid]; }
    else if y < g[pid] { y = g[pid]; }
    f[cid] = x + y + 2;

    e[cid] = if a[nxt] == ma[cur].0 { ma[cur].1 } else { ma[cur].0 };
    e[cid] = e[cid].max(e[pid]).max(f[cid]);
}
fn dfs3(e: &mut Vec<i32>, f: &mut Vec<i32>, g: &mut Vec<i32>,
        d: &Vec<i32>, md: &Vec<(i32,i32,i32)>,
        a: &Vec<i32>, ma: &Vec<(i32,i32)>,
        adj: &Vec<Vec<usize>>, map: &HashMap<(usize,usize),usize>,
        par: usize, cur: usize) {
    for &nxt in adj[cur].iter() {
        if nxt == par { continue; }
        go(e, f, g, d, md, a, ma, map, par, cur, nxt);
        dfs3(e, f, g, d, md, a, ma, adj, map, cur, nxt);
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; n+1];
    let mut map = HashMap::new();
    map.insert((0,1), 0);
    for i in 1..n {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push(v);
        adj[v].push(u);
        map.insert((u,v), i);
        map.insert((v,u), i);
    }

    let mut d = vec![0; n+1];
    let mut md = vec![(-1,-1,-1); n+1];
    dfs1(&mut d, &mut md, &adj, 1, 0);

    let mut a = vec![0; n+1];
    let mut ma = vec![(0,0); n+1];
    dfs2(&mut a, &mut ma, &md, &adj, 1, 0);

    let mut e = vec![0; n];
    let mut f = vec![0; n];
    let mut g = vec![-1; n];
    dfs3(&mut e, &mut f, &mut g, &d, &md, &a, &ma, &adj, &map, 0, 1);

    for _ in 0..q {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let x = if d[u] < d[v] { u } else { v };
        writeln!(so, "{}", e[map[&(u,v)]] + a[x] + 1)?;
    }

    Ok(())
}
