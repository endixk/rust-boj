// BOJ 2593 [Elevators]
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

fn xgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (g, x, y) = xgcd(b, a % b);
    (g, y, x - (a / b) * y)
}
fn find(m: i64, n: i64, x: i64, y: i64, min: i64, max: i64) -> Option<i64> {
    let (g, k, _) = xgcd(m, n);
    if (y - x) % g != 0 { return None; }
    let k = k * (y - x) / g;
    let g = m * n / g;
    let mut k = (m * k + x) % g;
    while k < min { k += g; }
    if k > max { return None; }
    Some(k)
}
fn reach(m: i64, x: i64, a: i64) -> bool {
    if a < x { return false; }
    (a - x) % m == 0
}
use std::collections::VecDeque;
fn bfs(adj: &Vec<Vec<usize>>, trk: &mut Vec<usize>, v: &[(i64, i64)], m: usize, s: usize, dst: i64) -> Option<i32> {
    let mut d = 0;
    let mut q = VecDeque::new();
    let mut vis = vec![false; m];
    let mut par = vec![0; m];
    q.push_back(s);
    vis[s] = true;
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let u = q.pop_front().unwrap();
            if reach(v[u].1, v[u].0, dst) {
                trk.clear();
                let mut u = u;
                trk.push(u);
                while u != s {
                    u = par[u];
                    trk.push(u);
                }
                return Some(d);
            }
            for &v in &adj[u] {
                if !vis[v] {
                    vis[v] = true;
                    par[v] = u;
                    q.push_back(v);
                }
            }
        }
        d += 1;
    }
    None
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<i64>(&mut it), next::<usize>(&mut it));
    let v = (0..m).map(|_| (next::<i64>(&mut it), next::<i64>(&mut it))).collect::<Vec<_>>();
    let mut adj = vec![vec![]; m];
    for i in 0..m-1 {
        for j in i+1..m {
            if let Some(_) = find(v[i].1, v[j].1, v[i].0 % v[i].1, v[j].0 % v[j].1, v[i].0.max(v[j].0), n) {
                adj[i].push(j);
                adj[j].push(i);
            }
        }
    }

    let (a, b) = (next::<i64>(&mut it), next::<i64>(&mut it));
    let mut ans: Option<i32> = None;
    let mut trk = vec![]; let mut mi = 0;
    for i in 0..m {
        if reach(v[i].1, v[i].0, a) {
            if let Some(k) = bfs(&adj, &mut trk, &v, m, i, b) {
                if let Some(a) = ans {
                    if k < a {
                        ans = Some(k);
                        mi = i;
                    }
                } else {
                    ans = Some(k);
                    mi = i;
                }
            }
        }
    }
    if let Some(a) = ans {
        writeln!(so, "{}", a+1).unwrap();
        bfs(&adj, &mut trk, &v, m, mi, b);
        for i in trk.iter().rev() {
            writeln!(so, "{}", i + 1).unwrap();
        }
    } else {
        writeln!(so, "-1").unwrap();
    }
}
