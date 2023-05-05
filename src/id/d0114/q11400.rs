// BOJ 11400 [Articulation Bridges]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

fn dfs(u: usize, p: usize, adj: &Vec<Vec<usize>>,
       dfsn: &mut Vec<usize>, cnt: &mut usize, arts: &mut Vec<(usize, usize)>) -> usize {
    *cnt += 1;
    dfsn[u] = *cnt;
    let mut ret = dfsn[u];
    for &v in adj[u].iter() {
        if v == p { continue; }
        if dfsn[v] == 0 {
            let prev = dfs(v, u, adj, dfsn, cnt, arts);
            if prev > dfsn[u] {
                if u < v { arts.push((u, v)); }
                else { arts.push((v, u)); }
            }
            ret = ret.min(prev);
        } else {
            ret = ret.min(dfsn[v]);
        }
    }
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (v, e) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let mut adj = vec![vec![]; v + 1];
    (0..e).for_each(|_| {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push(v);
        adj[v].push(u);
    });

    let mut arts = Vec::<(usize, usize)>::new();
    let mut dfsn = vec![0; v + 1];
    let mut cnt = 0;
    for u in 1..=v {
        if dfsn[u] == 0 {
            dfs(u, 0, &adj, &mut dfsn, &mut cnt, &mut arts);
        }
    }

    arts.sort();
    writeln!(so, "{}", arts.len()).ok();
    arts.iter().for_each(|(u, v)| { writeln!(so, "{} {}", u, v).ok(); });
}
