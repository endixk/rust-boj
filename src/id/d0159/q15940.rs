// BOJ 15940 [Network Hacking]
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

fn dfs1(c: &mut Vec<i64>, z: i64, d: &mut Vec<i64>, md: &mut Vec<(i64,i64,i64)>, adj: &Vec<Vec<(usize,i64)>>, cur: usize, par: usize, dst: i64) -> i64 {
    c[cur] = z;
    for &(nxt, nds) in adj[cur].iter() {
        if nxt == par { continue; }
        let x = dfs1(c, z+1, d, md, adj, nxt, cur, nds) + dst;
        if x > md[cur].0 { md[cur].2 = md[cur].1; md[cur].1 = md[cur].0; md[cur].0 = x; }
        else if x > md[cur].1 { md[cur].2 = md[cur].1; md[cur].1 = x; }
        else if x > md[cur].2 { md[cur].2 = x; }
    }
    if md[cur].0 == 0 { md[cur].0 = dst; }
    d[cur] = md[cur].0;
    d[cur]
}
fn dfs2(a: &mut Vec<i64>, ma: &mut Vec<(i64,i64)>, md: &Vec<(i64,i64,i64)>, adj: &Vec<Vec<(usize,i64)>>, cur: usize, par: usize, dst: i64) -> i64 {
    for &(nxt, nds) in adj[cur].iter() {
        if nxt == par { continue; }
        let x = dfs2(a, ma, md, adj, nxt, cur, nds);
        if x > ma[cur].0 { ma[cur].1 = ma[cur].0; ma[cur].0 = x; }
        else if x > ma[cur].1 { ma[cur].1 = x; }
    }
    let x = if md[cur].0 > dst { md[cur].0 - dst } else { 0 };
    let y = if md[cur].1 > dst { md[cur].1 - dst } else { 0 };
    a[cur] = ma[cur].0.max(x + y);
    a[cur]
}

use std::collections::HashMap;
fn go(e: &mut Vec<i64>, f: &mut Vec<i64>, g: &mut Vec<i64>,
      d: &Vec<i64>, md: &Vec<(i64,i64,i64)>,
      a: &Vec<i64>, ma: &Vec<(i64,i64)>,
      map: &HashMap<(usize,usize),usize>,
      par: usize, cur: usize, nxt: usize, dst: i64) {
    let cid = map[&(cur,nxt)];
    let pid = map[&(par,cur)];

    g[cid] = if d[nxt] == md[cur].0 - dst { md[cur].1 - dst } else { md[cur].0 - dst };
    g[cid] = g[cid].max(g[pid] + dst);

    let (mut x, mut y) = (md[cur].0 - dst, md[cur].1 - dst);
    if d[nxt] == x { x = y; y = md[cur].2 - dst; }
    else if d[nxt] == y { y = md[cur].2 - dst; }
    if x < g[pid] + dst { y = x; x = g[pid] + dst; }
    else if y < g[pid] + dst { y = g[pid] + dst; }
    if x < 0 { x = 0; }
    if y < 0 { y = 0; }
    f[cid] = x + y;

    e[cid] = if a[nxt] == ma[cur].0 { ma[cur].1 } else { ma[cur].0 };
    e[cid] = e[cid].max(e[pid]).max(f[cid]);
}
fn dfs3(e: &mut Vec<i64>, f: &mut Vec<i64>, g: &mut Vec<i64>,
        d: &Vec<i64>, md: &Vec<(i64,i64,i64)>,
        a: &Vec<i64>, ma: &Vec<(i64,i64)>,
        adj: &Vec<Vec<(usize,i64)>>, map: &HashMap<(usize,usize),usize>,
        par: usize, cur: usize, dst: i64) {
    for &(nxt, nds) in adj[cur].iter() {
        if nxt == par { continue; }
        go(e, f, g, d, md, a, ma, map, par, cur, nxt, dst);
        dfs3(e, f, g, d, md, a, ma, adj, map, cur, nxt, nds);
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut adj = vec![vec![]; n+1];
    let mut map = HashMap::new();
    map.insert((0,1), 0);
    for i in 1..n {
        let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i64>(&mut it));
        adj[u].push((v, w));
        adj[v].push((u, w));
        map.insert((u,v), i);
        map.insert((v,u), i);
    }

    let mut c = vec![0; n+1];
    let mut d = vec![0; n+1];
    let mut md = vec![(0,0,0); n+1];
    dfs1(&mut c, 1, &mut d, &mut md, &adj, 1, 0, 0);

    let mut a = vec![0; n+1];
    let mut ma = vec![(0,0); n+1];
    dfs2(&mut a, &mut ma, &md, &adj, 1, 0, 0);

    let mut e = vec![0; n];
    let mut f = vec![0; n];
    let mut g = vec![0; n];
    dfs3(&mut e, &mut f, &mut g, &d, &md, &a, &ma, &adj, &map, 0, 1, 0);

    let mut ans = 0;
    for u in 1..=n {
        for &(v, dst) in adj[u].iter() {
            if c[u] > c[v] { continue; }
           ans = ans.max(e[map[&(u,v)]] + a[v] + dst);
        }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
